# rust-web-foundry

A `cargo-generate` template for building production-oriented Rust web services with explicit Clean Architecture boundaries.

The repository is an independent template project. It generates a runnable five-package Axum workspace with a canonical Task reference slice, MySQL persistence, an outbound TaskPolicy integration, health endpoints, observability, graceful shutdown, tests, and reproducible quality commands.

> **Positioning:** rust-web-foundry is currently a runnable reference-service template. The Task slice is included by default to make the architecture executable and verifiable; it is not intended to define a universal task-management domain model.

## What it generates

```text
app/                         # executable host and composition root
├── src/                     # commands, configuration, wiring, lifecycle
├── examples/                # local external-service stubs
└── tests/                   # production-path acceptance tests
crates/
├── domain/                  # entities, value objects, invariants
├── application/             # use cases, Ports, application errors
├── http/                    # Axum routes, DTOs, extractors, API errors
└── infrastructure/          # SQLx/MySQL and reqwest adapters
```

Dependencies point inward:

```text
http ────────────┐
                 ├──> application ───> domain
infrastructure ─┘

app ──> http + infrastructure + application + domain
```

The generated Task path is deliberately small: `POST /api/v1/tasks` validates and normalizes a title, calls an external TaskPolicy Port, persists one MySQL row, and returns a fixed public response/error contract.

## Generate a service

Prerequisites:

- Rustup and a stable Rust toolchain
- Docker with Compose v2
- [`cargo-generate`](https://github.com/cargo-generate/cargo-generate) `0.23.x`
- [`just`](https://github.com/casey/just) `1.58.x`
- [`sqlx-cli`](https://github.com/launchbadge/sqlx) `0.9.x` for SQLx verification

Install the template tools:

```sh
cargo install cargo-generate --version 0.23.14 --locked
cargo install just --version 1.58.0 --locked
cargo install sqlx-cli --version 0.9.0 --locked --no-default-features --features rustls,mysql
```

Generate and verify a service:

```sh
cargo generate --path . --name my-service
cd my-service
just verify
```

`just verify` starts MySQL with Docker Compose and runs the generated project's formatting, Clippy, tests, migration, SQLx metadata verification, live HTTP smoke checks, trace propagation checks, and lifecycle acceptance checks.

## Local commands

Run these commands from a generated project:

| Command | Purpose |
|---|---|
| `just infra-up` | Start local MySQL |
| `just infra-down` | Stop local MySQL |
| `just policy-stub` | Start the local TaskPolicy stub |
| `just migrate` | Apply embedded migrations explicitly |
| `just sqlx-prepare` | Apply development migrations and refresh `.sqlx` metadata |
| `just serve` | Start the production server path |
| `just test` | Run the workspace tests against MySQL |
| `just check` | Run formatting, Clippy, and database-free tests |
| `just ci` | Run checks and database-backed acceptance steps against existing services |
| `just verify` | Start dependencies and run the complete verification path |

The server does not run migrations automatically. Use the dedicated `migrate` command with migration credentials before starting `serve`.

## Documentation

- [Template usage and positioning](TEMPLATE.md)
- [Generated-service README template](README.md.liquid)
- [Guide](docs/guide/README.md)
- [Architecture](docs/guide/architecture/README.md)
- [Task flow](docs/guide/task-flow.md)
- [Testing and quality gates](docs/guide/testing.md)
- [Observability](docs/guide/observability.md)
- [Outbound HTTP](docs/guide/reference/outbound-http.md)
- [Project Rules](AGENTS.md)
- [Template validation evidence](.scratch/template-validation/issues/06-validation-report-and-handoff.md)

## Referenced projects and attribution

This template is an independent implementation. The following projects are referenced because they provide either the template-generation toolchain, runtime libraries, or design material. They are not bundled copies of one another.

### Primary design reference: Bulletproof Rust Web

The most important upstream reference is [`gruberb/bulletproof-rust-web`](https://github.com/gruberb/bulletproof-rust-web), whose published guide is available at <https://gruberb.github.io/bulletproof-rust-web/>.

That repository is the **source of the original Bulletproof Rust Web guide and architectural subject matter** that informed this project. In particular, this project draws on its discussion of:

- Clean Architecture and dependency direction;
- Rust workspace and crate structure;
- domain modeling and validation;
- application Ports and adapters;
- Axum routing and thin handlers;
- error handling and public API contracts;
- SQLx/MySQL persistence;
- configuration and secret handling;
- testing, observability, graceful shutdown, and outbound I/O;
- AI-agent Project Rules, Skills, and development guidance.

`rust-web-foundry` is **not the upstream repository, not a fork, and not a drop-in mirror**. It is a separately designed `cargo-generate` template that reorganizes and operationalizes selected ideas from that guide into a generated five-package workspace. The generated code, Guide structure, Project Rules, Skills, validation harness, dependency choices, and runtime contracts in this repository should be treated as this project's own implementation and may intentionally differ from the upstream guide.

The historical name **Bulletproof Rust Web** is retained only in validation and migration records where it identifies the upstream source material or an earlier working name. The current product name is **rust-web-foundry**.

### Tooling and runtime dependencies

| Project | Role in rust-web-foundry | Link |
|---|---|---|
| [`cargo-generate`](https://github.com/cargo-generate/cargo-generate) | Renders this repository into a new project | [Documentation](https://cargo-generate.github.io/cargo-generate/) |
| [`Axum`](https://github.com/tokio-rs/axum) | HTTP server and routing | [Documentation](https://docs.rs/axum/) |
| [`Tokio`](https://github.com/tokio-rs/tokio) | Async runtime and process signals | [Documentation](https://docs.rs/tokio/) |
| [`SQLx`](https://github.com/launchbadge/sqlx) | MySQL pool, migrations, and SQL integration | [Documentation](https://docs.rs/sqlx/) |
| [`reqwest`](https://github.com/seanmonstar/reqwest) | Outbound TaskPolicy HTTP client | [Documentation](https://docs.rs/reqwest/) |
| [`fastrace`](https://github.com/fastn-stack/fastrace) | Trace spans and W3C trace context propagation | [Documentation](https://docs.rs/fastrace/) |
| [`fastrace-axum`](https://crates.io/crates/fastrace-axum) | Axum request trace integration | [Documentation](https://docs.rs/fastrace-axum/) |
| [`fastrace-reqwest`](https://crates.io/crates/fastrace-reqwest) | Outbound reqwest trace propagation | [Documentation](https://docs.rs/fastrace-reqwest/) |
| [`Logforth`](https://github.com/fastn-stack/logforth) | Structured process logging through the `log` facade | [Documentation](https://docs.rs/logforth/) |
| [`config`](https://github.com/mehcode/config-rs) | Environment-backed configuration loading | [Documentation](https://docs.rs/config/) |
| [`secrecy`](https://github.com/iqlusioninc/crates/tree/master/secrecy) | Secret-string handling for configuration | [Documentation](https://docs.rs/secrecy/) |
| [`just`](https://github.com/casey/just) | Reproducible developer and acceptance commands | [Documentation](https://just.systems/man/en/) |

### Additional design reference

[`tyrchen/rust-lib-template`](https://github.com/tyrchen/rust-lib-template) was consulted separately as a **read-only reference for template ergonomics and automation**. It is not the source of the Bulletproof Rust Web guide, and rust-web-foundry is not a fork, copy, or modification of it.

## Validation status

Fresh generated projects have been validated for template rendering, workspace compilation, formatting, Clippy, MySQL migration, SQLx verification, production-path Task behavior, trace propagation, and lifecycle behavior when the documented Docker/MySQL environment is available.

Database-backed checks require explicit `DATABASE_URL`, `MIGRATION_DATABASE_URL`, and `TEST_DATABASE_URL` values. AI/context findings in the validation material are limited to the tested model, prompts, repository state, and context protocol.

## License

No license is declared by this repository yet. Add a `LICENSE` file and update this section before publishing or redistributing the template.
