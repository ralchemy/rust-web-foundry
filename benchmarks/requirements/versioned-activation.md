# Held-out requirement: versioned activation

This benchmark requirement is intentionally not part of the generated reference domain.

Add activation for a `Subscription` capability. A subscription starts `draft` at revision 1. `POST /api/v1/subscriptions/{id}/activate` receives the revision observed by the caller. Activation is valid only from `draft`. A successful activation changes status to `active` and increments revision once. If two requests use the same observed revision, at most one may succeed; the other returns the project's conflict response. Missing subscription, illegal lifecycle transition, stale revision, and infrastructure failure remain distinguishable.

Do not add authorization, expiry, retries, idempotency guarantees, Domain Events, Outbox, CQRS, or additional lifecycle states unless required by existing project evidence.

Acceptance must include a real persistence/concurrency proof and production composition path, not only an in-memory fake.
