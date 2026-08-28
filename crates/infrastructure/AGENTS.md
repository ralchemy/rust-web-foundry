# Infrastructure Adapter Rules

## Responsibility

This crate implements application Ports for MySQL and outbound HTTP. Keep SQLx code under `mysql/` and reqwest code under `outbound_http/`; repositories stay under `mysql/repositories/`.

Conditional context for this crate is selected through `docs/agents/context-routes.tsv` and the compiled Context Pack. Do not preload database or observability guidance merely because this crate is touched.

## Adapter contract

- Depend only on `application` and `domain` inside the workspace; never depend on `http`.
- Use MySQL 8.4 syntax, bound parameters, SQLx checked query macros, and embedded forward migrations.
- Commit refreshed `.sqlx` metadata with every query or schema change. Use `query!` for statements and `query_as!` when a returned row has an adapter-owned type.
- A fixed production query is not dynamic merely because it is passed to `sqlx::query` at runtime. Fixed statements must not use unchecked `query`, `query_as`, `MySqlRow`/`Row` extraction, or `SELECT *`; use checked macros, explicit columns, and private adapter row types.
- Keep database rows and downstream wire DTOs private to their adapter. Convert raw IDs, states, capabilities, timestamps, quantities, and validated values into Domain or Application types before returning through a Port.
- Convert private database rows with `TryFrom`; persisted data remains untrusted even after a checked query proves column shape. Reject invalid and non-canonical IDs, text, enums, quantities, and revisions before reconstructing Domain state.
- `query_as!` maps a named row without `FromRow`; do not derive `FromRow` merely to preserve an unchecked fixed-query escape hatch.
- When loading persisted state for a mutation, lock and reconstruct the named Domain owner from a private checked row inside the transaction, invoke its behavior exactly once there, then persist the resulting state. Never accept an Aggregate loaded or preflighted outside the transaction as the authoritative mutation input.
- Do not hide an oversized Port implementation behind helper modules. Narrow the Port at its Application owner first, then implement each coherent Port in its owning adapter module.
- Do not use startup migrations, `INSERT IGNORE`, upserts, or transactions for the one-write Task flow.
- Classify concrete failures here, log only safe categories, and return stable Port errors inward.
- Own client spans and W3C propagation; never record full URLs, Task Titles, payloads, SQL, credentials, or raw errors.

## Proof

- At handoff, account for every changed SQL statement by its checked macro, explicit selected columns, private row mapping when it returns data, and refreshed `.sqlx` metadata.
