# Database

The database adapter owns SQL syntax, SQLx types, connection pools, migrations, row reconstruction, and concrete database failures. Application Ports express persistence capabilities without exposing those details inward.

## Pool and schema ownership

`app` creates the MySQL pool and injects clones of its handle into Infrastructure adapters. SQLx owns shared pool state; do not create a pool per repository or request. HTTP and Application never receive `MySqlPool`.

Database constraints are the final persistence boundary. Domain protects in-process construction; schema constraints also protect imports, older binaries, scripts, and future adapters.

## Checked queries

Use `query!` for statements and anonymous records and `query_as!` for Infrastructure-owned row types. Do not map database rows directly into Domain entities when stored values must satisfy current invariants; reconstruct through Domain constructors/reconstitution and classify invalid stored data as persistence corruption before it crosses a Port.

Checked macros validate SQL/parameter/result shape against a schema. They do not make dynamic SQL syntax safe. Values stay bound parameters; request-selected identifiers, directions, or clauses require explicit allowlists.

Normal builds use committed `.sqlx/` metadata with `SQLX_OFFLINE=true`. After changing a checked query or migration, refresh metadata against the intended development schema and review the query/migration/metadata changes together.

## Migration lifecycle

Migrations are forward-only and owned by Infrastructure. Do not rewrite a migration that may already have been applied outside a disposable local environment.

`serve` never migrates. Deployment runs the separate `migrate` command with schema-changing credentials before starting instances whose runtime database account may be limited to application DML. Readiness checks connectivity, not schema currency.

## Transactions and cross-system consistency

Use a transaction when several database operations must commit or roll back as one local invariant. Keep it short and explicit. Do not hold database locks while calling an external network service.

When a business invariant requires a concurrency-sensitive mutation, the Application Port must be strong enough to express the atomic guarantee. A detached `find` followed by an unrelated `save` is not proof of compare-and-set behavior.

MySQL and an HTTP peer do not share an atomic transaction. For side-effecting remote work, define idempotency, compensation/reconciliation, or a durable workflow such as reservation/confirmation or outbox based on the requested guarantee. Do not hide the failure window behind a long database transaction or generic retry.

## Failures and verification

Infrastructure classifies concrete SQLx failures and returns stable Application categories. SQL text, values, URLs, credentials, and raw database messages do not cross inward or reach public responses.

Unit tests prove row reconstruction and classification logic where present; real MySQL tests prove migrations, constraints, checked-query behavior, and atomic persistence semantics.

Run focused evidence while editing. Finish with `just verify` for migration, SQLx metadata, or production database-composition changes; it already includes `just check`. For database-free Infrastructure changes, finish with `just check`.
