# Rust quality baseline

Generic Rust implementation and review guidance comes from the pinned [`leonardomso/rust-skills`](https://github.com/leonardomso/rust-skills) checkout described in [Reviewing changes](../reviewing.md). It covers ownership, errors, async/concurrency, API and type design, conversions, Serde, testing, documentation, observability, performance, project structure, lints, and common anti-patterns.

Normal implementation remains code-first and does not preload that rule index. A fresh review reads `.scratch/rust-skills/SKILL.md`, selects only rules relevant to the diff, and applies `.agents/rust-skills-overrides.md`.

Project-specific authority remains with:

- the requested behavior and acceptance tests;
- the Clean Architecture responsibility and dependency map;
- `Cargo.toml`, `Cargo.lock`, and the [selected stack](../stack.md);
- the HTTP, database, security, runtime, and testing production paths and gates.

A generic Rust recommendation must not introduce a parallel framework, dependency, module shape, telemetry stack, mocking tool, public API promise, or release profile without a concrete project requirement.
