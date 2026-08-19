# Application Rules

This crate owns use cases, outbound Port traits, orchestration, and stable application error categories. Keep them under `use_cases/`, `ports/`, or `errors/`.

Before adding or changing commands, results, conversion ownership, Port value types, or orchestration style, read `docs/guide/reference/idiomatic-rust.md`.

- Depend only on `domain` inside the workspace.
- Ports describe capabilities needed by use cases and expose no adapter/framework types.
- Application commands, results, and Port methods use Domain or Application-owned types for every value with distinct business meaning, an invariant, a finite set, a unit, a trust distinction, or a risk of confusion with another value using the same primitive. Identity, state, authorization, routing, idempotency, validated input, time, and quantity are examples, not an exhaustive list. Free-form text and opaque external payloads that do not participate in business decisions may remain primitives; constrained text is a validated business type.
- Do not expose HTTP, database, configuration, or downstream wire encodings through a Port. Convert them at the adapter that owns the encoding.
- Application commands and results are explicit contracts. Do not return an Aggregate directly to an adapter when an Application-owned result should select the data approved for that boundary.
- Port decisions use named enums or business types rather than ambiguous booleans.
- Port methods must not provide default implementations that return `Unsupported`, `Unavailable`, or another placeholder error. An adapter either implements the declared interface or does not implement that Port.
- A Port exposes only the coherent capability required by its callers. Split unrelated capabilities instead of forcing adapters to pretend to support them, but do not create one-method traits for hypothetical substitution.
- When a use case promises atomicity or idempotency across related state changes, expose that consistency boundary as one coherent Port operation. Separate Port calls are valid only when partial completion is part of the defined semantics; orchestration alone cannot make independent adapter calls atomic. Never load or preflight a mutable Aggregate outside that Port operation and then treat it as authoritative inside the transaction.
- A use-case name must describe all behavior it owns. Do not place queries, callbacks, renewals, retries, or revocations behind a create use case merely because they concern the same Entity.
- When a confirmed capability has two or more independently named or independently testable command workflows, use one capability directory with each complete command in a private child module. Do this from the start when workflows are planned together. Keep operations or a coherent query group together only when they are inseparable or child modules would add navigation and pass-through glue without clearer ownership; record why.
- A command module owns its input, result, orchestration, and focused tests. Keep small related queries or operations together when separate modules would only forward to the same Port.
- A type whose public operation only forwards the same-shaped command to one Port and returns its result owns no use case. Remove that wrapper or give it real orchestration, such as obtaining time from an independent Port before invoking one atomic persistence operation; never create pass-through command files merely to satisfy the capability-directory rule. Domain behavior runs exactly once at the correctness boundary that owns the required state: when a mutation requires a database lock, the Infrastructure adapter reconstructs the Aggregate and invokes its Domain behavior inside the atomic Port operation. Do not duplicate or preflight that behavior in Application merely to make a short use case look substantial.
- Construct Domain values before calling Ports. Treat an external acceptability check as a business Policy, not as Domain or transport validation, and short-circuit later calls on failure.
- Before adding or changing a Port or its dispatch strategy, read `docs/guide/architecture/ports-and-adapters.md`.
- Prefer generic static dispatch; do not add `async_trait`, trait objects, a DI container, or application-owned `Arc` for the current single composition path.
- Do not map HTTP status, log concrete failures, or catch errors that an outer boundary owns.
- Test orchestration with inline fakes through the public use case.
