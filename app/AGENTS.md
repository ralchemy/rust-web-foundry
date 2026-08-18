# Application Host Rules

This crate is the only composition root. It owns command parsing, settings, concrete adapter wiring, Logforth/fastrace initialization, listener startup, and ordered shutdown.

Read `docs/guide/app/configuration.md` before changing commands, environment sources, settings types, defaults, validation, or secret exposure.
Read `docs/guide/observability.md` before changing Logforth/fastrace initialization, exporter selection, Resource attributes, or trace flushing.
Read `docs/guide/runtime.md` before changing migration/serve dispatch, startup order, health wiring, signal handling, or shutdown order.

- Keep `main.rs` as a thin call to `app::run`.
- Parse the command before command-specific settings. `migrate` requires only migration credentials; `serve` requires runtime credentials and never migrates.
- Keep raw deserialization and validated settings private and command-scoped. Add defaults only for safe operational choices; credentials and external endpoints remain explicit.
- A missing `.env` is valid, but any other load failure stops startup. Existing process variables take precedence over `.env`; do not add implicit environment-specific behavior.
- Keep database URLs in `SecretString` until the concrete connection boundary and never log them.
- Connect to MySQL before binding HTTP; do not add startup retries.
- Construct configured use cases and adapters once in `build`. Keep process-lifecycle resources such as the pool in `BuiltService` rather than HTTP state so ordered shutdown retains ownership.
- When configuration selects a composition profile or adapter, `build` installs that actual implementation and the production-composition test proves the selection. If a configured production capability is unavailable, fail startup instead of silently substituting a controlled or test adapter.
- On shutdown, stop accepts, bound request draining, close the pool, then flush fastrace. A drain timeout is an error.
- Build the same installed Router for production and the cross-crate integration test; do not create a second test composition path or state bag.
