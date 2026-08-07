# Version business routes and own Router failures

## Decision

Business routes are nested under `/api/v1`; health routes remain unversioned. The outer Router converts unmatched paths and unsupported methods into the fixed `ApiError` JSON envelope.

## Why

The public Task contract needs an explicit compatibility namespace, while liveness and readiness describe the running process rather than a versioned business resource. Axum's default 404 and 405 responses bypassed the HTTP-owned envelope, so the documented unified error boundary was not true for the installed Router.

One nested Router and two outer fallbacks make those contracts visible at the interface that owns them. Version names do not enter Application or Domain.

## Rejected alternatives

- Keeping `/tasks` as an alias would create a compatibility promise in a new template.
- Versioning health endpoints would couple operational probes to business-contract evolution.
- Adding controller, responder, or version framework modules would only wrap the existing Router and handlers.
