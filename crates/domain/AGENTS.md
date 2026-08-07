# Domain Rules

This crate owns Task entities, value objects, invariants, and domain errors. Keep code under `entities/`, `value_objects/`, or `errors/`; add a directory only for a new stable domain responsibility.

Before adding or changing an Entity, Value Object, invariant, or reconstruction path, read `docs/guide/domain/modeling.md`.
Before introducing generic state markers or compile-time state transitions, read `docs/guide/reference/typestate.md`.

- Depend on no workspace crate.
- Keep construction invariant-preserving and fields private.
- Keep each invariant authoritative in its owning constructor or behavior; adapter validation never replaces Domain construction.
- Give every named, semantically distinct, or easily confused Domain value its own type rather than representing Domain concepts with bare primitives.
- Prefer behavior on its owning Entity or Value Object. Add `services/` only for a real pure rule spanning multiple Domain concepts; Domain Services never perform I/O or call Ports.
- Do not add async I/O, Ports, use cases, serialization, runtime, HTTP, database, logging, or tracing types.
- Generate Task IDs here with ULID; do not add an ID-generator Port until a real requirement needs one.
- Test invariants beside the owning type.
