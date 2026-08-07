# HTTP middleware

Middleware owns request or response policy that crosses several HTTP operations. It is still an adapter concern: it may inspect HTTP metadata, short-circuit a request, add request-local context, or annotate a response, but it must not become a second Application layer.

The generated baseline installs only the middleware required by its current contract. The extension examples below are intentionally incomplete and are **not installed dependencies**. Before using one, check the candidate crate's current API, feature flags, and compatibility with the versions selected by the generated project.

## Installed stack

[`routes::router`](../../../crates/http/src/routes/mod.rs) registers routes and fallbacks before chaining these layers:

```text
request
  → FastraceLayer
  → mark_server_error
  → DefaultBodyLimit(8 KiB)
  → extractor / handler

response
  ← FastraceLayer
  ← mark_server_error
  ← DefaultBodyLimit(8 KiB)
  ← extractor / handler
```

Repeated `Router::layer` calls wrap the routes already present, so the last chained layer receives the request first; the response unwinds in reverse. `ServiceBuilder` uses the opposite visual convention: its listed layers receive requests from top to bottom. Do not reorder either form by intuition—write down the request and response path first. See Axum's [ordering model](https://docs.rs/axum/0.8.9/axum/middleware/index.html#ordering).

All current routes and both Router fallbacks are installed before the layers, so all of them receive this stack. A route added after a `layer` call would not inherit that layer; `Router::layer` applies only to routes and fallbacks already present. See the [`Router::layer` contract](https://docs.rs/axum/0.8.9/axum/struct.Router.html#method.layer).

### `FastraceLayer`

[`trace_layer`](../../../crates/http/src/middleware/trace.rs) is the outer request layer and the single server-span authority:

- When tracing is enabled, it accepts a valid W3C `traceparent`; a missing or invalid header starts a random root context.
- When `TRACE_EXPORTER=none` makes tracing disabled, or when the path starts with `/health/`, the extractor returns `None` and `fastrace-axum` uses a noop span.
- `fastrace-axum` records the method, URL path, matched route, and response status for an active server span. Its custom extractor controls the parent context, not sampling policy inside a configured reporter.

These behaviors follow the [`FastraceLayer` API](https://docs.rs/fastrace-axum/0.2.0/fastrace_axum/struct.FastraceLayer.html). Keep the exporter decision and reporter lifecycle in `app`; HTTP owns only extraction and request-span policy.

### `mark_server_error`

[`mark_server_error`](../../../crates/http/src/middleware/trace.rs) runs inside the Fastrace span. It adds `span.kind=server`, calls the next service once, and marks only a returned `5xx` response with the stable properties `span.status_code=error` and `error.type=server_error`.

A `4xx` is a handled client outcome, not a server failure. This middleware annotates observability only: it does not catch errors, change status codes, or construct the public error envelope.

### `DefaultBodyLimit`

The innermost layer sets the request-body limit used by body-buffering extractors. Any JSON handler under this Router inherits that limit when it uses Axum's bounded `Json` extractor; adding another such route does not change the middleware contract.

`DefaultBodyLimit` is not a transport-wide byte counter. An extractor that reads `Body` directly can bypass it; a future streaming or upload endpoint therefore needs an explicit route-specific limit. Axum documents both scopes and points to `RequestBodyLimitLayer` when every request body must be limited: [`DefaultBodyLimit`](https://docs.rs/axum/0.8.9/axum/extract/struct.DefaultBodyLimit.html).

## Choose the smallest middleware shape

Use [`from_fn`](https://docs.rs/axum/0.8.9/axum/middleware/fn.from_fn.html) for application-local async request/response logic. Its final arguments are the body-consuming request extractor and `Next`; use `from_fn_with_state` only when middleware needs a configured capability. Implement Tower `Layer` and `Service` only for a reusable, configurable component or when a third-party Tower component already solves the problem.

Use `layer` for policy that must also see the Router's 404 and 405 fallbacks, such as the installed server span and error annotation. Use [`route_layer`](https://docs.rs/axum/0.8.9/axum/struct.Router.html#method.route_layer) for policy that may return early only after a route matched, such as authentication. Otherwise an unknown path can incorrectly become `401` instead of the fixed `404` response. Both methods affect only routes registered before the call.

Axum expects the service exposed to Hyper to be infallible. A Tower middleware error that escapes can terminate the connection without an HTTP response. Prefer middleware that returns `ApiError` as a response; otherwise put [`HandleErrorLayer`](https://docs.rs/axum/0.8.9/axum/error_handling/struct.HandleErrorLayer.html) outside the fallible layer and translate its error into the fixed public envelope. A bare status or empty-body timeout response is not sufficient for this project.

## Conditional extension examples

The following snippets are design sketches, not copy-ready code. Names such as `config`, `principal`, and new `ApiError` variants are placeholders. Install and configure a candidate only after a current requirement exists, then test it through the public Router.

### Alternative HTTP tracing

[`tower_http::trace::TraceLayer`](https://docs.rs/tower-http/latest/tower_http/trace/index.html) provides `tracing`-based request, response, body, end-of-stream, and failure callbacks:

```rust,ignore
// Illustration only: tower-http's `trace` feature and a tracing subscriber are not installed.
router.layer(TraceLayer::new_for_http())
```

Treat it as an alternative server-span/logging authority, not an automatic companion to `FastraceLayer`. Installing both without a deliberate bridge duplicates request telemetry. Never enable header or body capture in this template.

### Response compression

[`CompressionLayer`](https://docs.rs/tower-http/latest/tower_http/compression/index.html) selects response compression from `Accept-Encoding` when a compression feature is enabled:

```rust,ignore
// Illustration only: choose and enable only the encodings the deployment supports.
router.layer(CompressionLayer::new())
```

Measure before enabling it. Exclude latency-sensitive streams such as SSE until flush behavior has been tested, and use a [compression predicate](https://docs.rs/tower-http/latest/tower_http/compression/predicate/index.html) for responses that must not be compressed.

### Request IDs

Request IDs provide local lookup and support correlation with systems that do not propagate trace context; they do not replace the distributed trace ID. `tower-http` can set a missing request ID and propagate it to the response. When combined with its `TraceLayer`, its documented order is set, trace, then propagate: [`request_id`](https://docs.rs/tower-http/latest/tower_http/request_id/index.html).

```rust,ignore
// Illustration only: `make_request_id` and the header policy are project decisions.
ServiceBuilder::new()
    .layer(SetRequestIdLayer::new(x_request_id.clone(), make_request_id))
    .layer(server_trace_layer)
    .layer(PropagateRequestIdLayer::new(x_request_id))
```

Define whether a trusted ingress ID is preserved or replaced. Log only the validated ID, never surrounding headers.

### Deadlines and timeouts

The smallest fixed-envelope implementation is local middleware around `Next`:

```rust,ignore
// Illustration only: add and map `ApiError::RequestTimeout` before using this.
async fn deadline(request: Request, next: Next) -> Result<Response, ApiError> {
    tokio::time::timeout(Duration::from_secs(30), next.run(request))
        .await
        .map_err(|_| ApiError::RequestTimeout)
}
```

[`tower_http::timeout::TimeoutLayer`](https://docs.rs/tower-http/latest/tower_http/timeout/index.html) instead returns a configured status with an empty body, while `tower::timeout::TimeoutLayer` returns an error. The former needs response-envelope conversion here; the latter needs `HandleErrorLayer` outside it. Distinguish a handler deadline from request-body idle and total-transfer deadlines, which tower-http models separately.

A handler timeout cancels by dropping the in-flight future; it does not prove that a database or downstream write did not commit. Define operation idempotency and retry semantics before exposing a timeout response for a mutating operation. See Tokio's [`timeout` cancellation contract](https://docs.rs/tokio/1.53.1/tokio/time/fn.timeout.html#cancellation).

### Explicit CORS

CORS is a browser-facing deployment contract, so derive it from validated runtime configuration and enumerate the allowed origin, methods, and headers. [`CorsLayer`](https://docs.rs/tower-http/latest/tower_http/cors/index.html) exposes each part explicitly:

```rust,ignore
// Illustration only: `origin` is a validated HeaderValue from app configuration.
let cors = CorsLayer::new()
    .allow_origin(origin)
    .allow_methods([Method::GET, Method::POST])
    .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
    .allow_credentials(true);
```

Do not use a permissive production default. If CSRF tokens travel in headers, the CORS allow/expose lists must match that protocol exactly.

### Authentication and request identity

Authentication parses HTTP credentials, invokes a configured verifier, and inserts a validated principal into request extensions. Apply it with `route_layer` so unmatched paths retain 404 semantics; handlers can then extract `Extension<Principal>`. Axum documents request extensions as request-scoped data and `from_fn_with_state` as the state-aware middleware form: [`from_fn_with_state`](https://docs.rs/axum/0.8.9/axum/middleware/fn.from_fn_with_state.html), [`Extension`](https://docs.rs/axum/0.8.9/axum/struct.Extension.html).

```rust,ignore
// Illustration only: verifier and Principal are project-owned types.
async fn authenticate(
    State(verifier): State<Verifier>,
    mut request: Request,
    next: Next,
) -> Result<Response, ApiError> {
    let credential = credential_from(&request)?;
    let principal = verifier.verify(credential).await?;
    request.extensions_mut().insert(principal);
    Ok(next.run(request).await)
}

let protected = Router::new()
    .route("/account", get(account))
    .route_layer(from_fn_with_state(verifier, authenticate));
```

HTTP owns credential extraction and `401` translation. Application or Domain owns business authorization decisions. If verification calls an external identity provider, Application owns the Port, Infrastructure implements it, and `app` wires it.

### Rate limiting

A local rate limiter is an HTTP admission policy; a shared quota across replicas needs an external store and an explicit consistency decision. [`tower-governor`](https://docs.rs/tower_governor/latest/tower_governor/) is one Tower-compatible candidate with peer-IP, forwarded-IP, global, and custom key extractors:

```rust,ignore
// Illustration only: key source, quota, cleanup, and error mapping are deployment decisions.
protected.route_layer(GovernorLayer::new(governor_config))
```

Do not trust forwarded IP headers unless the proxy boundary overwrites and validates them. Verify that rejection becomes `429` with this project's error envelope and decide whether the quota key is an IP, principal, tenant, or operation before installation.

### Sessions, cookies, and CSRF

These are related but separate contracts:

| Need | Candidate entry point | Boundary rule |
|---|---|---|
| Server-side session data | [`tower_sessions::SessionManagerLayer`](https://docs.rs/tower-sessions/latest/tower_sessions/service/struct.SessionManagerLayer.html) | `app` selects and constructs the store; HTTP extracts session context. Use an in-memory store only for tests or a deliberately single-process ephemeral service. |
| Direct cookie reads and writes | [`axum_extra::extract::cookie::CookieJar`](https://docs.rs/axum-extra/latest/axum_extra/extract/cookie/struct.CookieJar.html) or [`tower_cookies::CookieManagerLayer`](https://docs.rs/tower-cookies/latest/tower_cookies/) | Prefer the Axum extractor when handlers alone own cookies; choose the Tower layer only when middleware or another Tower service also needs them. A changed `CookieJar` must be returned with the response. Set `Secure`, `HttpOnly`, `SameSite`, path, expiry, and scope from the real browser contract. |
| Origin/fetch-metadata CSRF rejection | [`tower_http::csrf::CsrfLayer`](https://docs.rs/tower-http/latest/tower_http/csrf/index.html) | Uses `Origin`, `Host`, and `Sec-Fetch-Site` without per-request token state. Configure trusted origins and proxies, and replace its default `403` with the fixed envelope. |
| Cookie-authenticated browser mutations | [`axum_csrf::CsrfLayer` and `CsrfToken`](https://docs.rs/axum_csrf/latest/axum_csrf/) | Generate and verify a token on unsafe operations; map failure to the fixed envelope. Treat this as a candidate, not a compatibility promise. |

CSRF protection is required by the authentication mechanism and browser behavior, not by the presence of HTML forms alone. For ambient cookie credentials, use a synchronizer-token or other documented defense; `SameSite` is defense in depth, not a replacement for token validation. Bearer credentials that browsers do not attach automatically have a different threat model. See the [OWASP CSRF Prevention Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Cross-Site_Request_Forgery_Prevention_Cheat_Sheet.html).

Do not choose these candidates independently and assume they interoperate. Before installation, verify their current Axum, `cookie`, and session-store dependencies together, then test login, rotation, logout, expiry, replay rejection, cross-origin preflight, and every public error response.

Do not copy the old Book's `axum-csrf-sync-pattern` stack. Its own [current crate documentation](https://docs.rs/axum_csrf_sync_pattern/latest/axum_csrf_sync_pattern/) still requires `axum-sessions`, whose documentation says development moved to `tower-sessions` because of bugs and a design flaw: [`axum-sessions` migration notice](https://docs.rs/axum-sessions/latest/axum_sessions/).

## Ownership and redaction

- `http` owns middleware behavior, HTTP metadata parsing, request extensions, status codes, and conversion to `ApiError`.
- `app` owns configuration, concrete middleware construction when secrets or stores are involved, and startup/shutdown lifecycle.
- `application` owns use cases, identity or quota Ports when they represent an external capability, and stable failure categories.
- `infrastructure` implements external identity, distributed quota, or session-store access and logs concrete failures safely.
- `domain` owns business authorization invariants and never sees Axum, headers, cookies, spans, or middleware types.

Never log or trace secrets, database URLs, Task Titles, bodies, headers, query strings, SQL text, session values, bearer tokens, cookies, CSRF tokens, or raw downstream errors. Record only bounded metadata with an explicit operational purpose, such as matched route, method, status category, validated request ID, latency, and stable error category.

## Verification

Drive middleware through the public installed Router. For any new layer, cover the exact path it can break: ordering, early return, 404/405 preservation, error envelope, body limit, response header, timeout, or request extension. Configuration-, lifecycle-, or installed-route changes require `just verify` after `just check`.
