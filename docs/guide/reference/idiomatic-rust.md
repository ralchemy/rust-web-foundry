# Rust quality baseline

Generic Rust implementation and review guidance comes from the pinned [`leonardomso/rust-skills`](https://github.com/leonardomso/rust-skills) checkout described in [Reviewing changes](../reviewing.md). It covers ownership, errors, async/concurrency, API and type design, conversions, Serde, testing, documentation, observability, performance, project structure, lints, and common anti-patterns.

Normal implementation remains code-first and does not preload the generic rule index. A fresh review classifies the diff first, reads only matching rule files under `.scratch/rust-skills/rules/`, and consults the large upstream `SKILL.md` index only when the relevant rule family cannot be identified from the diff and filenames. Project overrides in `.agents/rust-skills-overrides.md` apply before generic advice.

Project-specific authority remains with:

- requested behavior and explicitly confirmed acceptance criteria;
- the Clean Architecture responsibility and dependency map;
- `Cargo.toml`, `Cargo.lock`, and the [selected stack](../stack.md);
- the HTTP, database, security, runtime, and testing production paths and gates.

Tests created during implementation are evidence unless the originating request or a confirmed decision explicitly makes them acceptance criteria.

A generic Rust recommendation must not introduce a parallel framework, dependency, module shape, telemetry stack, mocking tool, public API promise, or release profile without a concrete project requirement.
