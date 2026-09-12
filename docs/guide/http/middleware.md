# HTTP middleware

Middleware owns request/response policy shared across HTTP operations. It remains an adapter concern: it may inspect HTTP metadata, short-circuit a request, add request-local context, or annotate a response, but it must not become a second Application layer.

The generated baseline installs only behavior required by the current contract. Candidate middleware is not an installed capability until a concrete requirement, dependency choice, configuration contract, and Router test exist.

## Installed stack

`routes::router` applies the current shared layers to all installed routes and fallbacks:

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

Repeated `Router::layer` calls wrap routes already present; the last chained layer receives the request first. Do not reorder by visual intuition. Write down the request and response path and test the behavior that depends on ordering.

`FastraceLayer` is the server-span authority. HTTP owns trace-context extraction; reporter/export lifecycle stays in `app`. Health paths use noop tracing under the current policy.

`mark_server_error` annotates returned `5xx` responses with stable error metadata. It does not catch failures, change status codes, or construct the public error envelope. Handled `4xx` outcomes are not server failures.

`DefaultBodyLimit` protects Axum body-buffering extractors. It is not a transport-wide byte counter; streaming or direct `Body` consumers need an explicit route-specific contract.

## Choose the smallest middleware shape

Use Axum `from_fn` for local async request/response logic and `from_fn_with_state` only when configured state is actually needed. Implement Tower `Layer`/`Service` only for a reusable configurable component or when an installed Tower component already solves the requirement.

Use `layer` for policy that must also observe Router fallbacks. Use `route_layer` for matched-route policy that may short-circuit, such as authentication, so unknown paths can retain the service's 404 behavior.

Axum expects the service exposed to Hyper to be infallible. Any fallible Tower layer needs an outer error translation that preserves the fixed `ApiError` response contract.

## Conditional capabilities

Before adding any of the following, establish the caller/deployment contract and verify current dependency compatibility:

- **Alternative tracing:** do not install another request-span authority beside fastrace without a deliberate migration/bridge.
- **Compression:** measure payload/CPU/streaming behavior before enabling it.
- **Request IDs:** define trust, generation, propagation, and logging rules; request IDs do not replace distributed trace identity.
- **Deadlines/timeouts:** define cancellation, mutation/idempotency, retry, and public response semantics first. A timeout does not prove an external write was rolled back.
- **CORS:** derive origins/methods/headers/credential behavior from the browser contract; CORS is not access control.
- **Authentication:** HTTP extracts credentials and creates request identity; Application/Domain owns business authorization; Infrastructure implements external verification behind a Port when needed.
- **Rate limiting:** define the quota key, trusted identity source, replica consistency, storage, and public `429` behavior before selecting a library.
- **Sessions/cookies/CSRF:** define the credential/session lifecycle and browser threat model first. Cookie presence alone does not select a CSRF mechanism.

Do not add a middleware framework, DI container, cache, session store, or background cleanup task merely because an example exists.

## Ownership and redaction

- `http` owns HTTP metadata parsing, request extensions, middleware responses, and `ApiError` conversion.
- `app` owns construction/configuration when secrets, stores, exporters, or lifecycle are involved.
- `application` owns use cases and Ports for external capabilities.
- `infrastructure` implements those Ports and classifies concrete dependency failures.
- `domain` owns business invariants and never sees headers, cookies, Axum, spans, or middleware types.

Never log or trace secrets, database URLs, bodies, headers, query strings, SQL text, session values, bearer tokens, cookies, CSRF tokens, rejected values, or raw dependency errors. Record only bounded operational metadata with an explicit purpose.

## Verification

Drive middleware through the public installed Router. Cover the exact path a new layer can break: ordering, early return, 404/405 preservation, error envelope, body limits, headers, timeout, or request extensions.

Finish with `just check` for database-free middleware logic. Use `just verify` instead when configuration, production composition, lifecycle, or the installed route graph changes; `just verify` already includes the check gate.
