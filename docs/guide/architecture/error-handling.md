# Error handling

Errors serve different callers at different boundaries. Preserve the detail a layer needs to make a decision, and remove implementation detail before it crosses inward or becomes public.

| Layer | Owns |
|---|---|
| Domain | invalid Domain state and rejected Domain operations |
| Application | stable use-case outcomes and Port failure categories |
| Infrastructure | classification of concrete SQLx, reqwest, and protocol failures |
| HTTP | status, public code/message, and the public error envelope |
| App | configuration, startup, listener, migration, and shutdown failures that terminate the process |

A concrete dependency failure is classified by the adapter that understands it, then translated into a stable inward category. HTTP maps only stable Domain/Application outcomes to public responses.

## Typed errors and host-boundary context

Use typed errors whenever a caller must distinguish outcomes. Domain, Application, HTTP, and Infrastructure keep typed errors and stable categories at their public architectural seams.

`thiserror` is appropriate when deriving `Display`, `Error`, or local conversions improves those types. Small enums may remain handwritten; do not refactor only to exercise a dependency.

`anyhow` is limited to the executable host boundary, where callers no longer make architectural decisions from variants. The current app may use its existing boxed process error or opt into `anyhow::Result` there without changing inner contracts.

Do not use `anyhow::Error` as a Domain catch-all, an Application Port error, an HTTP handler error, or an Infrastructure boundary type. Opaque errors at those seams erase decisions the next owner must make and encourage leaking internal strings.

## Convert deliberately

Use `From` / `#[from]` only when one source category always maps to exactly one destination category. Use an explicit `match` when status, protocol state, retryability, corruption, conflict, or another property determines the destination.

Let failures propagate with `?` after the owning boundary has classified them. Do not catch failures to return fallback configuration, partial success, or empty values unless that fallback is itself part of the requested behavior.

Request-path production code does not use `unwrap` or `expect` for fallible input, I/O, locks, or external responses. Tests may use them when a panic clearly identifies a failed precondition.

## Public error contract

`ApiError` is the only `IntoResponse` authority. Public status, code, and message are fixed values selected by explicit mapping; they never come from an internal error's `Display`, `Debug`, source chain, SQLx/reqwest text, or downstream response.

The installed Router uses the same authority for unmatched paths and unsupported methods. Do not expose SQL, schema names, full URLs, file paths, dependency names, configuration values, credentials, request bodies, headers, Task titles, raw validation values, or stack traces.

## Operational recording

Outer adapters may record only allowlisted operational categories needed to run the service. Do not log or trace raw dependency errors, SQL text, query strings, payloads, headers, rejected business values, database URLs, secrets, or arbitrary downstream messages.

A source chain is useful only when it is safe at the process boundary. Adding generic context does not make sensitive source data safe.

## Verification

Test errors where their meaning is owned:

- Domain tests distinguish invalid construction and rejected behavior;
- Application tests prove stable outcomes and short-circuiting;
- Infrastructure tests prove concrete failures are classified before crossing inward;
- HTTP tests assert exact status/code/message and absence of internal detail through the installed Router;
- process-path verification requires startup, migration, listener, and shutdown failures to terminate nonzero rather than silently fall back.

When adding a new failure, start with the narrowest layer that can name it accurately, then add only the outward mappings required by real callers.
