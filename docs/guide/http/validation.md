# Validation boundaries

Validation is not one operation owned by one layer. The generated service separates transport checks, Domain construction, Application decisions, and persistence integrity so each rule has one authority.

| Boundary | Question | Owner | Reference Task example |
|---|---|---|---|
| Transport structure | Can the request be decoded according to the HTTP contract? | HTTP | content type, JSON syntax/types, unknown fields, body size |
| Domain invariant | Can a raw value become a valid Domain value? | Domain, invoked by the inbound adapter | title/description/priority/ID/revision parsing |
| Business decision | Is valid Domain input acceptable for this operation? | Application/Domain through explicit behavior and Ports | TaskPolicy decision or Task state transition |
| Persistence integrity | Can invalid/conflicting data be committed? | Infrastructure/database | schema constraints, checked bound SQL, atomic mutation |

Client-side validation may improve feedback, but it is never a trust boundary.

## Reference creation path

```text
HTTP bytes and headers
    → Result<Json<CreateTaskRequest>, JsonRejection>
    → CreateTaskRequest
    → TryFrom<CreateTaskRequest> for CreateTaskCommand
       → Domain value constructors/parsers
    → CreateTask::execute(CreateTaskCommand)
    → TaskPolicy::evaluate(command.policy_input())
    → Task::create(NewTask)
    → TaskRepository::insert(&Task)
```

Application receives validated Domain/Application values rather than raw transport strings. The policy call is a business capability, not structural validation: it may depend on external state and can be unavailable even when the input is valid.

## Do not duplicate Domain invariants on DTOs

Do not add validator rules merely to repeat Domain normalization, length, state, unit, or transition rules. Two implementations can diverge. HTTP may reject impossible transport representations early, but Domain construction remains the authority for values that must be valid inside the model.

The same rule applies to other inbound representations: CLI arguments, messages, import rows, and persisted rows reconstruct Domain values through public constructors or explicit reconstitution paths.

Use `axum-valid` only for rules that belong specifically to the transport contract, such as a query page-size range or a cross-field requirement that has no Domain meaning. Do not perform I/O from DTO validators, and do not let a validation library define the public error envelope.

## Public errors

Map transport, Domain, business-policy, conflict, and infrastructure failures to stable allowlisted HTTP outcomes. Never serialize dependency error strings, raw `validator::ValidationErrors`, rejected values, SQL, URLs, credentials, headers, or internal source chains into the response.

Adding field-level error details is a public API change: define the JSON shape and stable field codes in HTTP-owned types and prove it through the installed Router.

## Verification

Put proof at the narrowest owner:

- Domain tests prove invariant construction and transition behavior;
- Application tests prove orchestration and stable outcomes;
- HTTP tests send real requests through the installed Router and assert exact public behavior;
- Infrastructure/database tests prove constraints and query/atomicity contracts.

Do not repeat every Domain boundary case in HTTP or assert validator implementation details from Application tests.
