# Development

Use the smallest public check that proves the current change:

- run the focused owning-crate test while editing;
- run `just check` before handoff; it does not require MySQL;
- run `just test` when an existing local MySQL should exercise every workspace test;
- run `just verify` for configuration, migrations, composition, lifecycle, or installed-route changes.

The complete local proof is:

```sh
just verify
```

It starts MySQL 8.4, runs formatting, Clippy, and all tests, applies migrations explicitly, starts the local TaskPolicy and production server, checks live/ready/Task behavior and trace propagation, sends SIGTERM, verifies a clean exit, and stops local processes without deleting the MySQL volume.

See [Testing](testing.md) for test placement, the real adapter path, SQLx offline limits, and the exact database contract of each command.

SQLx query metadata is committed under `.sqlx/`, and `.cargo/config.toml` keeps normal compilation offline even when `DATABASE_URL` exists. After changing a migration or query macro, start MySQL and run `just sqlx-prepare`.

Forward migrations use `YYYYMMDDNNN_information.sql`, such as `20260806001_drop-task-id-index.sql`. The date and three-digit daily sequence form SQLx's integer version; its required underscore separates that version from the description.

Generated CI runs `just ci` with MySQL 8.4 on the latest stable Rust selected by `rust-toolchain.toml`. It applies migrations before checking that `.sqlx/` matches the current schema and query macros. Template CI first generates a fresh service and then runs that generated CI path.

The [Guide authority hierarchy](README.md#authority) defines which surface owns each kind of fact or contract. Tracked `CONTEXT.md`, `docs/adr/`, Domain, and Guide documents are repository artifacts at their declared authority level. Ignored `.scratch/` files, local agent configuration, generated stage reports, and ignored `CONTEXT.md`, `docs/adr/`, or `docs/agents/` files are working material, not repository contracts; promote confirmed durable knowledge to its tracked owner instead of relying on an ignored artifact.

When behavior changes, update code/tests and user documentation together. Keep each mandatory rule with its single repository owner rather than copying it into a Skill or prompt.

## Design checkpoint

Before production implementation that adds or changes a public workflow, Domain behavior, Port, database schema or index, persistence contract, or external integration, record a short checkpoint in the active issue or plan:

- **Type map:** identify every value with distinct business meaning, an invariant, a finite set, a unit, a trust distinction, or a risk of same-primitive confusion, and name its Domain or Application owner. Review identity, state, authorization, routing, idempotency, validated input, time, and quantity where applicable.
- **Conversion seams:** state where raw HTTP, database, configuration, and downstream values become Domain or Application types and where they are serialized again. For every changed boundary, record source type, target type, mechanism, conversion owner, and failure owner.
- **Interface:** name the module that owns the workflow and the smallest interface callers need. Record why inseparable operations remain together when splitting them would add only forwarding glue.
- **Acceptance path:** name the public path and the smallest test that proves the behavior.

Documentation-only, message-only, and small expectation-only changes do not require this checkpoint. Update the checkpoint when evidence changes the design; do not repeat or re-read it mechanically at every task boundary. Before handoff, reconcile every named interface, module, conversion seam, and acceptance path with the actual code and executable evidence.

## Coherent implementation slices

Implement a confirmed multi-task design in dependency order, grouping interface-coupled work into the smallest coherent buildable slice. A cross-layer signature change may be edited across its owning layers before that slice compiles.

At each stable slice boundary, run the smallest focused check that can expose a broken contract. Do not require full acceptance or a conformance audit for every internal task, and do not add public abstractions, unused wiring, placeholders, or compatibility scaffolding merely to make an intermediate task appear complete.

Continue automatically after a stable boundary passes. Stop only when a material business decision is unresolved, required authority or external access is missing, or no remaining task can progress independently; report the exact blocker and the evidence already completed.

## Governance and documentation changes

Changing `AGENTS.md`, Guide authority, Context Pointer routing, or a generated governance check requires Governance or Documentation scope declared by the active issue or specification. That scope must provide an exact changed-path allowlist and map every rule move to its current owner and stable clause key before the first repository edit. Ordinary implementation treats these documents as read-only; if it discovers an owner gap or needs an undeclared path, stop and request the smallest scope decision instead of editing the contract opportunistically.

Review the complete diff of every changed owner. Account for each changed path against the allowlist and each moved clause against one complete canonical owner; a pointer, optional procedure, postmortem, or executable check is not a second prose owner. Preserve business, architecture, dependency, runtime, transport, persistence, and named-gate semantics unless the declared scope explicitly changes one of them.

### Standing-brief size review

This section is the canonical location for the frozen UTF-8 byte sizes of the six standing briefs and their root-plus-nearest-local totals. The values below are frozen after the six-file routing migration:

| Metric | Bytes |
|---|---:|
| `AGENTS.md` | 6,920 |
| `app/AGENTS.md` | 1,853 |
| `crates/domain/AGENTS.md` | 1,882 |
| `crates/application/AGENTS.md` | 3,201 |
| `crates/http/AGENTS.md` | 3,860 |
| `crates/infrastructure/AGENTS.md` | 2,833 |
| Six-file aggregate | 20,549 |
| `AGENTS.md` + `app/AGENTS.md` | 8,773 |
| `AGENTS.md` + `crates/domain/AGENTS.md` | 8,802 |
| `AGENTS.md` + `crates/application/AGENTS.md` | 10,121 |
| `AGENTS.md` + `crates/http/AGENTS.md` | 10,780 |
| `AGENTS.md` + `crates/infrastructure/AGENTS.md` | 9,753 |

A later change requires Governance review only when a frozen metric grows by both more than 5% and more than 256 bytes; the review explains why the added content must remain standing or why a Context Pointer cannot own it. The threshold is a review warning, not an automatic rejection. The root standing brief must remain at or below 10,000 bytes.

## AI-assisted workflow

Give an agent the smallest authoritative context for the change, independent of its orchestration framework:

1. read root and the nearest `AGENTS.md` for every touched path; do not assume descendant rules were loaded automatically;
2. trace the existing public path and read each matched canonical owner once;
3. use a repository Skill when supported and applicable, otherwise produce the same repository-defined outcome directly;
4. implement coherent slices and run the owning checks.

Do not paste the entire Guide into a prompt or copy its explanations into standing briefs.

## Dependency selection

The root manifest and lockfile are the exact dependency catalogue. A `[workspace.dependencies]` entry centralizes a version and features; it does not make that crate available until an owning member declares `dependency.workspace = true`.

Before adding a dependency:

1. confirm the standard library, Axum/Tokio/Tower, or an already installed crate does not own the behavior;
2. place it only in the crate that owns the boundary—framework and adapter types must not leak inward;
3. enable only features required by a compiled path;
4. verify compiler baseline, transitive features, license/security policy, and maintenance against the real project requirement;
5. add the smallest public test that fails if the dependency integration breaks.

Use `cargo tree -p <package>` and `cargo tree -e features -p <package>` when ownership or feature activation is unclear. Do not maintain a second version table in the Guide or choose from a broad ORM, authentication, background-job, gRPC, or observability catalogue before the corresponding capability exists.

## Boundary anti-patterns

These are symptoms of responsibility drift in this workspace:

- SQL, reqwest, or business decisions in an Axum handler;
- Axum, Tokio, SQLx, reqwest, Serde, fastrace, or Logforth types leaking into Application or Domain;
- database rows or wire DTOs reused as Domain entities;
- raw dependency errors, rejected values, credentials, or secrets exposed in responses or telemetry;
- liveness depending on MySQL or another external service;
- `serve` running migrations or holding a database transaction across an external request;
- `unwrap`/`expect` on request- or dependency-controlled paths;
- detached production tasks, unbounded channels, or retries without lifecycle and idempotency contracts;
- `common`, `utils`, pass-through wrappers, or a trait with one speculative implementation.

Fix the owning boundary rather than compensating at callers. A matched Guide chapter owns the conditional contract, standing briefs retain path protection, and source plus public tests own executable facts.
