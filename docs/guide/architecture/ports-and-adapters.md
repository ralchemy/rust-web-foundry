# Ports and adapters

Ports let application use cases express required external capabilities without depending on their concrete implementations.

```text
HTTP request
    ↓
application use case
    ↓ calls an application-owned Port
infrastructure adapter
    ↓
MySQL or external HTTP
```

The use case that needs a capability owns its Port. A Port describes the operation in Domain and Application terms; it does not expose SQLx, reqwest, HTTP status, serialization, or runtime configuration. Infrastructure implements the Port, records only safe operational categories, and translates concrete failures into stable Application errors. `app` constructs the concrete types and connects both sides.

Do not create a Port for every adapter or struct. Add one when a use case currently needs to cross an external boundary. A future possibility of replacing an implementation is not sufficient.

## Domain Services and Application use cases

Prefer an Entity or Value Object method when a rule belongs to one concept. When a pure business rule spans several Domain concepts and has no I/O, Port, runtime, or orchestration responsibility, it may become a Domain Service under `crates/domain/src/services/`.

An Application use case coordinates Domain behavior and external capabilities. `CreateTask`, for example, accepts a valid `TaskTitle`, calls the Task Policy Port, creates a `Task`, and persists it through the Task Repository Port. It belongs to Application because orchestration crosses external boundaries; moving it into a Domain Service would make Domain own I/O contracts.

Do not pre-create a Domain Service directory. Add it only with the first real cross-concept Domain rule.

## Default: static dispatch

The template's Ports return `impl Future`, and use cases are generic over their concrete adapters. This keeps internal Ports directly type-checked without boxed futures, object-safety constraints, or a dependency on `async_trait`.

Static dispatch remains the default while the service has one production composition. Keep generic types inside Application, HTTP state, and `app`; do not add a DI container or application-owned trait-object registry.

## Optional: dynamic dispatch

Dynamic dispatch becomes reasonable when the generated project has a real runtime requirement such as:

- selecting an implementation from runtime configuration;
- storing heterogeneous adapters behind one Port;
- exposing a plugin boundary; or
- containing generic types that have spread across many public seams and materially harm navigation.

A small fixed set of implementations can often remain statically dispatched through an app-owned enum. When trait objects are the clearer boundary, common options are:

- `async_trait` with `Arc<dyn Port>` for concise object-safe async methods;
- an explicit `Pin<Box<dyn Future<...>>>` return for direct control without a macro;
- `trait_variant` for generating `Send` and non-`Send` variants of a public async trait.

`trait_variant` does not by itself make an async trait dynamically dispatchable. Trait objects also introduce vtable dispatch and, for the usual async patterns, boxed-future allocation. Accept those costs only for the runtime flexibility the project actually uses, and keep failure types and dependency direction unchanged.
