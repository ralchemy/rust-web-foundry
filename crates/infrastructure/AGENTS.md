# Infrastructure Adapter Rules

This crate implements application Ports for MySQL and outbound HTTP. Keep SQLx code under `mysql/` and reqwest code under `outbound_http/`; repositories stay under `mysql/repositories/`.

Read `docs/guide/infrastructure/database.md` before changing a query, migration, transaction boundary, database row conversion, or persistence error mapping.
Read `docs/guide/observability.md` before changing client spans, outbound propagation, dependency logging, or telemetry redaction.
Read `docs/guide/reference/outbound-http.md` before changing client construction, timeouts, redirects, retries, idempotency, response limits, or downstream failure policy.
Read `docs/guide/reference/idiomatic-rust.md` before changing row/wire conversions, checked query style, ownership, or naming conventions.

- Depend only on `application` and `domain` inside the workspace; never depend on `http`.
- Use MySQL 8.4 syntax, bound parameters, SQLx checked query macros, and embedded forward migrations.
- Name migrations `YYYYMMDDNNN_information.sql`, for example `20260806001_drop-task-id-index.sql`. The date and three-digit daily sequence form SQLx's integer version; the underscore is required by SQLx.
- Commit refreshed `.sqlx` metadata with every query or schema change. Use `query!` for statements and `query_as!` when a returned row has an adapter-owned type.
- A fixed production query is not dynamic merely because it is passed to `sqlx::query` at runtime. Fixed statements must not use unchecked `query`, `query_as`, `MySqlRow`/`Row` extraction, or `SELECT *`; use checked macros, explicit columns, and private adapter row types.
- Keep database rows and downstream wire DTOs private to their adapter. Convert raw IDs, states, capabilities, timestamps, quantities, and validated values into Domain or Application types before returning through a Port.
- Convert private database rows with `TryFrom`; persisted data remains untrusted even after a checked query proves column shape. Reject invalid and non-canonical IDs, text, enums, quantities, and revisions before reconstructing Domain state.
- Convert downstream wire responses with `TryFrom` whenever protocol values or field combinations can be invalid. Do not return raw wire booleans or strings through a Port when a named Application decision exists.
- `query_as!` maps a named row without `FromRow`; do not derive `FromRow` merely to preserve an unchecked fixed-query escape hatch.
- When loading persisted state for a mutation, lock and reconstruct the named Domain owner from a private checked row inside the transaction, invoke its behavior exactly once there, then persist the resulting state. Never accept an Aggregate loaded or preflighted outside the transaction as the authoritative mutation input. A read-only query may select only the needed columns and map its private typed row directly to an Application projection; it need not load an Aggregate or an unused large field.
- Use runtime query construction only when the query shape is genuinely dynamic. Record the reason beside the construction and test its parameterization and result mapping.
- Split Infrastructure modules by implemented Port, transaction or consistency responsibility, or external system; do not mirror every Application command mechanically.
- An extracted adapter module owns its complete SQL or HTTP interaction, private row or wire types, conversion, error classification, and focused tests.
- Keep an adapter `mod.rs` focused on private module declarations, selective exports, and construction. Do not leave SQL, wire mapping, or workflow behavior in it.
- Do not hide an oversized Port implementation behind helper modules. Narrow the Port at its Application owner first, then implement each coherent Port in its owning adapter module.
- Give shared row or wire types their own module only when multiple adapter operations genuinely share the same representation.
- Do not use startup migrations, `INSERT IGNORE`, upserts, or transactions for the one-write Task flow.
- Reuse one redirect-disabled reqwest client with a finite timeout and no retry.
- Classify concrete failures here, log only safe categories, and return stable Port errors inward.
- Own client spans and W3C propagation; never record full URLs, Task Titles, payloads, SQL, credentials, or raw errors.
- At handoff, account for every changed SQL statement by its checked macro, explicit selected columns, private row mapping when it returns data, and refreshed `.sqlx` metadata.
