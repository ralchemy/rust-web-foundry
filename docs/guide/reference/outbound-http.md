# Outbound HTTP

> **Status:** Development reference
>
> **Baseline:** `HttpTaskPolicy` reuses one reqwest client, accepts one configured HTTP(S) endpoint, disables redirects, applies a finite timeout, performs no retry, propagates fastrace context, and returns stable Application error categories.
>
> **Read when:** Adding a downstream, changing client construction or timeout budgets, retrying requests, accepting destinations from input, or coordinating an external effect with MySQL.
>
> **Authority:** Infrastructure and security Project Rules, the TaskPolicy Port contract, and [Async cancellation rules](async-and-cancellation.md) override conditional options here.

One configured downstream capability should have one typed Infrastructure adapter that owns its reqwest details and implements an Application Port. Construct and reuse its client in `app`; reqwest clients own connection pools and should not be rebuilt per request. Keep base URLs, credentials, wire DTOs, status/body parsing, and concrete failures outside Application and Domain.

## Bound time and data

Separate the budgets that matter: connection establishment, one request attempt, and the whole use-case operation. An outer deadline must include all attempts and backoff. A timeout returns uncertainty about an external mutation; it does not prove that the peer rejected or rolled it back.

Define redirect behavior, accepted status codes, maximum response size, and strict response decoding. A future user-selected destination is an SSRF boundary and needs destination and network policy beyond URL syntax; see [Security](../security.md). Never record full URLs, queries, headers, bodies, credentials, or raw downstream errors.

## Retry only a retry-safe operation

Retries are a business delivery policy, not a universal client feature. Before enabling one, answer:

1. Can the first attempt have succeeded even though no response arrived?
2. Does repeating the operation have the same intended effect?
3. Is there a stable idempotency key or a way to reconcile the result?
4. Which transport errors and response statuses are transient for this specific dependency?
5. Do bounded attempts plus backoff fit inside the caller's deadline and shutdown contract?

HTTP defines safe methods plus `PUT` and `DELETE` as idempotent, but application semantics still matter. RFC 9110 says clients should not automatically retry a non-idempotent request unless they know its semantics are idempotent or know the original was not applied. Preserve the same idempotency key across every attempt; generating a new key per retry defeats the guarantee. See [RFC 9110 section 9.2.2](https://www.rfc-editor.org/rfc/rfc9110.html#section-9.2.2).

When retry is justified, keep it bounded, use backoff with jitter, respect dependency-specific rate-limit guidance, and expose attempt exhaustion as one stable category. Do not prescribe “retry all 5xx” or a fixed attempt count across services. A circuit breaker or bulkhead is warranted only after measurements show repeated downstream failure consumes local capacity; it adds shared state, probing, and replica-level operational behavior that a timeout alone does not.

## Cross-system consistency

MySQL and an HTTP peer do not share an atomic transaction. Choose the failure you can represent and recover:

| Required ordering | Pattern | Residual failure to handle |
|---|---|---|
| external decision must precede a local write | call external first, then use a short local transaction | external success followed by local failure needs idempotent retry, compensation, or reconciliation |
| local truth must commit before delivery | commit local state plus an outbox record, then deliver asynchronously | duplicate delivery requires an idempotent consumer |
| a multi-step business process spans systems | persist explicit workflow state and compensating transitions | partial progress remains visible until reconciled |

Do not hold a database transaction open across an external request. If the downstream check is read-only, as TaskPolicy is intended to be, call it before the one local write and let a failure short-circuit persistence. If a future downstream call causes a durable effect, change the Application contract so indeterminate and compensating outcomes are explicit rather than hiding them as a generic internal error.

## Verification

Use a controllable local HTTP peer and drive the real adapter. Prove request shape, trace propagation, redirect/timeout behavior, response classification, and that later effects do not run after failure. For a retry policy, additionally prove duplicate-risk cases, the overall deadline, attempt bounds, and stable idempotency keys without sleeping for production durations.
