# State management

State is an ownership decision, not a convenient place to store every shared object. The generated service separates process resources, HTTP capabilities, request context, and business data so each remains with the module that owns its lifecycle and invariants.

## Four kinds of state

| Kind | Owner | Examples | Transport |
|---|---|---|---|
| Process lifecycle | `app` | Router, MySQL pool, shutdown order | app-owned service values |
| Configured HTTP capability | `http` | Task use cases, `ReadinessProbe` | Axum `State` |
| Request-derived context | HTTP middleware/request | authenticated principal, request metadata | request extensions |
| Business state | Domain / Infrastructure | Task values and persisted rows | Domain types and Ports |

Do not collapse these into a global dependency container. HTTP state never needs to expose settings, secrets, a SQLx pool, reqwest clients, or concrete Infrastructure adapters. `app` constructs those once and gives HTTP only configured Application capabilities.

## Default and reference state

The default service installs only `HealthState<H>` because it has no Task routes.

With `reference-task`, Axum installs one private aggregate state:

```text
HttpState<P, R, H>
├── TaskState<P, R>
│   ├── CreateTask<P, R>
│   ├── GetTask<R>
│   └── StartTask<R>
└── HealthState<H>
    └── ReadinessProbe
```

Handlers extract focused substates rather than the aggregate. Manual `FromRef` implementations expose only the capability group needed by a handler family. Do not switch to trait objects or add a DI container merely to reduce generic syntax.

Add another substate only when a current handler family needs a genuinely distinct configured capability. Do not create one substate per field mechanically, and do not expose the aggregate just to avoid a deliberate mapping.

## State versus request extensions

Use `State` for capabilities installed when the Router is built and shared by matching requests. Use request extensions for values derived from the current request by middleware. A future authenticated principal is request context; a verifier used to construct that principal is configured capability/state.

Do not use extensions as a runtime-typed replacement for configured state.

## Cloning and mutation

Clone handles, not resources. SQLx pools and reqwest clients already share their underlying resources through their owning Infrastructure adapters; use cases may clone those cheap handles through their concrete types. Do not construct a pool, client, repository, or use case per request.

The baseline Router state is immutable after construction. Do not preconfigure `Mutex`, `RwLock`, DashMap, or a cache for hypothetical needs. Real mutable process-local coordination belongs with the module that owns its invariant and should expose operations rather than locks. State that must survive restart or coordinate replicas belongs behind an external Application Port.

## Verification

HTTP tests build the public Router with fake Application capabilities, which compile-checks the `FromRef` mappings. Cross-crate tests use the same app-owned composition path as production to prove concrete adapters remain hidden behind those capabilities.
