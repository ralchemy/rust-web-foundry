# Rust Web Foundry Project Template

This repository is an independent `cargo-generate` template for a runnable Axum workspace. The generated service uses five crates to enforce Clean Architecture dependencies and includes a canonical Task reference slice behind the default-off `reference-task` feature.

The default generated server contains the production host, health/readiness path, configuration, observability, database connection, lifecycle, and quality gates without installing Task routes, TaskPolicy configuration, or Task schema. The reference feature enables the complete Task path through HTTP, an outbound policy service, Domain behavior, and MySQL. Its business rules are illustrative; they are not a universal task-management domain model.

Generate a service from this checkout:

```sh
cargo generate --path . --name my-service
cd my-service
just verify
```

## Relationship to upstream references

The primary upstream reference for this project is [`gruberb/bulletproof-rust-web`](https://github.com/gruberb/bulletproof-rust-web), including its published guide at <https://gruberb.github.io/bulletproof-rust-web/>. This repository is an independent `cargo-generate` template inspired by and developed from selected ideas in that guide; it is not the upstream repository, not a fork, and not a drop-in mirror.

The generated workspace, Project Contract, Guide organization, dependency baseline, optional Task reference slice, runtime contracts, and validation harness are maintained here as this project's own implementation. Historical validation documents may use “Bulletproof Rust Web” when discussing the upstream guide or the former working name. The current product name is **rust-web-foundry**.

[`tyrchen/rust-lib-template`](https://github.com/tyrchen/rust-lib-template) was consulted separately only as a read-only reference for template ergonomics and automation. It is not the primary source of the upstream Rust Web guide.
