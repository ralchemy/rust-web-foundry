# Development

## Code-first workflow

Generated services start from a small project contract and executable repository evidence rather than a preloaded documentation bundle or implementation workflow Skill.

1. Read enough source and tests to trace the affected public path.
2. Separate current executable facts from intended behavior. Requested behavior and explicitly confirmed decisions own new business and public-API semantics. If a material rule is absent, leave it unresolved instead of copying a reference-domain rule or inventing a default.
3. When a change introduces a new lifecycle, invariant, authorization decision, consistency boundary, or meaning for an existing term, record the smallest design decision needed before production edits. Ordinary changes inside an established model do not need a repeated DDD ceremony.
4. Add the smallest failing test at the owner that can prove the behavior.
5. Implement one coherent buildable slice, following the existing layer and conversion patterns unless the requirement establishes a new responsibility.
6. Run the focused test, then `just check`; add `just verify` for SQLx metadata, migrations, installed routes/composition, configuration, or runtime behavior.

Code, tests, manifests, migrations, Just recipes, and CI own current executable facts. The Guide explains the current design and conditional alternatives, but ordinary work reads a chapter only when a concrete question is not answered by the production path.

## Reference Task

The canonical Task slice is executable reference material and is disabled from the generated service by default. Enable `reference-task` only when running or validating the example. Its architecture, boundary conversions, tests, and Domain techniques are patterns to inspect; its Task-specific states and rules are not business requirements for a generated project.

The reference Domain intentionally includes named creation, reconstitution, and state-transition operations. Rejected transitions leave the entity unchanged. Add richer reference behavior only when it demonstrates a distinct engineering or modeling decision; do not grow the example through repetitive CRUD endpoints.

## Architecture and dependencies

The root contract defines the responsibility map: Domain owns business types, invariants, and state transitions; Application owns use cases and Ports; HTTP and Infrastructure are inward-facing adapters; `app` owns composition, configuration, and process lifecycle. `just architecture` enforces objective dependency facts. Semantic ownership remains review work.

The root manifest and lockfile are the exact dependency catalogue. Use workspace dependency inheritance and add a crate only when the standard library and installed stack cannot satisfy the requirement. A new dependency needs an owning crate, minimal features, tests, and a lockfile change.

## Checks

Use the smallest public check that proves the current change. `just check` validates both the default feature set and the full reference configuration without MySQL. `just verify` starts local dependencies and proves the reference production path, SQLx metadata, migrations, trace propagation, and lifecycle behavior. The default service must still build and test without `reference-task` and without TaskPolicy configuration.

SQLx query metadata is committed under `.sqlx/`. Reference Task migrations live under `crates/infrastructure/migrations/reference-task/`; the default migration set is intentionally empty of example schema. After changing a reference migration or checked query, run `just sqlx-prepare`.

Generated CI runs the same public gates used locally. Template CI additionally proves the freshly generated default shape; template-default assertions must not prevent a real generated project from later adding its own Skills or local agent instructions.

## Review

Do not preload the generic Rust rules during implementation. After implementation, start a fresh review session and use the project review Skill described in [Reviewing changes](reviewing.md). Review requested behavior, semantic ownership and project architecture, and applicable Rust quality as separate axes.

Keep complete successful command logs and full diffs outside the conversation when tooling permits. A useful handoff contains the goal, changed paths, material decisions, commands and outcomes, unresolved behavior, and stable paths to detailed evidence—not the exploration transcript.
