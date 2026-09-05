# Testing

Tests follow ownership boundaries instead of a generic test pyramid. Put each proof at the narrowest public seam that can fail when the promised behavior breaks, and use the real composition path only for contracts that cross crates.

## Evidence by boundary

| Surface | Location | What it proves |
|---|---|---|
| Domain invariants | beside `Task` and its Value Objects | canonical identity, validation, named state transitions, revision advancement, and rejected-transition state safety |
| Application orchestration | beside `CreateTask` | typed input → policy → persistence order and stable failure categories through small fake Ports |
| Default HTTP contract | `crates/http/src/routes/mod.rs` | health/readiness work without `reference-task` and Task routes are absent |
| Reference HTTP contract | `crates/http/src/routes/mod.rs` with `reference-task` | versioned Task paths, extractor rejection, exact public errors, fallbacks, and health mapping |
| Adapter construction | beside the concrete adapter | local configuration rules that do not require network or database I/O |
| Reference production composition | `app/tests/create_task.rs` | real `app::build`, MySQL, reqwest, local Policy server, persistence, propagation, and cross-boundary failures |

Do not test private helpers when the installed Router or use case can expose the same failure. Do not move every test into `app/tests/`: a cross-crate test is heavier and often hides which owner broke.

## Default and reference shapes

The generated project has two intentionally tested configurations.

The default feature set is the user-facing baseline. It requires database connectivity for readiness but does not register Task APIs, require TaskPolicy configuration, or expose a Task migrator.

The default-off `reference-task` feature enables the canonical reference slice. It exists to make architectural and DDD patterns executable for humans and AI; its business semantics are not requirements for a generated project's real domain.

The Task integration test is therefore feature-gated:

```sh
TEST_DATABASE_URL=mysql://app:app@127.0.0.1:3306/app \
  cargo test -p {{ project-name }} --test create_task --features reference-task --locked
```

## SQLx offline is a compile boundary

Committed `.sqlx/` metadata lets checked query macros compile without connecting to MySQL. It does not turn database behavior into an offline test and does not replace migrations, constraints, or real query execution.

Reference Task migrations live under `crates/infrastructure/migrations/reference-task/`. After changing a reference migration or checked query, start MySQL and run `just sqlx-prepare`. CI verifies refreshed metadata against the migrated reference schema. The default runtime does not embed that schema.

`check-idiomatic-rust.sh` is part of `just check`, so local and CI verification no longer rely on a workflow-only SQLx style step.

## Command boundaries

| Command | Contract |
|---|---|
| `just architecture` | Requires no MySQL; checks fixed workspace dependency direction, forbidden outer-framework dependencies, the objective multi-workflow threshold, and the small project/review contract. It does not judge semantic ownership or generic Rust style. |
| `just check` | Requires no MySQL; runs architecture and SQLx style checks, format, a no-default-features workspace check/tests, and all-feature Clippy/tests. It proves both the default and reference compile/test shapes. |
| `just test` | Requires existing MySQL; runs both no-default-features and all-feature workspace tests. |
| `just ci` | Assumes MySQL already exists; runs `check`, proves the default server has no Task API, runs the feature-gated reference integration and live smoke, verifies SQLx metadata and trace propagation, and runs lifecycle drain/timeout proof. |
| `just verify` | Starts local MySQL, delegates to `just ci`, and always stops Compose while preserving its named volume. |

Use the smallest focused test while editing, then run `just check`. Run `just verify` when configuration, migrations, lifecycle, composition, SQLx metadata, or the installed route graph changes. Generated CI uses the same public `just ci` gate.

## Review

Mechanical checks prove explicit executable facts. Fresh review still checks whether the requested behavior was understood correctly, Domain responsibility is meaningful rather than merely well-shaped, reference semantics were not copied into user requirements, and tests would fail for a materially wrong implementation.

The pinned `rust-skills` testing rules apply subject to `.agents/rust-skills-overrides.md`. Generic recommendations do not justify adding mockall, snapshot testing, property testing, testcontainers, or another framework without a demonstrated gap.

## Deliberately absent

The baseline adds no mockall, WireMock, testcontainers, snapshot, coverage, per-test database factory, generic fixture tree, or test-support crate. Add a tool only when repeated tests create a concrete setup problem that the existing inline fake Ports, local Axum server, and Compose MySQL cannot solve simply.
