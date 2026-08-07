# Separate fast checks from integration acceptance

## Decision

`just check` runs formatting, Clippy, and all tests that need no live database. It excludes only the app integration target while still compiling all targets through Clippy.

`just test` runs the full workspace suite against an existing MySQL. `just ci` adds the real MySQL integration test, migration and SQLx metadata checks, live HTTP smoke, trace propagation, and shutdown. `just verify` starts local MySQL and delegates to `just ci`.

## Why

The previous `_check` executed `cargo test --workspace`, so the supposedly fast local gate waited for MySQL and failed with a pool timeout when Compose was stopped. SQLx offline metadata only removes a database connection from checked-query compilation; it cannot make a runtime integration test database-free.

Keeping one integration target gives the command boundary a precise seam: the quick gate remains useful during ordinary edits, while CI and one-command local acceptance still exercise production composition and real adapters.

## Rejected alternatives

- Silently skipping the integration test when `TEST_DATABASE_URL` is absent would allow the same command to prove different behavior on different machines.
- Replacing MySQL or reqwest with mocks would stop proving the generated production path.
- Adding another public recipe, feature flag, test-support crate, or test runner would duplicate the existing Just workflow without solving a new problem.
