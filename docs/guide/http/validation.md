# Validation boundaries

Validation is not one operation owned by one layer. The generated service separates transport checks, Domain construction, Application decisions, and persistence integrity so that each rule has one authority.

| Boundary | Question | Owner | CreateTask example |
|---|---|---|---|
| Transport structure | Can this request be decoded according to the HTTP contract? | HTTP | content type, JSON syntax and types, unknown fields, and body size |
| Domain invariant | Can this raw value become a valid Domain value? | Domain | HTTP invokes `TaskTitle::parse`, which trims and rejects empty, overlong, or control-character input before calling Application |
| Business decision | Is this valid value acceptable for the current operation? | Application orchestrating a Port | `TaskPolicy::is_allowed` |
| Persistence integrity | Can invalid or conflicting data be committed? | Infrastructure and the database | column types, length, keys, and checked bound SQL |

Client-side validation may improve feedback, but it is never a trust boundary. Every request still passes through the server-owned boundaries below.

## Reference path: CreateTask

```text
HTTP bytes and headers
    → Result<Json<CreateTaskRequest>, JsonRejection>
    → CreateTaskRequest { title: String }
    → TaskTitle::parse
    → CreateTask::execute(TaskTitle)
    → TaskPolicy::is_allowed(&TaskTitle)
    → TaskRepository::insert(&Task)
```

The CreateTask handler uses Axum `Json` directly and converts `JsonRejection` through [`ApiError`](../../../crates/http/src/errors/mod.rs). `CreateTaskRequest` uses `serde(deny_unknown_fields)`, and its bounded extractor inherits the installed Router's 8 KiB limit. Each future request DTO owns its unknown-field contract, and each body consumer must confirm whether the shared limit applies. These checks answer whether the wire representation is acceptable; they do not establish a Domain invariant.

The HTTP handler constructs [`TaskTitle`](../../../crates/domain/src/value_objects/task_title.rs) before invoking [`CreateTask`](../../../crates/application/src/use_cases/create_task.rs). Every inbound adapter must perform the same explicit Domain construction at its own trust boundary, while the use case and its Ports accept only the valid type. An invalid title cannot enter Application, reach the external Policy, or reach MySQL.

`TaskPolicy` is deliberately not called validation. It is an external business capability whose answer may depend on current state and may be unavailable. A structurally valid title can therefore be rejected by Policy without becoming an invalid `TaskTitle`.

## Do not duplicate Domain invariants on DTOs

Do not add `#[derive(Validate)]` to `CreateTaskRequest` merely to repeat the `TaskTitle` length, whitespace, or control-character rules. Two implementations can disagree about normalization, character counting, and future rule changes. HTTP may reject an impossible transport representation early, but `TaskTitle::parse` remains the sole authority for valid Domain state.

The same rule applies to later adapters:

- a CLI argument, message payload, import row, or database row still reconstructs `TaskTitle` through its public constructor;
- an HTTP pre-check is an adapter convenience, not proof that the Domain value is valid;
- a database constraint is a final integrity defense, not a replacement for Domain construction.

## Use `axum-valid` for transport DTOs

The HTTP crate includes [`axum-valid`](https://docs.rs/axum-valid/0.25.0/axum_valid/) with JSON, query, form, and [`validator`](https://docs.rs/validator/0.20.0/validator/) support. Appropriate transport-only rules include:

- a query parameter range such as `page_size=1..=100`;
- a form-only batch-size limit;
- a cross-field confirmation required by one public request contract;
- an email or URL format that belongs only to that wire contract and is not a Domain concept.

If a rule has business meaning, is reused by another inbound adapter, or must be true inside an Entity, move it to a Domain type or owning Domain behavior instead. If a decision needs a repository, downstream service, clock, or other external capability, model it as Application orchestration through a Port; do not perform I/O from a DTO validator.

Use `validator` derives for the DTO rule and `axum_valid::Valid` to run it after Axum extraction:

```rust
use serde::Deserialize;
use validator::Validate;

// Conditional example; this route is not part of the generated baseline.
#[derive(Deserialize, Validate)]
#[serde(deny_unknown_fields)]
struct ListTasksQuery {
    #[validate(range(min = 1, max = 100, code = "page_size_out_of_range"))]
    page_size: u16,
}
```

The same ownership applies to the common Axum inputs:

| Input | Validated extractor |
|---|---|---|
| JSON | `Valid<Json<T>>` |
| query string | `Valid<Query<T>>` |
| form body | `Valid<Form<T>>` |

Do not create local generic `ValidatedJson`, `ValidatedQuery`, or `ValidatedForm` extractors. `axum-valid` already owns extraction followed by validation. HTTP still owns the public boundary: accept, for example, `Result<Valid<Query<T>>, ValidRejection<QueryRejection>>`, convert both `ValidRejection::Inner` and `ValidRejection::Valid` into `ApiError`, and only then enter the handler's normal use-case path. This small rejection mapping preserves the fixed envelope without reimplementing validation.

Do not enable `axum-valid`'s `into_json` or `422` features. Its dependency-shaped default response is not this service's public contract, regardless of status or body format. JSON DTOs without validator rules use `Result<Json<T>, JsonRejection>` and the same `ApiError` conversion. Add a custom extractor only when it hides a distinct current responsibility across multiple callers; renaming Axum `Json<T>` does not qualify.

## Keep the public error contract stable

The baseline maps malformed transport input to `400 invalid_request`, Domain title failure to `422 task_title_invalid`, and Policy rejection to `422 task_policy_rejected`. These codes name different owners and must not collapse into a raw validation string.

Never serialize `validator::ValidationErrors`, its `Display` output, custom messages, parameters, or rejected values directly. They are dependency-shaped data and may reveal input that the service must not log, trace, or return. Map known failures to allowlisted public codes and fixed messages.

The current envelope has no field-error member. Adding one is a public API change: define its exact JSON shape and stable field codes in the HTTP DTO, update `ApiError`, and prove it through the installed Router. A project could deliberately extend the contract like this:

```json
{
  "error": {
    "code": "invalid_request",
    "message": "request is invalid",
    "fields": [
      { "field": "page_size", "code": "page_size_out_of_range" }
    ]
  }
}
```

This is a conditional example, not the generated response. Keep field names and codes allowlisted, and do not let a validation library silently define their shape.

See [Error handling](../architecture/error-handling.md) for cross-layer failure conversion and [Routing and handlers](routing-and-handlers.md) for extractor ordering and installed-Router testing.

## Verify the owning boundary

- Domain tests prove normalization, invariant boundaries, and valid construction beside the owning type.
- Application tests prove typed Domain input reaches Policy before persistence and that every Application failure short-circuits later calls.
- HTTP tests send real JSON, query, or form requests through the installed Router and assert the exact status and public envelope.
- Migration and Infrastructure checks prove database constraints and checked parameter binding without treating persistence as the first validation layer.

Add the smallest test at the boundary that owns the new rule. Do not repeat every Domain boundary case in HTTP tests or assert a validator implementation detail from an Application test.
