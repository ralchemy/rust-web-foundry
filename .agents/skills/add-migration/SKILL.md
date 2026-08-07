---
name: add-migration
description: Add and verify a forward MySQL migration in this Rust web workspace. Use when schema, indexes, constraints, or persisted Task data must change.
---

# Add a migration

1. Read root and infrastructure `AGENTS.md`, `docs/guide/infrastructure/database.md`, the current migrations, and every checked query affected by the schema change.
2. Add the next forward-only SQL file under `crates/infrastructure/migrations/` as `YYYYMMDDNNN_information.sql`, incrementing the three-digit sequence within that date. SQLx requires the underscore after the integer version. Do not rewrite an already released migration.
3. Keep MySQL 8.4 compatibility and review locks, table scans, constraint rollout, and rollback operations before choosing SQL.
4. Update checked queries and the real MySQL integration assertion in the same change.
5. Run `just sqlx-prepare` and commit the resulting `.sqlx` changes.
6. Run the focused test and `just verify` against MySQL. Production still uses the application-owned `migrate` command; never add migration-on-serve behavior.
