# Typestate

> **Status:** Development reference
>
> **Baseline:** The reference Task already has a runtime lifecycle (`pending -> in_progress -> completed`) implemented as an enum plus named Domain transitions. Its value objects and constructors prevent invalid values without generic state markers.
>
> **Read when:** A small, local protocol or Domain workflow has invalid calls that should be impossible to express at compile time rather than merely rejected at runtime.
>
> **Authority:** Domain modeling and boundary-conversion rules override generic examples here.

Use the cheapest type guarantee that fits:

| Problem | Prefer |
|---|---|
| one value must satisfy an invariant | private newtype plus validating constructor |
| persisted or externally supplied entity moves among runtime states | enum plus authoritative transition methods |
| a small local API has ordered compile-time phases | typestate, if invalid calls are materially dangerous |

Typestate represents a phase with a type parameter and exposes operations only for valid phases. It repays its cost when there are few stable states, transitions are mostly linear, invalid calls risk security or corruption, and the generic state does not spread through many crates.

It is a poor fit when persisted values must be reconstructed dynamically, multiple states share collections, the transition graph changes often, or callers mostly branch on runtime state. In those cases, keep the runtime enum and let one Domain operation own each transition.

Keep typestate local. Do not serialize marker types, expose state parameters through Application Ports, or create one marker type per ordinary enum variant solely to claim compile-time safety. Compare the call sites against the existing enum-and-transition shape first; adopt typestate only when compile-time exclusion materially simplifies or hardens the API.

If typestate is adopted, ordinary tests prove valid transitions. Add compile-fail infrastructure only when normal compilation of real call sites cannot protect the contract.
