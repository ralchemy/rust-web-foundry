# Idiomatic Rust code contract

> **Status:** Generated-service code contract
>
> **Authority:** When a Context Pointer matches, this chapter owns the conditional shared Rust code contract. Source, tests, Clippy, and architecture checks own executable facts; root and nearest-local rules retain standing scope responsibility and local hard protection.

This chapter defines the Rust vocabulary that the generated service and coding agents use by
default. It constrains durable code outcomes rather than depending on a particular AI agent,
prompting framework, or editor integration.

## Model semantic differences, not directory symmetry

Create a distinct Domain or Application type when a value has different business meaning, an invariant, a finite set, a unit, a trust distinction, a representation, a lifecycle, or a risk of same-primitive confusion. Swappable same-primitive arguments, directly constructible invalid values, or validation, string comparison, and unit conversion repeated by callers are evidence that the owning type is missing.

Free-form human text, descriptions, reasons, display-only values, and genuinely opaque external payloads may remain primitives when they have no independent invariant and do not participate in business decisions. Human text with a non-blank, bounded, normalized, or other business invariant follows the [Domain modeling contract](../domain/modeling.md) and is owned by a validating business type. Do not add a wrapper when a value has no independent meaning or invariant.

The canonical Task slice therefore has separate types for real boundaries:

| Type | Owner | Why it exists |
|---|---|---|
| `CreateTaskRequest`, `TaskPath`, `TaskResponse` | HTTP | public JSON and path contracts |
| `CreateTaskCommand`, `TaskView` | Application | use-case input and approved output |
| `Task`, IDs, validated text, enums, quantities, revision | Domain | business meaning and valid state |
| `TaskRow` | MySQL adapter | selected database representation |
| `PolicyRequestWire`, `PolicyResponseWire` | outbound HTTP adapter | downstream protocol |

A type is not required when the contract and semantics are already identical. Type safety should make invalid states and accidental substitutions difficult, not produce ceremonial wrappers. Do not copy a struct mechanically into HTTP, Application, Domain, Infrastructure, and database modules merely to make every layer look symmetric.

Keep HTTP DTOs, database rows, and downstream wire types private to their owning adapters. Convert raw transport, persistence, configuration, and downstream values at the adapter that owns that representation. Once a Domain or Application value exists, pass that type inward rather than reducing it to a primitive and reconstructing it later. Its owning type defines parsing, validation, and formatting; callers do not repeat those rules. Prefer enums and explicit transitions to boolean flag combinations and string comparisons.

## Use the standard conversion vocabulary

- Implement `FromStr` for a stable textual representation and `Display` for its canonical outward form. Call parsing through `str::parse` at the boundary that owns the raw text.
- Implement `TryFrom` for a fallible DTO, database-row, configuration, or downstream-wire
  conversion.
- Implement `From` only when conversion cannot fail and preserves the complete target contract.
- Implement `From` or `TryFrom`, not `Into` or `TryInto` directly; the standard library supplies the
  reciprocal caller-side traits.
- Use `AsRef` only for a genuine cheap borrowed view that improves generic APIs. Do not replace
  every clear accessor with a trait solely for stylistic symmetry.
- Keep creation, reconstitution, and state transitions as named Domain operations. They establish
  business semantics and are not generic representation conversions.
- Do not add public `to_domain`, `into_domain`, `from_row`, or `to_response` methods when a standard
  conversion trait expresses the same operation.

Use this matrix when recording changed conversion seams in the [design checkpoint](../development.md#design-checkpoint):

| Source | Target | Mechanism | Owner | Failure owner |
|---|---|---|---|---|
| HTTP DTO or path text | Application command or Domain value | `TryFrom` / `FromStr` | HTTP | HTTP boundary |
| Application policy input | downstream request wire | `From` | outbound adapter | infallible |
| downstream response wire | Application decision | `TryFrom` | outbound adapter | downstream adapter |
| database row | Domain entity or Application projection | `TryFrom` | database adapter | database adapter |
| Application result | HTTP response | `From` | HTTP | infallible |

## Preserve ownership and borrowing

- Accept `&str` rather than `&String`, `&[T]` rather than `&Vec<T>`, and `&T` rather than `&Box<T>`.
- Borrow when reading. Consume a value when storing it or transferring ownership across a real
  boundary.
- Do not clone merely to silence the borrow checker. A material clone should correspond to an
  explicit ownership, persistence, queueing, or serialization boundary.
- Do not expose `Arc`, locks, pools, framework state, SQLx types, reqwest types, or concrete adapters
  inward.
- Keep Domain and Application fields private unless a public carrier type deliberately forms a
  coherent command, result, or reconstruction contract.

## Name code predictably

- Accessors omit `get_`: use `task.title()`.
- Use `as_*` for borrowed views, `to_*` for allocating conversions, and `into_*` for consuming
  extraction.
- Use `new` for an unsurprising primary construction. Use semantic names such as `create`,
  `reconstitute`, `parse`, `connect`, or `open` when behavior differs.
- Name business types with Domain nouns and operations with Domain verb phrases. Name booleans as predicates and collections with plural nouns.
- Do not use placeholder names such as `data`, `info`, `obj`, `tmp`, `x`, `v`, `r`, or `a` for business values. Conventional short names are acceptable only when their meaning is obvious in a tiny local scope.
- Do not let CRUD or framework vocabulary replace the ubiquitous language.

## Keep control flow readable

- Use `?` for error propagation and early return or `let else` for guard conditions.
- Keep orchestration at one level of abstraction.
- Use an explicit `match` when branches represent business decisions or stable failure categories.
- Use iterator, `Option`, and `Result` combinators for mechanical transformations such as
  `map(...).transpose()`, but do not hide a Domain workflow in a dense expression.
- Extract a function or module when it owns a coherent responsibility, invariant, conversion, or
  side effect—not merely because a file crossed an arbitrary line count.
- Prefer code that explains its intent through types and names over comments that narrate mechanics.

## Keep errors at their owning boundary

- Domain errors describe invalid Domain state or rejected Domain behavior.
- Application errors describe stable use-case and Port failure categories.
- Infrastructure owns concrete SQLx, reqwest, wire, and row-conversion causes.
- HTTP alone owns status codes and public response bodies.
- Never use strings as inward error categories and never expose raw adapter errors publicly.

## SQLx contract

- Use `query!` for checked statements and anonymous records.
- Use `query_as!` for a checked returned result mapped to a private named row.
- Use `query_scalar!` for a checked scalar query.
- List every selected or returned column explicitly. Never use `SELECT *` in production code.
- Never use `MySqlRow`, `Row::get`, or `Row::try_get` for a fixed production query.
- Do not derive `FromRow` merely to give a `query_as!` row an unchecked alternative; the macro maps
  by field name without that derive.
- Convert a database row with `TryFrom`. Compile-time SQL shape checking does not make persisted
  strings, IDs, enums, quantities, or cross-field state valid Domain data.
- Use `fetch_optional` when absence is a normal lookup result, then use
  `row.map(Target::try_from).transpose()` to preserve both absence and conversion failure.
- Commit refreshed `.sqlx` metadata with every query or migration change.

## Verification

The Task create and lookup paths are executable examples of this contract. Architecture checks also
reject `SELECT *`, row-get extraction, and unchecked fixed `query_as` use in Infrastructure. A
review still verifies the semantic choices that grep cannot prove: whether a new type is meaningful,
which conversion owns failure, and whether a named Domain operation is more appropriate than a
standard conversion trait.

## References

- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [`From`](https://doc.rust-lang.org/std/convert/trait.From.html),
  [`TryFrom`](https://doc.rust-lang.org/std/convert/trait.TryFrom.html),
  [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html), and
  [`AsRef`](https://doc.rust-lang.org/std/convert/trait.AsRef.html)
- [SQLx `query_as!`](https://docs.rs/sqlx/latest/sqlx/macro.query_as.html)
- [Rust Design Patterns](https://rust-unofficial.github.io/patterns/)
- [Bulletproof Rust Web domain modeling](https://github.com/gruberb/bulletproof-rust-web/blob/main/book/src/domain-modeling.md)
