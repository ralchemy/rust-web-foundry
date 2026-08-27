# Database

The database adapter owns SQL syntax, SQLx types, connection pools, migrations, row reconstruction, and concrete database failures. Application Ports express persistence capabilities without exposing those details inward.

## Pool and schema ownership

`app` creates the MySQL pool and injects clones of its handle into infrastructure adapters. SQLx owns the shared pool state; creating a new pool per repository or request would fragment limits and shutdown behavior. HTTP and Application never receive `MySqlPool`.

MySQL constraints are the final persistence boundary. The Task table keeps its ID length and binary collation, non-empty bounded title, and trimmed-title checks even though Domain constructors enforce the same business invariants. Domain protects in-process construction; the database also protects imports, scripts, older binaries, and future adapters.

## Checked queries

Use `query!` for statements and anonymous result records. Use `query_as!` when returned columns should populate an infrastructure-owned row type. Do not map SQL rows directly into Domain entities: reconstruct Domain values through their constructors so stored data cannot bypass current invariants. A read-only query may select only the needed columns and map its private typed row directly to an Application projection; it need not load an Aggregate or an unused large field.

The macros validate SQL, parameter count and types, and returned column shapes against a real MySQL schema. They do not make dynamic SQL safe by themselves. Values must still be passed as bound parameters. Table names, column names, sort directions, and other SQL syntax cannot be bound; select them from an explicit allowlist rather than accepting arbitrary input. Use runtime query construction only when the query shape is genuinely dynamic. Record the reason beside the construction and test its parameterization and result mapping.

Normal builds read committed metadata from `.sqlx/`. `.cargo/config.toml` sets `SQLX_OFFLINE=true`, so an ambient `DATABASE_URL` cannot unexpectedly turn compilation into a database operation. After changing a checked query or migration:

1. Start the development MySQL service.
2. Run `just sqlx-prepare`.
3. Review and commit the query and `.sqlx/` changes together.
4. Run the focused test and `just check`; use `just verify` for schema or composition changes.

`just sqlx-prepare` is a development command. It applies migrations with sqlx-cli so metadata can be generated even when the application does not yet compile against a changed schema. Production does not need sqlx-cli and continues to run the separate application-owned `migrate` command.

CI first compiles from committed metadata, applies migrations to MySQL, and then runs `cargo sqlx prepare --check`. This proves both sides of the contract: a clean checkout builds without a database, and the committed metadata still matches the current queries and schema.

## Migration lifecycle

Migrations are forward-only, embedded in the infrastructure crate, and named `YYYYMMDDNNN_information.sql`. The integer prefix is SQLx's version; the underscore separates it from the description. Never rewrite a migration that may have been applied outside the local workstation.

`serve` never migrates. A deployment runs `migrate` with schema-changing credentials before starting instances whose `DATABASE_URL` may be limited to application DML. Readiness checks connectivity, not schema currency; the deployment step owns migration success.

## Transaction boundaries

Use a transaction only when multiple database operations must commit or roll back as one local invariant. Keep it short, pass the transaction explicitly to the operations that participate, and never perform an external network call while holding database locks.

When an external result is required before a write, distinguish two cases:

- A read-only validation or policy call completes first; only a successful result permits the database transaction or write. The Task flow follows this order.
- A side-effecting remote operation cannot be made atomically consistent with MySQL by holding a local transaction open. Give the remote request an idempotency key and model the workflow explicitly with a reservation plus confirmation, compensation, an outbox, or a saga according to the business guarantee.

There is always a failure window between two independent systems unless both participate in a real distributed transaction. Do not hide that window behind a long-lived SQL transaction or a generic retry. Record enough durable state to resume or compensate, then make each repeated step idempotent.

## Failures and verification

Infrastructure logs only a safe database category or vendor code and converts concrete SQLx failures into stable Application Port errors. SQL text, values, URLs, credentials, and raw database messages never cross inward or reach public responses.

Unit tests prove Domain reconstruction and error classification where those contain logic. Real MySQL tests prove migrations, constraints, checked query behavior, and persisted values. A fake repository cannot prove a schema or SQL contract.
