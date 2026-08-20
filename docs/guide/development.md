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

Each kind of knowledge has one authority:

| Surface | Owns | Does not own |
|---|---|---|
| Source, tests, manifests, migrations, Just, and CI | Executable structure, behavior, and named quality-gate proof | Unimplemented product intent or semantic review |
| [`CONTEXT.md`](../../CONTEXT.md) and on-demand Domain capability documents | Confirmed Domain language and durable capability semantics | Implementation details, working assumptions, or agent procedure |
| Guide Baseline | Installed behavior, current technical contracts, rationale, and extension seams | Uninstalled options or project history |
| Guide Reference | Conditional options, trade-offs, and safety boundaries | Claims that an optional capability is installed |
| README | Positioning, first use, and a short public-command summary | Complete contribution, architecture, Domain, or acceptance policy |
| `CONTRIBUTING.md` | Human entry point, change navigation, minimum evidence path, and authority links | Copies of the Guide, Project Rules, or testing policy |
| Root and nested `AGENTS.md` | Agent reading order, triggers, local routing, stop conditions, and evidence handoff | Product or engineering knowledge unavailable to humans |
| [`docs/agents/domain.md`](../agents/domain.md) | Framework-neutral Domain modeling outcomes and where to preserve them | Concrete business facts or Skill-specific procedure |
| `.agents/skills/` | Optional procedures for recurring agent-supported work | New rules, uniquely mandatory knowledge, or a stable public interface |
| `postmortem/` | Immutable historical reasons, trade-offs, and failed approaches | Current specification |
| Wayfinder maps and tickets | Decision routes and target designs before implementation | Current generated-service behavior or product documentation |

When behavior changes, update its authority, public summaries or links, relevant tests, and the named gate together. Do not copy a rule into a Skill or turn optional Guide advice into an always-on requirement.

## AI-assisted workflow

Give an agent the smallest authoritative context for the change, independent of its orchestration framework:

1. read root and the nearest `AGENTS.md` for every touched path plus `CONTEXT.md`; do not assume descendant rules were loaded automatically;
2. trace the existing public path and read its Baseline chapter;
3. follow a trigger into Development Reference only when the change creates that concern;
4. use a repository Skill when the active agent supports it and the work matches; otherwise follow the same documented outcome directly;
5. implement the smallest coherent buildable slice and run the owning check; produce the evidence-backed rule-conformance review once at handoff.

For a confirmed multi-task design, follow dependencies while grouping interface-coupled tasks into the smallest buildable slice. Run focused checks at stable boundaries and the complete required checks at handoff. Continue automatically; ask the user only for a material unresolved business decision, missing authority, or external access that blocks all remaining independent work.

Do not paste the entire Guide into a prompt or copy its explanations into Project Rules. When an agent repeats a mistake, place the correction with its single owner: executable behavior in code/tests, a universal constraint in the applicable `AGENTS.md`, an optional repeated procedure in a Skill, or conditional rationale in Reference. No mandatory rule may live only in a framework-specific prompt or Skill, and a framework's completion signal is not evidence that Project Rules passed. This keeps future context precise without turning one conversation into hidden project policy.

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

Fix the owning boundary rather than compensating at callers. The linked Baseline or Reference chapter explains the trade-off; Project Rules and public tests define the required result.
