# Application Host Rules

## Responsibility

This crate is the only composition root. It owns command parsing, settings, concrete adapter wiring, Logforth/fastrace initialization, listener startup, and ordered shutdown.

## Host and composition contract

- Keep `main.rs` as a thin call to `app::run`.
- Parse the command before command-specific settings. `migrate` requires only migration credentials; `serve` requires runtime credentials and never migrates.
- Keep raw deserialization and validated settings private and command-scoped. Add defaults only for safe operational choices; credentials and external endpoints remain explicit.
- Keep database URLs in `SecretString` until the concrete connection boundary and never log them.
- Construct configured use cases and adapters once in `build`. Keep process-lifecycle resources such as the pool in `BuiltService` rather than HTTP state so ordered shutdown retains ownership.
- When configuration selects a composition profile or adapter, `build` installs that actual implementation and the production-composition test proves the selection. If a configured production capability is unavailable, fail startup instead of silently substituting a controlled or test adapter.

## Proof

- Composition changes are proved through the same installed Router used by production and the cross-crate integration test; one composition path owns both.
