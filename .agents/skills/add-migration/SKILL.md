---
name: add-migration
description: Add and verify a forward MySQL migration in this Rust web workspace. Use when schema, indexes, constraints, or persisted Task data must change.
---

# Add a migration

1. Read root and Infrastructure `AGENTS.md`. Match their applicable action-first Context Pointers as a union, record or reuse the active plan's Context Set, and load each canonical owner once. Do not unconditionally read whole Guide chapters outside the matched branches.
2. Inspect the current migrations and every checked query affected by the schema change. If the query or touched-path set expands, update the Context Set and route only newly matched owners.
3. Complete the matched design checkpoint before production edits. Add the next forward-only SQL file under `crates/infrastructure/migrations/` as `YYYYMMDDNNN_information.sql`, incrementing the three-digit sequence within that date. SQLx requires the underscore after the integer version. Do not rewrite an already released migration.
4. Keep MySQL 8.4 compatibility and review locks, table scans, constraint rollout, and rollback operations before choosing SQL.
5. Update checked queries and the real MySQL integration assertion in the same change.
6. Run `just sqlx-prepare` and commit the resulting `.sqlx` changes.
7. Run the focused test and `just verify` against MySQL. Production still uses the application-owned `migrate` command; never add migration-on-serve behavior. Treat Guide and Context Pointer owners as read-only unless the task explicitly declares Governance or Documentation scope.
