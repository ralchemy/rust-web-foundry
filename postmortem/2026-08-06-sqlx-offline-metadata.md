# SQLx offline metadata replaces runtime-only queries

## Decision

The generated service uses SQLx checked query macros and commits workspace metadata under `.sqlx/`. Normal Cargo builds force offline mode; the developer and CI workflows refresh or verify metadata against migrated MySQL 8.4.

## Why the earlier choice was wrong

The runtime-only query kept tooling minimal, but it discarded SQLx's compile-time schema and type checks. That is the wrong trade-off for a reusable template whose example should teach a safe default and whose clean checkout must compile without a live database.

SQL injection protection still comes from bound parameters and allowlisted dynamic syntax, not from the macro name. The metadata workflow adds schema validation without changing that security boundary.

## Operational boundary

sqlx-cli is development and CI tooling only. Production migrations remain an explicit `app migrate` operation, and `serve` never changes schema.
