# Development

## Code-first workflow

The generated service does not require a compiled documentation bundle or an implementation workflow Skill. Work from the requested behavior and the nearest complete production slice:

1. Read enough source and tests to trace the affected public path.
2. Identify behavior that is genuinely new. If a material public or business rule is absent from the request and executable evidence, leave it unresolved instead of inventing a default.
3. Add the smallest failing test at the owner that can prove the behavior.
4. Implement one coherent buildable slice, following the existing layer and conversion patterns unless the requirement establishes a new responsibility.
5. Run the focused test, then `just check`; add `just verify` for SQLx metadata, migrations, installed routes/composition, configuration, or runtime behavior.

Code, tests, manifests, migrations, Just recipes, and CI own executable facts. The Guide explains the current design and conditional alternatives, but ordinary work reads a chapter only when a concrete question is not answered by the production path.

## Architecture and dependencies

The root contract defines the responsibility map: Domain owns business types and invariants; Application owns use cases and Ports; HTTP and Infrastructure are inward-facing adapters; `app` owns composition, configuration, and process lifecycle. `just architecture` enforces the objective dependency graph and project-contract shape. Semantic responsibility remains part of code review.

The root manifest and lockfile are the exact dependency catalogue. Follow the [selected stack](stack.md), use workspace dependency inheritance, and add a crate only when the standard library and installed stack cannot satisfy the requirement. A new dependency needs an owning crate, minimal features, tests, and a lockfile change.

## Checks

Use the smallest public check that proves the current change:

- run the focused owning-crate or Router test while editing;
- run `just check` before handoff; it does not require MySQL;
- run `just test` when an existing local MySQL should exercise every workspace test;
- run `just verify` for the complete local production path.

`just verify` starts MySQL 8.4, runs formatting, Clippy, architecture and database-contract checks, executes all tests, applies migrations explicitly, verifies SQLx metadata, starts the local TaskPolicy peer and production server, checks health/Task behavior and trace propagation, verifies shutdown, and stops local processes without deleting the MySQL volume.

SQLx query metadata is committed under `.sqlx/`, and `.cargo/config.toml` keeps normal compilation offline. After changing a migration or query macro, start MySQL and run `just sqlx-prepare`.

Forward migrations use `YYYYMMDDNNN_information.sql`, such as `20260806001_drop-task-id-index.sql`. The date and three-digit daily sequence form SQLx's integer version; the underscore separates that version from the description.

Generated CI runs `just ci` with MySQL 8.4 on stable Rust. Template CI first generates a fresh service and then runs that generated path.

## Review

Do not preload the generic Rust rules during implementation. After implementation, start a fresh review session and use the project review Skill described in [Reviewing changes](reviewing.md). It installs and applies the pinned `leonardomso/rust-skills` baseline progressively, after the requested behavior, project architecture, selected stack, and project overrides.

Keep complete successful command logs and full diffs outside the conversation when tooling permits. A useful handoff contains the goal, changed paths, material decisions, commands and outcomes, unresolved behavior, and stable paths to detailed evidence—not the exploration transcript.
