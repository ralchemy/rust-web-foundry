# Typestate

> **Status:** Development reference
>
> **Baseline:** Task has no lifecycle state. Its value objects and constructors already prevent invalid values without generic state markers.
>
> **Read when:** A small, local protocol or Domain workflow has invalid transitions that should be impossible to call, rather than merely rejected at runtime.
>
> **Authority:** Domain modeling and boundary-conversion rules override generic examples here.

Use the cheapest type guarantee that fits:

| Problem | Prefer |
|---|---|
| one value must satisfy an invariant | private newtype plus validating constructor |
| persisted or externally supplied entity moves among runtime states | enum plus one authoritative transition method |
| a small local API has ordered compile-time phases | typestate, if invalid calls are materially dangerous |

Typestate represents a phase with a type parameter and exposes operations only for valid phases. A transition consumes the old value and returns the next state, so an invalid call fails to compile. This can fit protocol handshakes, authenticated connection setup, or builders with genuinely mandatory phases.

## Keep the type-state machine local

Typestate repays its cost when there are few stable states, transitions are mostly linear, invalid calls risk security or corruption, and the generic state does not spread across many crates. It is a poor fit when values of several states must share a collection, state comes from a database or request at runtime, the transition graph changes often, or callers mostly branch on state.

Persist and transport an enum or explicit state value. Reconstruction must validate that stored data is internally consistent, then choose the appropriate runtime representation. Do not serialize `PhantomData`, expose generic state parameters through Application Ports, or create one marker type per ordinary enum variant solely to claim compile-time safety.

Before adopting typestate, compare the call sites with an enum whose transition method rejects invalid moves. If the enum is equally clear and the invalid state is already handled as a business outcome, keep it. If typestate is adopted, compile-fail examples should prove that invalid calls are unavailable and ordinary tests should prove valid transitions; add a compile-test dependency only when normal compilation cannot protect the public use sites.

See [Domain modeling](../domain/modeling.md) for private newtypes, invariant construction, persistence reconstruction, and boundary conversion.
