# State management

State is an ownership decision, not a convenient place to store every object that several requests might use. The generated service separates process resources, HTTP capabilities, request context, and business data so each remains with the module that owns its lifecycle and invariants.

## Four kinds of state

| Kind | Owner | Examples | Transport |
|---|---|---|---|
| Process lifecycle | `app` | Router, MySQL pool, shutdown order | `BuiltService` |
| Configured HTTP capability | `http` | `CreateTask`, `ReadinessProbe` | Axum `State` |
| Request-derived context | HTTP middleware/request | authenticated principal, request-local metadata | request extensions |
| Business state | Domain and Infrastructure | Task values and persisted rows | Domain types and Ports |

Do not collapse these into a global dependency container. In particular, HTTP state never contains settings, secrets, a SQLx pool, repositories, reqwest clients, or concrete Infrastructure adapters. `app` constructs those implementations once and gives HTTP only the configured Application capabilities its handlers invoke.

## Aggregate state and focused substates

Axum installs one private `HttpState<P, R, H>` on the Router. Handlers do not extract that aggregate. [`state.rs`](../../../crates/http/src/state.rs) exposes two crate-private capability slices:

```text
HttpState<P, R, H>
├── TaskState<P, R>   → CreateTask<P, R>
└── HealthState<H>    → ReadinessProbe
```

Manual `FromRef` implementations clone the requested slice from the aggregate. The CreateTask handler therefore has no readiness generic or capability, and the readiness handler cannot invoke Task creation. This smaller interface makes accidental coupling visible in the handler signature.

Axum also provides `#[derive(FromRef)]` for non-generic structs. The template keeps static generic dispatch for Application Ports, and Axum's derive does not support a generic state declaration, so the two implementations are explicit. Do not switch to trait objects or add a DI container merely to use the derive macro.

Add another substate only when a current handler family needs a distinct configured capability. Do not create one substate per field mechanically, and do not expose the aggregate to avoid writing a deliberate `FromRef` mapping.

## State versus request extensions

Use `State` for capabilities provided when the Router is built and shared by every matching request. Use request extensions for values derived from the current request by middleware. A future authenticated principal, for example, would be request context; a verifier used by authentication middleware would be configured state.

Missing or mismatched Router state is caught through Axum's typed handler and Router composition. Request extensions remain request-local and must be inserted before a consumer extracts them. Do not use extensions as a runtime-typed replacement for configured State.

## Cloning and ownership

Axum clones extracted state. Clone handles, not resources:

- SQLx pools and reqwest clients already share their underlying resources when cloned inside Infrastructure adapters.
- `CreateTask` and the readiness adapter clone those handles through their concrete types.
- Do not wrap the entire state in `Arc` when every field is already cheap to clone.
- Never create a new pool, client, repository, or use case per request.

The pool also remains in app-owned `BuiltService`. That ownership is operational: shutdown first drains HTTP requests, then closes the pool, then flushes tracing. Moving the pool into HTTP state would hide the handle needed to enforce that sequence.

## Shared mutation

The baseline Router state is immutable after construction. Do not preconfigure `Mutex`, `RwLock`, DashMap, or an in-memory cache for hypothetical use.

When a real process-local coordination requirement appears, place the mutable value and its operations in the module that owns the invariant. Expose an operation rather than the lock, and never hold a lock guard across `.await`. State that must survive restarts or coordinate several replicas belongs in MySQL or another external capability behind an Application Port, not in one process's Router state.

## Verification

HTTP tests build the public Router with fake Application capabilities. Because handlers extract `TaskState` and `HealthState`, Router construction itself compile-checks the `FromRef` mappings. Cross-crate tests call the same app-owned `build` function as production and prove that concrete adapters remain hidden behind those capabilities.
