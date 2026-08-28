# HTTP Adapter Rules

## Responsibility

This crate owns the Axum Router, handlers, DTOs, extractors, middleware, state, and public error translation. Place each change in the matching responsibility directory.

## Public adapter contract

- Depend only on `application` and `domain` inside the workspace; never depend on `infrastructure`.
- Keep the aggregate Router state private and limited to configured Application capabilities. Handlers extract the smallest capability state through `FromRef`; never expose configuration, pools, repositories, clients, or concrete adapters to them.
- Keep one production Router constructor and update all callers when its required capabilities change. Do not preserve an older signature with a second constructor that silently installs `Noop`, unavailable, or test adapters; test-only fakes stay under `#[cfg(test)]` and enter through the production constructor.
- Use `State` for startup-injected capabilities and request extensions only for values derived from the current request. Construct neither inside a handler.
- Keep handlers thin: extract, convert at the transport boundary, call the narrowest existing inward owner, and translate the result. Use an Application use case or Port for orchestration or business decisions; call a Domain constructor directly only for a pure invariant operation that needs no Application capability.
- Keep business routes under `/api/v1` and health routes under `/health`; preserve `/health/live` and `/health/ready`, and do not propagate HTTP versions inward.
- Default JSON request DTOs to rejecting unknown fields, and preserve the Router's 8 KiB limit for bounded body extractors. Declare and test any route-specific exception.
- Business endpoints return named response DTOs. Do not assemble public contracts with `serde_json::Value`, `json!`, maps, or anonymous tuples.
- Keep transport validation in HTTP, but never duplicate or replace a Domain invariant there. Construct Domain values through their validating constructor before calling external capabilities.
- Translate validation failures through `ApiError`; never expose library error objects, messages, parameters, or rejected values. Adding field errors changes the public API contract.
- Map every failure, including Router-level 404 and 405, through the fixed error envelope without internal details.
- Keep `FastraceLayer` outside `mark_server_error`; direct chained `Router::layer` calls execute from the last added Layer inward. Do not reorder the installed stack without proving request and response behavior through the public Router.
- `DefaultBodyLimit` protects only extractors that apply it. Any custom or streaming body consumer must declare and test its own limit.
- Middleware must return a response through the HTTP error contract; do not let a fallible Tower Layer close the connection with an unhandled service error.
- Keep health request spans noop; create request/use-case spans without recording titles, bodies, headers, queries, or secrets.

## Proof

- Public HTTP behavior changes are proved through the installed Router.
