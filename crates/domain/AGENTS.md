# Domain Rules

This crate owns Task entities, value objects, invariants, and domain errors. Keep code under `entities/`, `value_objects/`, or `errors/`; add a directory only for a new stable domain responsibility.

Before adding or changing an Entity, Value Object, invariant, or reconstruction path, read `docs/guide/domain/modeling.md`.
Before introducing generic state markers or compile-time state transitions, read `docs/guide/reference/typestate.md`.

- Depend on no workspace crate.
- Keep construction invariant-preserving and fields private.
- Keep each invariant authoritative in its owning constructor or behavior; adapter validation never replaces Domain construction.
- Give every named, semantically distinct, or easily confused Domain value its own type rather than representing Domain concepts with bare primitives. IDs, finite states, capabilities, permissions, validated input, and time or quantity units that affect decisions are types; free-form text remains a string unless it has an invariant.
- Let each value type own parsing, validation, and formatting. Express state changes through Entity behavior and enums rather than public field mutation, boolean flag combinations, or string comparisons.
- Prefer behavior on its owning Entity or Value Object. Add `services/` only for a real pure rule spanning multiple Domain concepts; Domain Services never perform I/O or call Ports.
- When a Domain file contains multiple independently named rules or lifecycle responsibilities, promote it to a directory module and split by Domain concept, invariant, policy, or state transition. Do not organize Domain modules around CRUD commands.
- Keep the directory module root as the Entity or Value Object interface. Each child module owns a complete behavior and its invariant tests rather than a fragment of a long method.
- Do not add async I/O, Ports, use cases, serialization, runtime, HTTP, database, logging, or tracing types.
- Generate Task IDs here with ULID; do not add an ID-generator Port until a real requirement needs one.
- Test invariants beside the owning type.
