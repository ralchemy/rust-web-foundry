# Development

Use the smallest public check that proves the current change:

- run the focused owning-crate test while editing;
- run `just check` before handoff; it does not require MySQL;
- run `just test` when an existing local MySQL should exercise every workspace test;
- run `just verify` for configuration, migrations, composition, lifecycle, checked SQL, or installed-route changes.

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

## Compiled Context Pack

Conditional engineering context is compiled so implementation does not carry the discovery transcript.

Admission precedes routing. Inspect only standing authority and enough of the nearest public path to expose material decisions. If behavior remains undefined, return `needs-decision` before compiling conditional context or loading a later optional Skill.

`docs/agents/context-routes.tsv` is the single routing catalogue. Each row defines one stable action key, the concrete change shape that selects it, and one canonical `path[#anchor]` owner. Root and nearest-local `AGENTS.md` files contain standing protection only; they do not maintain a second routing table.

Before production implementation, classify the smallest planned touched paths and applicable action keys, then run:

```sh
bash scripts/compile-agent-context.sh \
  --goal 'short task goal' \
  --path crates/http/src/routes/mod.rs \
  --action http-routing \
  --action design-checkpoint
```

Use `--list-actions` to inspect the finite action vocabulary. The compiler adds root and nearest-local standing briefs from the planned paths, resolves every selected route once, extracts only a referenced Markdown section when an anchor is present, records each source as `path[#anchor]@content-sha`, and refuses a pack above its 60,000-byte default ceiling.

The default output is the immutable, content-addressed `.scratch/context-packs/<pack_id>.md`. Extend scope without replacing earlier evidence:

```sh
bash scripts/compile-agent-context.sh \
  --goal 'expanded task goal' \
  --extend-from .scratch/context-packs/<pack_id>.md \
  --path crates/application/src/use_cases/task.rs \
  --action ports-adapters
```

Before handoff, verify source freshness and final coverage. `--base` includes committed, staged, unstaged, and untracked paths; explicit `--path` remains available when a Git baseline is not appropriate:

```sh
bash scripts/compile-agent-context.sh \
  --verify-pack .scratch/context-packs/<pack_id>.md \
  --base <starting-head> \
  --action http-routing \
  --action ports-adapters
```

The legacy `.scratch/context-pack.md` remains untouched. Every Pack is a read view and cache ledger, not an authority. Never edit or promote its prose. If a source SHA changes or scope expands, extend or rebuild and read only newly selected or changed entries.

After reading the Pack, keep a short active checklist tied to touched paths and acceptance proof. It is working state, not a new authority; update it when scope changes and include it in the handoff.

Action classification is about the operation being performed, not topical similarity. Touching `crates/http` does not imply every HTTP action. A route-only correction may need `http-routing`; transport validation is added only if validation semantics change; security, persistence, async, and Domain actions are absent unless their described change shape is actually present.

If the compiler cannot be executed, follow the same deterministic protocol manually from `context-routes.tsv`: root + nearest-local standing briefs for every touched path + every selected routed source, with unique sources loaded once. Broad Guide reads are not a fallback.

### Evidence Pack and stage reset

Implementation handoff must be small enough to start a fresh reviewer without replaying the implementation session. Record only:

```yaml
evidence_pack:
  goal: <requested outcome>
  context_pack:
    id: <content identity>
    path: <stable artifact path>
  active_constraints: []
  changed_paths: []
  decisions: []
  public_acceptance_path: <test or executable path>
  checks:
    - command: <focused command>
      outcome: passed|failed
      log: <stable log path>
  review:
    batch_id: <id or none>
    snapshot_id: <id or none>
    reports_loaded: <2/2 or none>
    standards_report: <stable path or none>
    spec_report: <stable path or none>
    reviewed_diff: <identity or none>
    post_review_delta:
      id: <identity or none>
      reviewed: <true|false|not-applicable>
    findings: []
  unresolved: []
```

Keep complete successful logs outside the prompt when tooling permits. Carry command, exit status, the failing test or first root cause, and the smallest useful excerpt. A reviewed handoff retains every original finding with its status and owner. If fixes change the frozen diff, record the post-review delta and whether it received conformance review.

This stage reset is deliberate: exploration, failed attempts, compiler output, and repeated source reads are execution history, not durable review context.

### Frozen Review Batch

A dispatcher freezes the request, Context Pack, Evidence Pack, complete diff, changed-file and commit lists, and the evidence for each review axis. A frozen reviewer reads only those files; missing evidence produces an `incomplete` report instead of live-tree exploration or new test execution.

Completion notifications and status snippets are previews. Load the stable reports together:

```sh
bash scripts/load-review-reports.sh \
  <batch-id> <snapshot-id> <standards-report> <spec-report>
```

The loader validates distinct readable files, matching marker lines, and non-empty `## Review` bodies, then prints both reports. Finding classification, fixes, handoff, and completion require an untruncated command result whose final line is `reports_loaded: 2/2` for the expected batch and snapshot. A transport without stable full-report paths leaves the Batch incomplete.

## Design checkpoint

Before production implementation that adds or changes a public workflow, Domain behavior, Port, database schema or index, persistence contract, or external integration, record a short checkpoint in the active issue or plan:

- **Type map:** identify every value with distinct business meaning, an invariant, a finite set, a unit, a trust distinction, or a risk of same-primitive confusion, and name its Domain or Application owner. Review identity, state, authorization, routing, idempotency, validated input, time, and quantity where applicable.
- **Conversion seams:** state where raw HTTP, database, configuration, and downstream values become Domain or Application types and where they are serialized again. For every changed boundary, record source type, target type, mechanism, conversion owner, and failure owner.
- **Interface:** name the module that owns the workflow and the smallest interface callers need. Record why inseparable operations remain together when splitting them would add only forwarding glue.
- **Acceptance path:** name the public path and the smallest test that proves the behavior.

Before changing a shared interface or Port, enumerate every caller and implementation; the checkpoint is incomplete until each one is accounted for.

Documentation-only, message-only, and small expectation-only changes do not require this checkpoint. Update the checkpoint when evidence changes the design; do not repeat or re-read it mechanically at every task boundary. Before handoff, reconcile every named interface, module, conversion seam, and acceptance path with the actual code and executable evidence.

## Coherent implementation slices

Implement a confirmed multi-task design in dependency order, grouping interface-coupled work into the smallest coherent buildable slice. A cross-layer signature change may be edited across its owning layers before that slice compiles.

At each stable slice boundary, run the smallest focused check that can expose a broken contract. Do not require full acceptance or a conformance audit for every internal task, and do not add public abstractions, unused wiring, placeholders, or compatibility scaffolding merely to make an intermediate task appear complete.

Continue automatically after a stable boundary passes. Stop only when a material business decision is unresolved, required authority or external access is missing, or no remaining task can progress independently; report the exact blocker and the evidence already completed.

When one coherent slice would compile a Context Pack above the default 60,000-byte ceiling, split at the smallest stable dependency boundary. Raising the ceiling is a Governance decision, not an implementation convenience.

## Governance and documentation changes

Changing `AGENTS.md`, Guide authority, `docs/agents/context-routes.tsv`, routed-context budgets, the context compiler, or a generated governance check requires Governance or Documentation scope declared by the active issue or specification. The change must provide an exact changed-path allowlist and map every moved rule to one canonical owner before the first repository edit. Ordinary implementation treats these documents as read-only; if it discovers an owner gap or needs an undeclared path, stop and request the smallest scope decision instead of editing the contract opportunistically.

Review the complete diff of every changed owner. Account for each changed path against the allowlist and each moved clause against one complete canonical owner; a route row, generated Context Pack, optional procedure, postmortem, or executable check is not a second prose owner. Preserve business, architecture, dependency, runtime, transport, persistence, and named-gate semantics unless the declared scope explicitly changes one of them.

### Standing-brief size review

Standing briefs are intentionally limited to always-relevant protection. `scripts/check-agent-context.sh` rejects a root `AGENTS.md` above 6,000 UTF-8 bytes and rejects conditional `→ read` routing in any of the six standing briefs. Conditional routing belongs only in `docs/agents/context-routes.tsv`.

Do not grow a standing brief merely to improve explanation. Put task-conditional engineering contracts with their existing Guide or Domain owner, add or refine one route action, and keep rationale outside the hot path. A hard rule stays standing only when a routing miss would make ordinary work unsafe.

## Routed-context budget

Standing-brief size alone does not measure the context an agent actually loads. `docs/agents/routed-context-budgets.tsv` defines representative implementation shapes as unions of standing briefs and routed owner sections. `scripts/check-agent-context.sh` resolves each Markdown anchor, counts every unique source once, and rejects a scenario above its byte ceiling.

The budgets are regression ceilings, not instructions to preload every listed source. A real task still selects only actions that match its change. When a ceiling would grow, first shorten standing protection, narrow an owner anchor, remove a redundant route, or split a task. Raising a ceiling requires Governance scope.

The same check validates that action keys are unique, every routed source and anchor resolves, large conditional owners are always anchored, standing briefs contain no legacy route pointers, and every repository Skill consumes the Context Pack without directly naming conditional Guide owners. `just architecture` runs this check without MySQL.

## AI-assisted workflow

Use stage-local context rather than one ever-growing conversation:

1. **Admit:** inspect the nearest public path and resolve material decisions; stop at `needs-decision` when required.
2. **Route:** classify planned paths and action keys; compile the bounded Context Pack and active checklist.
3. **Implement:** work in coherent slices and extend the Pack only when scope expands.
4. **Handoff:** verify final Pack coverage and emit the compact Evidence Pack; keep full logs and failed-attempt transcripts out.
5. **Review:** freeze evidence, run evidence-only review, and load every full report before classifying findings.

Load an optional Skill only when execution reaches its explicit run, use, or invoke step. Use a repository Skill when supported and applicable, otherwise produce the same repository-defined outcome directly. A subagent receives the task goal, current Context Set, relevant evidence, and the smallest source slice required for its assignment, not the parent conversation.

Do not paste the entire Guide into a prompt, copy its explanations into standing briefs, or treat a larger model context window as the task budget.

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

Fix the owning boundary rather than compensating at callers. A routed Guide chapter owns its conditional contract, standing briefs retain path protection, and source plus public tests own executable facts.
