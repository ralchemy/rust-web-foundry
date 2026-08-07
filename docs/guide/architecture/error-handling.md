# Error handling

Errors serve different callers at different boundaries. Preserve the detail a layer needs to make a decision, and remove implementation detail before it crosses inward or becomes public.

| Layer | Owns |
|---|---|
| Domain | invalid Domain state and rejected Domain operations |
| Application | stable use-case outcomes and Port failure categories |
| Infrastructure | classification of concrete SQLx, reqwest, and protocol failures |
| HTTP | the only status, public code, message, and error-envelope mapping |
| App | configuration, startup, listener, migration, and shutdown failures that terminate the process |

The complete Task mapping is recorded once in [Task flow](../task-flow.md). Its persistence path illustrates the intentional loss of detail:

```text
sqlx::Error
    → TaskRepositoryError
    → CreateTaskError::Persistence
    → ApiError::Internal
    → 500 { "error": { "code": "internal_error", ... } }
```

Infrastructure understands SQLx and therefore classifies and safely records the concrete failure before returning. Application only needs to know that persistence failed. HTTP only needs the stable public contract. Neither Application nor HTTP benefits from retaining the original SQLx value.

## Typed errors and opaque context

Use a typed error when a caller must distinguish outcomes:

- Domain errors identify invariant or operation failures.
- Application errors let use cases and adapters make stable decisions.
- HTTP errors select public responses.

`thiserror` is appropriate for these types when deriving `Display`, `Error`, or local conversions improves the implementation. Small category enums may remain handwritten; do not refactor them merely to exercise a dependency.

Use `anyhow` only where callers do not make programmatic decisions from variants, such as adding context inside an outer adapter or at the executable process boundary. The current `app` uses a boxed process error; a generated project may opt into `anyhow::Result` there without changing inner contracts.

Do not use `anyhow::Error` as a Domain catch-all, an Application Port error, or an HTTP handler error. Opaque errors at those boundaries erase decisions the next layer must make and tempt HTTP code to expose an internal string.

## Convert deliberately

Use `From` or `#[from]` when one source category always maps to exactly one destination category. Use an explicit `match` when status, protocol state, retryability, or another property determines the category. Automatic conversion must not silently cross several architectural boundaries or turn every failure into one catch-all before the owning layer has classified it.

Let errors propagate with `?` after the necessary boundary conversion. Do not catch an internal failure to return a default, partial success, empty collection, or fallback configuration. Only the HTTP response boundary and the executable process boundary change how a failure is presented to their caller.

Request-path production code does not use `unwrap` or `expect` for fallible input, I/O, locks, or external responses. Tests may use them when a panic clearly identifies a failed test precondition.

## Public error contract

HTTP returns one fixed envelope:

```json
{
  "error": {
    "code": "task_title_invalid",
    "message": "task title is invalid"
  }
}
```

[`ApiError`](../../../crates/http/src/errors/mod.rs) is the only `IntoResponse` authority. Public status, code, and message are fixed values selected by an explicit mapping; they never come from an internal error's `Display`, `Debug`, source chain, or downstream response.

The installed Router uses the same authority for unmatched paths and unsupported methods. Do not leave Router-level 404 or 405 responses in Axum's default non-envelope form.

Do not include SQL, schema names, full URLs, file paths, dependency names, configuration values, credentials, request bodies, headers, Task Titles, raw validation input, or stack traces in a public response. Adding field-level validation details or a new envelope shape is an API-contract change rather than an Infrastructure logging decision.

## Operational recording

The outer adapter records a safe operational category before converting a concrete dependency error into a stable Application error. Record only allowlisted properties needed to operate the service, such as the fixed dependency name, operation category, or safe database error code.

Never log or trace raw SQLx/reqwest errors, SQL text, query strings, payloads, headers, titles, database URLs, secrets, or arbitrary downstream messages. An error chain is useful only when it is safe for the selected process boundary; adding `anyhow::Context` does not make sensitive source data safe.

## Verification

Error tests prove behavior at the boundary that owns it:

- Domain tests distinguish invalid constructions without adapter types.
- Application tests prove each stable outcome and that failure short-circuits later calls.
- Adapter tests prove concrete failures are classified before crossing inward.
- HTTP tests drive the installed Router and assert exact status, code, and message while checking that internal details are absent.
- Process-path verification requires startup, migration, listener, and shutdown failures to exit nonzero rather than silently fall back.

When adding a new error path, start with the layer that can name the failure accurately, then add only the outward mappings required by real callers.
