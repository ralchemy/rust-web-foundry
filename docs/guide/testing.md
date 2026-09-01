# Testing

Tests follow ownership boundaries instead of a generic test pyramid. Put each proof at the narrowest public seam that can fail when the promised behavior breaks, and use the real composition path only for contracts that cross crates.

## Evidence by boundary

| Surface | Location | What it proves |
|---|---|---|
| Domain invariants | beside [`Task`](../../crates/domain/src/entities/task.rs) and [`TaskTitle`](../../crates/domain/src/value_objects/task_title.rs) | canonical identity, normalization, length, and rejected characters |
| Application orchestration | beside [`CreateTask`](../../crates/application/src/use_cases/task/create.rs) | typed input → policy → persistence order and stable failure categories through small fake Ports |
| HTTP contract | [`routes`](../../crates/http/src/routes/mod.rs) | the installed Router, versioned paths, extractor rejection, status, exact public error envelope, fallbacks, and health mapping |
| Adapter construction | beside the concrete adapter | local configuration rules that do not require network or database I/O |
| Production composition | [`app/tests/create_task.rs`](../../app/tests/create_task.rs) | real `app::build`, MySQL, reqwest, the local Policy server, persistence, propagation, and cross-boundary failure behavior |

Do not test private helpers when the installed Router or use case can expose the same failure. Do not move every test into `app/tests/`: a cross-crate test is heavier and often hides which owner broke.

Assert exact public status and JSON through the installed Router. Use the real composition path only for cross-crate adapter contracts. While editing, run the smallest owning test, then `just check`; add `just verify` when configuration, migrations, lifecycle, composition, or the installed route graph changes.

## Public HTTP contract

HTTP tests send requests through the Router returned by `http::router`. The error table asserts the complete JSON value, not only its status or error code:

```json
{"error":{"code":"task_title_invalid","message":"task title is invalid"}}
```

Exact equality also proves that internal details and extra envelope fields are absent. Handler-only tests would miss nesting, middleware, body limits, and 404/405 fallbacks, so the public Router is the correct seam.

## Real adapter path

The template keeps one sequential Task integration test. It:

1. applies the embedded production Migrator to `TEST_DATABASE_URL`;
2. starts a small Axum TaskPolicy server on an ephemeral loopback port;
3. calls the production [`app::build`](../../app/src/lib.rs) path, which constructs the real SQLx and reqwest adapters;
4. drives `POST /api/v1/tasks` through the installed Router and queries MySQL for the persisted row;
5. covers rejection, malformed response, downstream `5xx`, a stopped Policy server, and invalid Domain input, then proves only the successful request wrote a row;
6. verifies outbound W3C trace propagation without contacting a public service.

The local server is a controllable peer, not a mocked reqwest client. This keeps HTTP serialization, connection errors, and adapter classification in the tested path without adding a mock framework or public-network dependency.

Running the integration test directly requires an explicit database URL:

```sh
TEST_DATABASE_URL=mysql://app:app@127.0.0.1:3306/app   cargo test -p {{ project-name }} --test create_task --all-features --locked
```

## SQLx offline is a compile boundary

Committed `.sqlx/` metadata lets checked query macros compile without connecting to MySQL. It does not turn database behavior into an offline test and does not replace migrations, constraints, or real query execution.

After changing a migration or checked query, start MySQL and run `just sqlx-prepare`. CI verifies the refreshed metadata against migrated MySQL. Bound parameters remain the SQL injection boundary; checked macros add schema and type verification.

## Command boundaries

| Command | Database contract |
|---|---|
| `just architecture` | Requires no running MySQL; checks the fixed workspace dependency direction, forbidden outer-framework dependencies in Domain and Application, the objective multi-workflow threshold for top-level use-case files, and the small code-first project/review contract. It does not attempt to judge semantic ownership or generic Rust style. |
| `just check` | Requires no running MySQL; runs `architecture`, format, Clippy, all DB-free unit/Router tests, and app library/binary tests. Clippy may compile the integration target but does not execute it. |
| `just test` | Runs every workspace test and requires an existing MySQL at `TEST_DATABASE_URL` or the documented local default. |
| `just ci` | Assumes MySQL already exists; runs `check`, the real integration test, explicit migration, SQLx metadata verification, live HTTP smoke, propagation, and graceful shutdown. |
| `just verify` | Starts local MySQL, delegates to `just ci`, and always stops Compose while preserving its named volume. |

Use the smallest focused test while editing, then run `just check`. Run `just verify` when configuration, migrations, lifecycle, composition, or the installed route graph changes. Generated CI uses `just ci` with a MySQL 8.4 service, so local and remote acceptance share the same behavior gate.

## Rust review

Test placement and coverage topology are project facts. A fresh review additionally applies the pinned `rust-skills` testing rules where relevant, subject to `.agents/rust-skills-overrides.md`. Generic recommendations do not justify adding mockall, snapshot testing, property testing, testcontainers, or another test framework without a demonstrated gap.

## Deliberately absent

The baseline adds no mockall, WireMock, testcontainers, snapshot, coverage, per-test database factory, generic fixture tree, or test-support crate. Add a tool only when repeated tests create a concrete setup problem that the existing inline fake Ports, local Axum server, and Compose MySQL cannot solve simply.
