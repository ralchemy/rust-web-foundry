# Application Host Rules

## Responsibility

This crate is the only composition root. It owns command parsing, settings, concrete adapter wiring, Logforth/fastrace initialization, listener startup, and ordered shutdown.

Conditional context for this crate is selected through `docs/agents/context-routes.tsv` and the compiled Context Pack. Do not preload configuration, observability, or runtime chapters merely because this crate is touched.

## Host and composition contract

- Keep `main.rs` as a thin call to `app::run`.
- Parse the command before command-specific settings. `migrate` requires only migration credentials; `serve` requires runtime credentials and never migrates.
- Keep raw deserialization and validated settings private and command-scoped. Add defaults only for safe operational choices; credentials and external endpoints remain explicit.
- Keep database URLs in `SecretString` until the concrete connection boundary and never log them.
- Construct configured use cases and adapters once in `build`. Keep process-lifecycle resources such as the pool in `BuiltService` rather than HTTP state so ordered shutdown retains ownership.
- When configuration selects a composition profile or adapter, `build` installs that actual implementation and the production-composition test proves the selection. If a configured production capability is unavailable, fail startup instead of silently substituting a controlled or test adapter.

## Proof

- Build the same installed Router for production and the cross-crate integration test; do not create a second test composition path or state bag.
