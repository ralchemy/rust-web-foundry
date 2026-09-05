# rust-web-foundry

A `cargo-generate` template for production-oriented Rust web services with explicit Clean Architecture boundaries and executable reference patterns for AI-assisted development.

The generated workspace has five packages. Its default server contains production composition, MySQL readiness, configuration, observability, graceful shutdown, and quality gates without installing example business routes or schema. A canonical Task slice is available behind the default-off `reference-task` feature so the repository can carry richer architectural and DDD examples without making those examples part of a user's runtime by default.

The Task slice demonstrates independent wire, Application, Domain, and database types; explicit boundary conversion; checked SQLx queries; outbound Ports; private adapter models; stable public errors; production composition tests; and named Domain state transitions. Its Task-specific rules are illustrative, not a universal task-management domain model.

## What it generates

```text
app/                         # executable host and composition root
├── src/                     # commands, configuration, wiring, lifecycle
├── examples/                # opt-in external-service peers
└── tests/                   # production-path acceptance tests
crates/
├── domain/                  # business types, entities, invariants
├── application/             # use cases, Ports, stable application errors
├── http/                    # Axum routes, DTOs, middleware, public errors
└── infrastructure/          # SQLx/MySQL and reqwest adapters
```

Dependencies point inward:

```text
http ────────────┐
                 ├──> application ───> domain
infrastructure ──┘

app ──> http + infrastructure + application + domain
```

## Generate a service

Prerequisites are stable Rust, Docker with Compose v2, `cargo-generate` 0.23.x, `just` 1.58.x, and `sqlx-cli` 0.9.x.

```sh
cargo generate --path . --name my-service
cd my-service
just verify
```

`just verify` proves both shapes: the default service does not register Task APIs or require TaskPolicy configuration, while the opt-in reference shape exercises Task HTTP, MySQL persistence, outbound policy integration, trace propagation, and shutdown behavior.

## Code-first development

Generated projects use a small root contract rather than preloading a documentation bundle or implementation workflow. Feature work starts from requested behavior, explicitly confirmed decisions, the nearest complete production path and its tests, manifests/migrations/Just/CI, and a focused failing test at the owning seam.

The Task reference is executable teaching material. Read it when its pattern answers a concrete design question, but never copy its business semantics into a user's domain without a requirement. New lifecycles, invariants, authorization decisions, consistency boundaries, or changed meanings should receive the smallest necessary design decision before implementation; established model changes do not require a repeated DDD ceremony.

The Guide remains cold documentation for concrete design questions and capabilities with no existing example.

## Rust review

Generated projects pin `leonardomso/rust-skills` to an exact commit for explicit, fresh review. Generic Rust rules are not copied into `AGENTS.md` and are not loaded during implementation. Review keeps requested behavior, project architecture/stack, and Rust quality separate, with project-specific overrides taking precedence over generic advice.

Template-default minimalism is checked by template CI. Generated projects may later add their own Skills or local agent instructions without failing the application architecture gate.

## Local commands

| Command | Purpose |
|---|---|
| `just infra-up` / `just infra-down` | Start or stop local MySQL |
| `just migrate` | Run the default migration boundary without reference schema |
| `just serve` | Start the default health/readiness service |
| `just policy-stub` | Start the opt-in local TaskPolicy peer |
| `just migrate-reference-task` | Apply reference Task migrations |
| `just serve-reference-task` | Start the server with reference Task routes |
| `just sqlx-prepare` | Refresh SQLx metadata against the reference schema |
| `just test` | Run default and reference workspace tests |
| `just architecture` | Check dependency direction and the small project/review contract |
| `just check` | Check both feature shapes, format, Clippy, DB-free tests, and SQLx style |
| `just ci` | Run checks, acceptance, SQLx verification, and lifecycle proof against existing services |
| `just verify` | Start dependencies and run the complete verification path |

## Selected stack

The generated manifests are the exact dependency authority. The reference service uses Axum/Tower, Tokio, SQLx with MySQL, Reqwest, Serde, validator/axum-valid, config/dotenvy/secrecy, fastrace/Logforth/OpenTelemetry, thiserror/anyhow, Chrono, and ULID.

## Documentation

- [Template usage and positioning](TEMPLATE.md)
- [Generated-service README template](README.md.liquid)
- [Guide](docs/guide/README.md)
- [Architecture](docs/guide/architecture/README.md)
- [Selected stack](docs/guide/stack.md)
- [Task golden path](docs/guide/task-flow.md)
- [Testing and quality gates](docs/guide/testing.md)
- [Code-first development](docs/guide/development.md)
- [Fresh Rust review](docs/guide/reviewing.md)

## Referenced projects and attribution

The primary design reference is `gruberb/bulletproof-rust-web` and its published guide. This repository is an independent `cargo-generate` template inspired by selected architectural material from that project; it is not a fork or drop-in mirror.

`leonardomso/rust-skills` is an MIT-licensed, pinned review dependency. `tyrchen/rust-lib-template` was consulted separately for template ergonomics and automation.

## License

No project license is declared yet. Add a `LICENSE` file before publishing or redistributing this template. Third-party projects retain their own licenses.
