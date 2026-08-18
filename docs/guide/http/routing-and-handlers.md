# Routing and handlers

The Router is the generated service's installed HTTP interface. It owns paths, methods, request limits, middleware, fallbacks, and state installation; handlers translate across that interface without becoming another business layer.

## Installed routes

This table is the Guide's only installed-route catalogue. Code and public Router tests remain executable truth; other chapters link to this table or use an explicitly named reference flow instead of maintaining another route list.

| Method | Path | Responsibility |
|---|---|---|
| `POST` | `/api/v1/tasks` | Convert the create request, call `CreateTask`, and return the created Task DTO |
| `GET` | `/api/v1/tasks/{task_id}` | Parse the typed Task ID, call `GetTask`, and return the Task DTO or fixed not-found error |
| `GET` | `/health/live` | Report that the HTTP runtime can respond, without external I/O |
| `GET` | `/health/ready` | Call `ReadinessProbe` and translate its stable result |

`/api/v1` is the compatibility namespace for business HTTP contracts. Health endpoints remain unversioned because they are process-operational contracts rather than versions of the Task resource. Add `/api/v2` only for a real incompatible public contract; do not copy the version into Application or Domain module names.

[`routes::router`](../../../crates/http/src/routes/mod.rs) is the only public builder. It nests the small business Router under `/api/v1`, installs health routes at the root, and then applies the shared HTTP limits, [middleware](middleware.md), state, and fallbacks. A version prefix does not create a new architecture layer.

## Request path

```text
method + path
    → installed Router
    → ordered extractors
    → adapter-owned boundary conversion
    → narrowest existing inward owner
    → Application result conversion or ApiError
    → status + JSON
```

The canonical [Task golden path](../task-flow.md) follows both create and lookup across every crate; it is a reference slice, not an exhaustive route catalogue. Route registration selects an HTTP operation; it does not construct dependencies or contain business decisions.

## Handler responsibility

The Task handlers perform five operations:

1. Extract focused Application capabilities and HTTP request/path DTOs.
2. Convert untrusted transport representations through `TryFrom` or `FromStr`.
3. Start any adapter-owned child span without recording request data.
4. Call one narrow Application use case.
5. Convert its approved result into `TaskResponse` or a failure into `ApiError`.

Health handlers follow the same translation rule by invoking the Application-owned readiness Port. A handler may call a Domain constructor directly only when the public operation is exactly that pure invariant construction and needs no Application decision or capability. Otherwise, Application owns orchestration.

Keep SQLx, reqwest, configuration, concrete adapters, Domain rule implementations, retry decisions, and dependency construction outside handlers. Do not add a Controller or a one-line Application wrapper merely to preserve a cosmetic layer hop.

## Extractor ordering

Axum evaluates handler arguments from left to right. Extractors based on request parts, including focused `State` and `Path`, belong before the one extractor that consumes the body. The CreateTask handler therefore accepts Axum `Json<CreateTaskRequest>` as its final extractor and maps `JsonRejection` through [`ApiError`](../../../crates/http/src/errors/mod.rs). [Validation boundaries](validation.md) explains why those transport checks do not replace Domain construction.

The Router's 8 KiB limit applies before JSON deserialization. A new body-consuming extractor must remain last; a handler must not attempt to consume the body twice.

## Responses and fallbacks

HTTP DTOs own serialization, and [`ApiError`](../../../crates/http/src/errors/mod.rs) is the only public error-response authority. Unknown paths and unsupported methods are Router failures, so the outer Router maps both through the same fixed envelope:

```json
{"error":{"code":"not_found","message":"route not found"}}
```

Use both path fallback and method-not-allowed fallback. A path fallback alone cannot translate a request whose path exists but whose method is unsupported.

## Handler diagnostics

Axum's `macros` feature is enabled, so a non-generic handler may temporarily or permanently use `#[axum::debug_handler]` for clearer extractor and future diagnostics. The macro does not support generic functions; the template's statically dispatched generic Task and readiness handlers therefore rely on compilation through the installed Router instead of changing dispatch merely to add the attribute.

## Growth and verification

Keep registration in `routes/` and behavior in the existing responsibility directories. Add a public route to the installed-routes table above; update another chapter only when the route changes that chapter's stable rule. Split a route family only when it contains enough real routes to own a coherent interface; do not pre-create version, CRUD, controller, or responder trees.

Tests drive the public Router with real methods, paths, headers, and bodies. They assert the exact status and public JSON contract, including invalid IDs, not-found resources, 404, and 405. A detached handler test cannot prove nesting, limits, middleware, focused state extraction, or fallbacks.
