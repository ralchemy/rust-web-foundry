# Project structure

The generated service uses one fixed Cargo workspace. Its crates establish architectural boundaries; responsibility directories inside each crate make the owner of a change predictable.

```text
app/                         # composition, configuration, lifecycle
crates/
├── domain/src/
│   ├── entities/
│   ├── value_objects/
│   ├── errors/
│   └── services/             # only after a real cross-concept rule exists
├── application/src/
│   ├── use_cases/
│   ├── ports/
│   └── errors/
├── http/src/
│   ├── routes/
│   ├── handlers/
│   ├── dtos/
│   ├── errors/
│   └── middleware/
└── infrastructure/src/
    ├── mysql/
    └── outbound_http/
```

Cargo manifests enforce the inward dependency graph. Inside a crate, Rust visibility provides the next boundary: keep implementation details private and expose only the types and operations another crate needs.

Place new behavior with the responsibility that owns it:

- domain concepts and invariants belong under `entities/`, `value_objects/`, or `errors/`; a pure cross-concept rule may introduce `services/` when it first exists;
- orchestration and required external capabilities belong under application `use_cases/` and `ports/`;
- transport translation belongs in the matching HTTP responsibility directory;
- concrete MySQL and reqwest behavior stays with its infrastructure adapter;
- process construction and lifecycle stay in root `app/`, with `main.rs` remaining a thin call into the library path used by integration tests.

A feature touches only the layers its behavior requires. Reuse an existing responsibility directory before adding another one, and create a directory only when a distinct current responsibility needs an owner. Do not add `shared`, `common`, `utils`, empty feature trees, or placeholders for anticipated adapters.

## Grow a file into a directory module

Start a responsibility in one file when only one workflow is known. When it grows another independently named workflow, promote the file to a directory module. When a confirmed design already contains several related workflows, create the capability directory from the start rather than adding several top-level command files. For example, a complex Application workflow may grow from `permission_requests.rs` into:

```text
crates/application/src/use_cases/permission_requests/
├── mod.rs
├── create.rs
├── renew.rs
└── revoke.rs
```

Keep `mod.rs` as the small module interface: declare private child modules and expose only what callers need. Each child owns a complete workflow, including its input, result, orchestration, and focused tests. Do not leave the implementation in `mod.rs` or extract child modules that only forward calls.

Split each crate by the responsibility it owns rather than copying the same tree through every layer:

| Crate | Useful split boundary |
| --- | --- |
| Domain | concept, invariant, policy, or state transition |
| Application | complete command or coherent query group |
| HTTP | public interaction or route family |
| Infrastructure | implemented Port, transaction boundary, or external system |

File length alone and CRUD symmetry are not split boundaries. Keep small related operations together when separate files would add only declarations and navigation. Move the smallest complete responsibility first, compile, and run its focused tests before extracting another one.

Guide chapters follow the same ownership rule. Crate-specific explanations live under their responsibility directory; cross-cutting implemented paths such as Task flow, observability, security, runtime, testing, and development stay at the Guide root rather than being assigned to one crate.

Conditional cross-cutting knowledge lives under `docs/guide/reference/`. A reference directory is not another architecture layer: root and crate Project Rules link to an exact chapter when a task creates its concern, and the chapter points back to the owning source seam.

Keep invariant tests beside their owning type, orchestration tests beside their use case, and adapter behavior tests at the public adapter seam. Use `app/tests/` only when a test must prove the real cross-crate composition path; do not create a generic test-support hierarchy before several tests share a coherent setup responsibility.
