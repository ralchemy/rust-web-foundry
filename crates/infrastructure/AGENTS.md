# Infrastructure Adapter Rules

This crate implements application Ports for MySQL and outbound HTTP. Keep SQLx code under `mysql/` and reqwest code under `outbound_http/`; repositories stay under `mysql/repositories/`.

Read `docs/guide/infrastructure/database.md` before changing a query, migration, transaction boundary, database row conversion, or persistence error mapping.
Read `docs/guide/observability.md` before changing client spans, outbound propagation, dependency logging, or telemetry redaction.
Read `docs/guide/reference/outbound-http.md` before changing client construction, timeouts, redirects, retries, idempotency, response limits, or downstream failure policy.

- Depend only on `application` and `domain` inside the workspace; never depend on `http`.
- Use MySQL 8.4 syntax, bound parameters, SQLx checked query macros, and embedded forward migrations.
- Name migrations `YYYYMMDDNNN_information.sql`, for example `20260806001_drop-task-id-index.sql`. The date and three-digit daily sequence form SQLx's integer version; the underscore is required by SQLx.
- Commit refreshed `.sqlx` metadata with every query or schema change. Use `query!` for statements and `query_as!` when a returned row has an adapter-owned type.
- Do not use startup migrations, `INSERT IGNORE`, upserts, or transactions for the one-write Task flow.
- Reuse one redirect-disabled reqwest client with a finite timeout and no retry.
- Classify concrete failures here, log only safe categories, and return stable Port errors inward.
- Own client spans and W3C propagation; never record full URLs, Task Titles, payloads, SQL, credentials, or raw errors.
