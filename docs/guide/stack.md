# Selected project stack

`Cargo.toml` and `Cargo.lock` are the exact dependency and feature catalogue. This page records the integration families that the generated service deliberately selects so reviews do not replace them with parallel frameworks merely because a generic Rust rule recommends another crate.

| Responsibility | Selected stack |
|---|---|
| HTTP server and middleware | Axum, Tower, `http-body-util` |
| Async runtime and lifecycle | Tokio, `tokio-util`, Futures |
| Transport data and validation | Serde, `serde_json`, validator, `axum-valid` |
| Persistence | SQLx with MySQL and checked offline metadata |
| Outbound HTTP | Reqwest with Rustls |
| Configuration and secrets | config, dotenvy, secrecy |
| Errors | typed errors in inner crates; anyhow-compatible boxed errors at the host boundary |
| Observability | fastrace, fastrace Axum/Reqwest integration, Logforth, the `log` facade, OpenTelemetry |
| Identity and time | ULID, Chrono |

## Dependency decisions

- Reuse the standard library or an installed dependency before adding another crate.
- Add a dependency only to the crate that owns the responsibility, declare its version/features in the workspace root, and commit the lockfile change.
- Do not introduce a second web framework, async runtime, database toolkit, HTTP client, validation stack, secrets abstraction, or telemetry pipeline without an explicit migration requirement.
- Enable the smallest features needed by compiled production and test paths.
- Generic `rust-skills` dependency, profile, mocking, logging, and module-layout recommendations are advisory. Existing integrations and `.agents/rust-skills-overrides.md` win.
- A deliberate stack migration changes code, tests, operational documentation, and verification together; it is not a local refactor.
