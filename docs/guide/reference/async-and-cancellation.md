# Async and cancellation

> **Status:** Development reference
>
> **Baseline:** The generated process owns one Axum server and no background worker, actor, channel loop, `CancellationToken` tree, or detached production task.
>
> **Read when:** Adding `tokio::spawn`, `spawn_blocking`, `select!`, `join!`, a timeout around mutating work, a channel loop, periodic work, a background job, a lock across async code, or another long-lived subsystem.
>
> **Authority:** When a Context Pointer matches, this chapter is the conditional async and cancellation contract. Source, tests, and gates own executable facts; root and nearest-local rules retain the standing trigger, scope responsibility, and local hard protection.

Dropping an incomplete Future is cancellation. Rust cleans up its owned values, but external effects that already happened—bytes sent, a downstream request accepted, or a database transaction committed—are not undone automatically.

## Classify every cancellation point

Before placing an operation in `select!` or behind a timeout, answer:

1. At which `.await` points can the Future be dropped?
2. What local and external progress may already exist?
3. Is restarting the operation a no-op, a retry, a duplicate, or corruption?
4. Who observes and records the cancellation?
5. Does the caller need an idempotency key, transaction, compensating action, or an indeterminate result?

Tokio's pinned [`select!` documentation](https://docs.rs/tokio/1.53.1/tokio/macro.select.html) lists common cancellation-safe and unsafe operations. Receiving from Tokio channels and accepting a connection are safe to restart; operations such as `read_exact`, `read_to_end`, `read_to_string`, and `write_all` may lose partial progress when repeatedly cancelled. Fair queued acquisition such as `Mutex::lock` and `Semaphore::acquire` may lose queue position.

## Choose the concurrency operation deliberately

| Need | Primitive |
|---|---|
| Run several Futures concurrently and wait for all | `join!` / `try_join!` |
| Race alternatives and continue with the first result | `select!`, after checking losing branches |
| Run an independently scheduled async task | `tokio::spawn`, with an owner for its result |
| Run bounded synchronous work | `spawn_blocking`, accounting for its non-abortable execution |
| Own a changing set of similar tasks | `JoinSet` or an equivalent owned collection |
| Signal several real long-lived subsystems | `tokio_util::sync::CancellationToken`, plus awaited task completion |

`select!` runs its branches concurrently on the current task; it is not a shortcut for parallel CPU work. `join!` does not cancel siblings when one completes. Pick based on completion semantics, not syntax.

## The current server pattern

[`app::server::serve`](../../../app/src/server.rs) pins one Axum server Future and races it against the shutdown signal. If the signal wins, the code does not abandon the losing server branch: it sends the graceful-shutdown notification and awaits that same pinned Future under `SHUTDOWN_TIMEOUT_SECS`.

This small lifecycle interface is enough because the baseline has one long-lived subsystem. Do not replace it with a task tree or root `CancellationToken` until another independently owned subsystem must receive shutdown and be joined.

The readiness timeout is different. Dropping its bounded `SELECT 1` Future produces an unavailable readiness result and has no business write to reconcile. Each timeout must be judged by the operation it encloses.

## Timeouts do not roll back mutations

`tokio::time::timeout` returns an elapsed error and cancels by dropping the inner Future, but completed external effects remain completed. See the pinned [`timeout` documentation](https://docs.rs/tokio/1.53.1/tokio/time/fn.timeout.html).

Before adding an HTTP deadline around Task creation, define what the client should do if TaskPolicy accepted the request or MySQL committed after the client-facing timeout. The [Middleware Guide](../http/middleware.md#deadlines-and-timeouts) therefore requires idempotency and retry semantics before exposing a timeout response for mutating work.

Library timeouts and HTTP deadlines serve different owners. The reqwest client timeout bounds the outbound adapter; the readiness timeout protects the probe; an outer request deadline bounds client waiting. Stacking them without a budget creates ambiguous failures.

## Spawned tasks require ownership

Dropping a Tokio `JoinHandle` detaches its task. Production code must retain a handle in the lifecycle owner, await it, and decide how cancellation, panic, early exit, and shutdown timeout affect the process. Review the pinned [`JoinHandle` documentation](https://docs.rs/tokio/1.53.1/tokio/task/struct.JoinHandle.html).

Use a spawned task only when the work must be scheduled independently from the current Future. If the caller must wait for the result immediately, direct `.await` is simpler and preserves error propagation.

When several long-lived subsystems become real, structured shutdown normally means:

1. stop accepting new work;
2. signal every owned subsystem;
3. close input channels so loops can finish;
4. await every handle;
5. apply an explicit outer bound;
6. propagate panic, early exit, and timeout instead of logging success.

`tokio-util` and `futures` are preconfigured workspace dependencies, but their presence is not a reason to create a subsystem or abstraction.

## Choose channel semantics before channel type

A channel is in-process coordination, not durable messaging. Messages disappear with the process, and a successful send proves only that the channel accepted a value. It does not prove that a consumer completed the work.

| Need | Tokio primitive | Required decision |
|---|---|---|
| many producers, one ordered consumer | bounded `mpsc` | capacity, backpressure, closure, and consumer failure |
| one reply to one command | `oneshot` | what sender or receiver loss means to the operation |
| every active subscriber sees a value | `broadcast` | how lagged receivers recover from missed messages |
| consumers need only the latest value | `watch` | whether intermediate changes may be discarded |

Prefer bounded `mpsc`; an unbounded queue converts overload into memory growth. Tokio documents the delivery and closure contracts for [`mpsc`](https://docs.rs/tokio/1.53.1/tokio/sync/mpsc/), [`broadcast`](https://docs.rs/tokio/1.53.1/tokio/sync/broadcast/), and [`watch`](https://docs.rs/tokio/1.53.1/tokio/sync/watch/) separately; they are not interchangeable notification APIs. Do not add an actor merely to avoid a lock. A single owner loop is useful when commands must be serialized around one coherent state and its lifecycle can be observed. Direct calls or a short lock are simpler when there is no independently scheduled owner.

The task that owns a receiver is a long-lived subsystem. `app` must retain its handle, define whether an unexpected exit stops the process, close its senders during shutdown, and await it. Channel client wrappers belong with the capability they expose, not in `common` or HTTP state as a generic message bus.

## Select background work by delivery contract

| Required result | Smallest fitting model |
|---|---|
| the caller must know success or failure | await the operation in the current use case |
| loss on restart is explicitly acceptable | an app-owned in-process task with an observed handle |
| work must survive restart or move across replicas | a durable database or broker queue and an independently owned worker |
| a database change and job publication must be atomic | write business state and an outbox record in one database transaction, then deliver separately |

Do not detach work from an HTTP handler. If a response may return before work completes, the public contract must say what was accepted, how status is observed, and whether duplicate delivery is possible. Durable workers additionally need claim/lease semantics, idempotent processing, bounded retries, dead-letter or terminal failure handling, and queue-depth/age visibility before a queue library is selected.

For periodic in-process work, define missed-tick behavior and overlap explicitly. Tokio's [`MissedTickBehavior`](https://docs.rs/tokio/1.53.1/tokio/time/enum.MissedTickBehavior.html) defaults to `Burst`; maintenance work commonly needs `Skip`, but only the product timing contract can decide. A periodic loop is still owned, cancellable production work and follows the same shutdown rules.

Cross-system ordering cannot create an atomic HTTP-plus-database transaction. If an external acceptance must happen before a database write, call it before opening the database transaction, then define idempotency, compensation, or reconciliation for the case where the external action succeeds and the database write fails. If both outcomes must be durably tracked, model the workflow state rather than holding a database transaction open across the request. See [Outbound HTTP](outbound-http.md#cross-system-consistency).

## Blocking work and locks

An async worker must not perform long synchronous I/O or CPU work because other Futures on that worker cannot progress. `spawn_blocking` is for bounded operations that eventually finish; once started, it cannot be reliably aborted. Limit CPU-bound concurrency or choose a dedicated compute executor when measurements require it.

Do not hold a synchronous mutex guard across `.await`. An async mutex allows awaiting while held but still serializes all contenders for the entire hold time. Prefer moving computation outside the critical section, shortening the owned mutation, or giving one task ownership of state when concurrent mutation is the real requirement.

The `std::sync::Mutex` values in current tests guard tiny in-memory records and are never held across `.await`; they are not a production state-management pattern.

## Review checklist

- Losing `select!` branches are safe to drop or deliberately abandoned.
- Each spawned task has one lifecycle owner and an observed result.
- Timeouts have an operation-specific error and side-effect contract.
- Blocking work is bounded and included in shutdown reasoning.
- Locks are not held across unrelated I/O.
- Channel closure and `None` cases terminate loops intentionally.
- Tests drive cancellation through the public lifecycle or operation seam rather than only a helper.
