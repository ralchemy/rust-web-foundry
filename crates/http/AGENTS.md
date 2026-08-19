# HTTP Adapter Rules

This crate owns the Axum Router, handlers, DTOs, extractors, middleware, state, and public error translation. Place each change in the matching responsibility directory.

Read `docs/guide/http/state-management.md` before changing Router state, handler state extraction, request extensions, or dependency construction.
Read `docs/guide/http/routing-and-handlers.md` before changing routes, handlers, extractors, response DTOs, or Router fallbacks.
Read `docs/guide/http/validation.md` before adding DTO validation, `axum-valid` rejection mapping, or field-level public errors.
Read `docs/guide/http/middleware.md` before adding or reordering a Layer, changing body consumption, or passing request-derived context to handlers.
Read `docs/guide/observability.md` before changing request trace extraction, span attributes, error marking, or health-span policy.
Read `docs/guide/reference/idiomatic-rust.md` before adding or changing DTO conversion traits, parsing, ownership, or naming conventions.

- Depend only on `application` and `domain` inside the workspace; never depend on `infrastructure`.
- Keep the aggregate Router state private and limited to configured Application capabilities. Handlers extract the smallest capability state through `FromRef`; never expose configuration, pools, repositories, clients, or concrete adapters to them.
- Keep one production Router constructor and update all callers when its required capabilities change. Do not preserve an older signature with a second constructor that silently installs `Noop`, unavailable, or test adapters; test-only fakes stay under `#[cfg(test)]` and enter through the production constructor.
- Use `State` for startup-injected capabilities and request extensions only for values derived from the current request. Construct neither inside a handler.
- Keep handlers thin: extract, convert at the transport boundary, call the narrowest existing inward owner, and translate the result. Use an Application use case or Port for orchestration or business decisions; call a Domain constructor directly only for a pure invariant operation that needs no Application capability.
- Keep business routes under `/api/v1` and health routes at `/health/live` and `/health/ready`; do not propagate HTTP versions inward.
- Split HTTP modules by public interaction or route family, not mechanically by every route. Use the same workflow vocabulary across routes, handlers, DTOs, tests, and documentation without requiring identical directory trees.
- When a route or handler file owns multiple independently testable interactions, promote it to a directory module and move each complete interaction or coherent route group into a private child module.
- Keep `routes/mod.rs` focused on composing route-family Routers. Route declarations and route-specific state belong to the owning route-family module.
- An extracted handler module owns request extraction, boundary conversion, the Application call, response conversion, and focused public-path tests. Keep a trivial forwarding handler local instead of creating a one-function directory.
- Default JSON request DTOs to rejecting unknown fields, and preserve the Router's 8 KiB limit for bounded body extractors. Declare and test any route-specific exception.
- Keep transport primitives in private DTOs. Use `TryFrom` or `FromStr` for fallible DTO/path conversion before calling inward, and use `From` for an infallible Application-result-to-response conversion.
- Keep conversion implementations beside the HTTP representation they decode or encode. Do not expose public `to_domain`, `into_domain`, or `to_response` methods when a standard conversion trait expresses the same seam.
- Business endpoints return named response DTOs. Do not assemble public contracts with `serde_json::Value`, `json!`, maps, or anonymous tuples.
- Keep transport validation in HTTP, but never duplicate or replace a Domain invariant there. Construct Domain values through their validating constructor before calling external capabilities.
- Use Axum extractors directly and add `axum_valid::Valid` for transport DTO validation. Convert their rejections through `ApiError`; do not create a local generic extractor wrapper.
- Translate validation failures through `ApiError`; never expose library error objects, messages, parameters, or rejected values. Adding field errors changes the public API contract.
- Map every failure, including Router-level 404 and 405, through the fixed error envelope without internal details.
- Keep `FastraceLayer` outside `mark_server_error`; direct chained `Router::layer` calls execute from the last added Layer inward. Do not reorder the installed stack without proving request and response behavior through the public Router.
- `DefaultBodyLimit` protects only extractors that apply it. Any custom or streaming body consumer must declare and test its own limit.
- Middleware must return a response through the HTTP error contract; do not let a fallible Tower Layer close the connection with an unhandled service error.
- Keep health request spans noop; create request/use-case spans without recording titles, bodies, headers, queries, or secrets.
- Test behavior through the public installed Router, not a detached handler helper.
