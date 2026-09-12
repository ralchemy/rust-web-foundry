# Async and cancellation

> **Status:** Development reference
>
> **Baseline:** The generated process owns one Axum server and no background worker, actor, channel loop, `CancellationToken` tree, or detached production task.
>
> **Read when:** Adding `tokio::spawn`, `spawn_blocking`, `select!`, `join!`, a timeout around mutating work, a channel loop, periodic work, a background job, a lock across async code, or another long-lived subsystem.
>
> **Authority:** Requested behavior and confirmed decisions own intended semantics; current source, tests, manifests, and gates own executable facts. This chapter is on-demand guidance only and adds no workflow gate.

Dropping an incomplete Future is cancellation. Owned Rust values are cleaned up, but external effects that already happened are not undone automatically.

## Classify cancellation points

Before placing work in `select!` or behind a timeout, establish what progress may already exist, whether retry can duplicate an effect, who observes cancellation, and whether idempotency, a transaction, compensation, or an indeterminate result is required.

Choose primitives by completion semantics:

| Need | Primitive |
|---|---|
| wait for several concurrent futures | `join!` / `try_join!` |
| race alternatives | `select!`, after checking losing branches |
| independently scheduled async work | `tokio::spawn`, with an owner for its result |
| bounded synchronous work | `spawn_blocking` |
| changing set of owned tasks | `JoinSet` or equivalent |
| several real long-lived subsystems | `CancellationToken` plus awaited completion |

`select!` is not CPU parallelism. A timeout drops the inner Future and does not prove that a database or downstream mutation did not commit.

## Current server pattern

`app::server::serve` owns one Axum server Future and races it with shutdown. If shutdown wins, it starts graceful shutdown and awaits that same server Future under `SHUTDOWN_TIMEOUT_SECS`. Keep this simpler lifecycle until another independently owned subsystem actually exists.

Readiness timeouts are different from mutation timeouts: abandoning a bounded `SELECT 1` has no business write to reconcile. Judge each timeout by the operation it encloses.

## Spawned work requires ownership

Dropping a Tokio `JoinHandle` detaches its task. Production-spawned work needs a lifecycle owner that retains and awaits the handle and decides how panic, early exit, cancellation, and shutdown timeout affect the process.

If the caller needs the result immediately, direct `.await` is simpler. If work must survive process restart or move across replicas, use a durable queue/store and an independently owned worker rather than an in-process detached task. If a database change and publication must be atomic, prefer an outbox-style durable workflow over holding a database transaction open across an external call.

## Channels, locks, and blocking work

A channel is in-process coordination, not durable messaging. Pick `mpsc`, `oneshot`, `broadcast`, or `watch` from delivery semantics, define closure/backpressure behavior, and make the receiver's task part of application lifecycle.

Do not hold a synchronous mutex guard across `.await`. Keep critical sections short. Use `spawn_blocking` only for bounded synchronous work; started blocking work cannot be reliably aborted and therefore participates in shutdown reasoning.

## Verification

Tests should drive cancellation, timeout, and shutdown through the public lifecycle or operation seam. Review must verify that losing branches are safe to drop, spawned tasks have observed results, timeouts have an operation-specific side-effect contract, and channel/worker shutdown is intentional.
