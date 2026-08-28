# Domain Rules

## Responsibility

This crate owns Task entities, value objects, invariants, and domain errors. Keep code under `entities/`, `value_objects/`, or `errors/`; add a directory only for a new stable domain responsibility.

Conditional context for this crate is selected through `docs/agents/context-routes.tsv` and the compiled Context Pack. Do not preload Domain modeling or typestate guidance merely because this crate is touched.

## Model contract

- Depend on no workspace crate.
- Keep construction invariant-preserving and fields private.
- Do not silence interface-design warnings such as `too_many_arguments` to make an unwieldy constructor pass. Use a named intent or reconstitution input only when those values form one coherent concept; otherwise simplify the interface.
- Keep each invariant authoritative in its owning constructor or behavior; adapter validation never replaces Domain construction.
- When a design assigns an invariant or state transition to an Entity, Aggregate, Domain Service, or Policy, every production mutation governed by that rule invokes that owner. A Domain model used only by its own tests, while Application or Infrastructure implements the real transition, does not satisfy that ownership.
- Express state changes through Entity behavior and enums rather than public field mutation, boolean flag combinations, or string comparisons.
- Prefer behavior on its owning Entity or Value Object. Add `services/` only for a real pure rule spanning multiple Domain concepts; Domain Services never perform I/O or call Ports.
- Do not add async I/O, Ports, use cases, serialization, runtime, HTTP, database, logging, or tracing types.
- Generate Task IDs here with ULID; do not add an ID-generator Port until a real requirement needs one.

## Proof

- Test invariants beside the owning type.
