# Project structure

The generated service uses one fixed Cargo workspace. Keep the four architecture crates under `crates/` and the executable host at root `app/`; introduce `apps/` only when a second real executable host exists.

```text
app/                         # composition, configuration, lifecycle
crates/
├── domain/src/              # entities, value objects, pure rules, domain errors
├── application/src/         # use cases, Ports, stable application errors
├── http/src/                # routes, handlers, DTOs, middleware, public errors
└── infrastructure/src/      # MySQL and outbound adapters
```

Cargo manifests enforce the inward dependency graph. Within a crate, visibility and module boundaries keep implementation details private. Expose only the types and operations another crate actually needs.

Place behavior with the responsibility that owns it:

- Domain concepts and invariants stay in Domain;
- orchestration and required external capabilities stay in Application;
- transport translation stays in HTTP;
- concrete MySQL and downstream behavior stays in Infrastructure;
- process construction and lifecycle stay in `app`, with `main.rs` remaining a thin call into the library path used by integration tests.

A feature touches only the layers its behavior requires. Reuse a current responsibility before adding another. Do not add generic `shared`, `common`, `utils`, `helpers`, placeholder adapters, or empty feature trees. A small amount of local duplication is preferable to an ownerless abstraction.

## Grow by responsibility

Start a responsibility in one file when only one workflow is known. Promote it to a directory module when multiple related workflows can evolve or be tested independently. Keep `mod.rs` as the module interface and put complete workflows in child modules.

Useful split boundaries differ by layer:

| Crate | Useful split boundary |
|---|---|
| Domain | concept, invariant, policy, or state transition |
| Application | complete command or coherent query group |
| HTTP | public interaction or route family |
| Infrastructure | implemented Port, transaction boundary, or external system |
| App host | command, composition path, or lifecycle owner |

File length and CRUD symmetry are not split boundaries. Keep inseparable operations together when splitting would add only forwarding code and navigation; keep the reason visible in code or the change description. Never mirror the same directory tree mechanically across all layers.

The generic `rust-skills` rule `proj-mod-by-feature` applies only within the owning architecture layer. It does not permit combining Domain, Application, HTTP and Infrastructure into one vertical feature module; see `.agents/rust-skills-overrides.md`.

Keep invariant tests beside their type, orchestration tests beside their use case, and adapter behavior tests at the adapter's public seam. Use `app/tests/` only when a test must prove the real cross-crate composition path.
