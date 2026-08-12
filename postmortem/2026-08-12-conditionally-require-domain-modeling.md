# Conditionally require Domain modeling

## Decision

Require Domain modeling before changes that alter business meaning, while invoking only the smallest applicable Skill: discovery, strategic design, tactical design, or model review.

## Why

Generated services need business language, invariants, ownership, and type candidates to be explicit before implementation. Without a trigger in Project Rules, an agent can skip an optional Skill and produce CRUD-shaped or primitive-heavy code.

Running a complete staged DDD process for every change would add context, documents, and false decisions to adapter-only and corrective work. Confirmed Domain docs therefore satisfy discovery, working output stays in the active issue or plan, and only durable knowledge becomes Domain documentation.

`CONTEXT.md` remains a glossary. Stable capability semantics such as scope, ownership, invariants, states, behaviors, Domain facts, and disambiguating scenarios live in one lazily created capability document under `docs/domain/` or its bounded-context equivalent. This prevents the ignored `.scratch/` tree or code from becoming the only surviving record without creating one document per modeling stage.

DDD Skills stop at business semantics. Existing Project Rules remain authoritative for crate boundaries, Ports, adapters, persistence, transport, resilience, and runtime design.

## Rejected alternatives

- Keep DDD optional: too easy for implementation agents to skip before coding.
- Run all Domain Skills for every task: produces ceremony without better decisions.
- Import the upstream nine-Skill pipeline unchanged: duplicates responsibilities, uses arbitrary quotas, and mixes Domain design with infrastructure policy and OpenSpec artifacts.
