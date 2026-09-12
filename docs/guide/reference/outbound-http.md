# Outbound HTTP

> **Status:** Development reference
>
> **Baseline:** The default generated service has no outbound HTTP dependency. With `reference-task`, `HttpTaskPolicy` reuses one reqwest client, accepts one configured HTTP(S) endpoint, disables redirects, applies a finite timeout, performs no retry, propagates fastrace context, and returns stable Application error categories.
>
> **Read when:** Adding a downstream, changing client construction or timeout budgets, retrying requests, accepting destinations from input, or coordinating an external effect with MySQL.
>
> **Authority:** Requested behavior, the owning Application Port, Infrastructure/security boundaries, and async-cancellation guidance override generic options here.

One configured downstream capability should have one typed Infrastructure adapter that owns reqwest details and implements an Application Port. Construct and reuse the client in `app`; keep URLs, credentials, wire DTOs, status/body parsing, and concrete failures outside Application and Domain.

## Bound time and data

Separate connection, attempt, and whole-operation budgets. Define redirect behavior, accepted statuses, maximum response size, and strict response decoding. A timeout returns uncertainty about a remote mutation; it does not prove the peer rolled work back.

A user-selected destination is an SSRF boundary and requires a destination/network policy beyond URL syntax. Never record full URLs, queries, headers, bodies, credentials, or raw downstream errors.

## Retry only retry-safe work

Before enabling retries, establish whether the first attempt might have succeeded, whether repetition preserves intended effect, whether a stable idempotency key or reconciliation path exists, which failures are actually transient for this dependency, and whether bounded attempts/backoff fit the caller's deadline and shutdown contract.

Do not prescribe retrying all `5xx` responses or a universal attempt count. Preserve the same idempotency key across attempts when the operation uses one. Add circuit breakers or bulkheads only when measured failure behavior justifies their shared state and operational complexity.

## Cross-system consistency

MySQL and an HTTP peer do not share an atomic transaction. Do not hold database locks across network I/O to simulate one. Choose an explicit consistency model: remote read-only decision before a local write, idempotent remote mutation plus reconciliation/compensation, or a durable workflow/outbox when both outcomes must be tracked.

## Verification

Use a local controlled peer or another deterministic seam to prove request mapping, timeout/status/decoding classification, propagation, and redaction. Cross-system side-effect tests must prove the business retry/idempotency contract rather than only that reqwest returned an error.
