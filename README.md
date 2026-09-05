# rust-web-foundry

A `cargo-generate` template for production-oriented Rust web services with explicit Clean Architecture boundaries.

The repository generates a runnable five-package Axum workspace with a canonical Task reference slice, MySQL persistence, an outbound TaskPolicy integration, health endpoints, observability, graceful shutdown, tests, and reproducible quality commands. The Task slice makes the architecture executable; it is not a universal task-management domain model.

## What it generates

```text
app/                         # executable host and composition root
├── src/                     # commands, configuration, wiring, lifecycle
├── examples/                # local external-service peers
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

The generated Task path includes `POST /api/v1/tasks` and `GET /api/v1/tasks/{task_id}`. It demonstrates independent wire, Application, Domain, and database types; explicit boundary conversion; checked SQLx queries; private adapter models; stable public errors; real composition tests; and process lifecycle behavior.

## Generate a service

Prerequisites:

- Rustup and stable Rust
- Docker with Compose v2
- `cargo-generate` 0.23.x
- `just` 1.58.x
- `sqlx-cli` 0.9.x for SQLx verification

```sh
cargo install cargo-generate --version 0.23.14 --locked
cargo install just --version 1.58.0 --locked
cargo install sqlx-cli --version 0.9.0 --locked --no-default-features --features rustls,mysql

cargo generate --path . --name my-service
cd my-service
just verify
```

`just verify` starts MySQL, runs the generated checks and tests, applies migrations explicitly, verifies SQLx metadata, drives the production server path, checks trace propagation, and proves graceful shutdown.

## Code-first development

Generated projects use a small root contract rather than preloading a compiled documentation bundle or implementation workflow. Feature work starts from:

1. the requested behavior;
2. the nearest complete production path and its tests;
3. Cargo manifests, migrations, Just recipes, and CI;
4. a focused failing test at the owning public seam.

The Guide remains available as cold documentation for concrete design questions and capabilities with no existing example. It is not automatically routed into ordinary implementation context.

## Rust review

Generated projects pin [`leonardomso/rust-skills`](https://github.com/leonardomso/rust-skills) to an exact commit for explicit, fresh review. The generic Rust rules are not copied into `AGENTS.md` and are not loaded during implementation.

The review keeps three axes separate:

- requested behavior and acceptance evidence;
- project Clean Architecture and selected stack;
- applicable Rust quality rules from the pinned Skill.

Project-specific overrides prevent generic advice from replacing the fastrace/Logforth telemetry stack, typed inner errors, inline test fakes, layered crate structure, or other installed choices without a concrete requirement.

```sh
bash scripts/install-rust-skills.sh
```

Then invoke `.agents/skills/review-rust-web/SKILL.md` in a fresh review session with the request and complete diff.

## Local commands

| Command | Purpose |
|---|---|
| `just infra-up` | Start local MySQL |
| `just infra-down` | Stop local MySQL |
| `just policy-stub` | Start the local TaskPolicy peer |
| `just migrate` | Apply embedded migrations explicitly |
| `just sqlx-prepare` | Apply development migrations and refresh `.sqlx` metadata |
| `just serve` | Start the production server path |
| `just test` | Run the workspace tests against MySQL |
| `just architecture` | Check dependency direction, inner-crate boundaries, and the small project/review contract |
| `just check` | Run architecture checks, formatting, Clippy, and database-free tests |
| `just ci` | Run checks and database-backed acceptance against existing services |
| `just verify` | Start dependencies and run the complete verification path |
| `just lifecycle` | Prove graceful drain and shutdown-timeout behavior |

The server never runs migrations automatically. Run the dedicated `migrate` command with schema credentials before starting `serve`.

## Selected stack

The generated manifests are the exact dependency authority. The reference service currently uses Axum/Tower, Tokio, SQLx with MySQL, Reqwest, Serde, validator/axum-valid, config/dotenvy/secrecy, fastrace/Logforth/OpenTelemetry, thiserror/anyhow, Chrono, and ULID. Generic Skill recommendations do not silently add parallel frameworks or replace these integrations.

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

The primary design reference is [`gruberb/bulletproof-rust-web`](https://github.com/gruberb/bulletproof-rust-web) and its published guide. This repository is an independent `cargo-generate` template inspired by selected architectural material from that project; it is not a fork or drop-in mirror.

[`leonardomso/rust-skills`](https://github.com/leonardomso/rust-skills) is an MIT-licensed, pinned review dependency. Its generic Rust guidance is applied only after this project's behavior, architecture, manifests, executable evidence, and override file.

[`tyrchen/rust-lib-template`](https://github.com/tyrchen/rust-lib-template) was consulted separately for template ergonomics and automation.

## License

No project license is declared yet. Add a `LICENSE` file before publishing or redistributing this template. Third-party projects retain their own licenses.
