# Routing and handlers

The Router is the generated service's installed HTTP interface. It owns paths, methods, request limits, middleware, fallbacks, and state installation; handlers translate across that interface without becoming another business layer.

## Installed routes

Code and public Router tests are executable truth. The generated project has two intentionally different route shapes:

### Default service

| Method | Path | Responsibility |
|---|---|---|
| `GET` | `/health/live` | Report that the HTTP runtime can respond, without external I/O |
| `GET` | `/health/ready` | Call `ReadinessProbe` and translate its stable result |

The default service does not install Task routes.

### `reference-task` feature

The feature retains both health routes and additionally installs:

| Method | Path | Responsibility |
|---|---|---|
| `POST` | `/api/v1/tasks` | Convert the create request, call `CreateTask`, return the created Task DTO |
| `GET` | `/api/v1/tasks/{task_id}` | Parse the typed Task ID, call `GetTask`, return the Task DTO or fixed not-found error |
| `POST` | `/api/v1/tasks/{task_id}/start` | Parse the path/request contract and invoke the concurrency-sensitive `StartTask` reference workflow |

`/api/v1` is the compatibility namespace for reference business HTTP contracts. Health endpoints remain unversioned process contracts. Add another API version only for a deliberate incompatible public contract; a version prefix does not create an Application or Domain layer.

`routes::router` is the only public builder. It installs the selected route shape, health routes, shared HTTP limits, middleware, state, and fixed fallbacks.

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

Route registration selects an HTTP operation; it does not construct dependencies or contain business decisions.

## Handler responsibility

A business handler should do only the work needed at the inbound boundary:

1. extract the focused Application capability and HTTP DTO/path values;
2. convert untrusted transport representations through `TryFrom` / `FromStr` / Domain constructors;
3. add adapter-owned diagnostics without recording request data;
4. invoke the narrow Application use case;
5. convert the approved result or failure into the public HTTP contract.

Health handlers follow the same translation rule by invoking the Application-owned readiness capability. Keep SQLx, reqwest, configuration, concrete adapters, Domain-rule implementations, retry decisions, and dependency construction outside handlers. Do not add a Controller or one-line Application wrapper merely to preserve a cosmetic layer hop.

## Extractor ordering and responses

Axum evaluates handler arguments from left to right. Request-part extractors belong before the one extractor that consumes the body. A body-consuming handler must not consume the body twice and must confirm which body limit applies.

HTTP DTOs own serialization, and `ApiError` is the only public error-response authority. Unknown paths and unsupported methods are Router failures and must use the same fixed envelope as handler failures. Keep both path and method-not-allowed fallbacks; a path fallback alone cannot translate a request whose path exists but whose method is unsupported.

## Growth and verification

Keep registration in `routes/` and behavior in the existing responsibility directories. Update this route catalogue when an installed route changes, rather than copying route lists into several chapters. Split a route family only when it contains enough real operations to own a coherent interface.

Tests drive the public Router with real methods, paths, headers, and bodies. They assert exact status and public JSON behavior, including fallback behavior. Finish with `just verify` rather than a separate `just check` when the installed route graph or production composition changes, because `just verify` already includes the check gate.
