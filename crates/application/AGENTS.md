# Application Rules

This crate owns use cases, outbound Port traits, orchestration, and stable application error categories. Keep them under `use_cases/`, `ports/`, or `errors/`.

- Depend only on `domain` inside the workspace.
- Ports describe capabilities needed by use cases and expose no adapter/framework types.
- Application commands, results, and Port methods use Domain or Application-owned types for every value with distinct business meaning, an invariant, a finite set, a unit, a trust distinction, or a risk of confusion with another value using the same primitive. Identity, state, authorization, routing, idempotency, validated input, time, and quantity are examples, not an exhaustive list. Free-form text and opaque external payloads that do not participate in business decisions may remain primitives.
- Do not expose HTTP, database, configuration, or downstream wire encodings through a Port. Convert them at the adapter that owns the encoding.
- Port methods must not provide default implementations that return `Unsupported`, `Unavailable`, or another placeholder error. An adapter either implements the declared interface or does not implement that Port.
- A Port exposes only the coherent capability required by its callers. Split unrelated capabilities instead of forcing adapters to pretend to support them, but do not create one-method traits for hypothetical substitution.
- A use-case name must describe all behavior it owns. Do not place queries, callbacks, renewals, retries, or revocations behind a create use case merely because they concern the same Entity.
- When a use-case file owns multiple independently named or independently testable workflows, promote it to a directory module and move each complete command or coherent query group into a private child module.
- A command module owns its input, result, orchestration, and focused tests. Keep small related queries or operations together when separate modules would only forward to the same Port.
- Construct Domain values before calling Ports. Treat an external acceptability check as a business Policy, not as Domain or transport validation, and short-circuit later calls on failure.
- Before adding or changing a Port or its dispatch strategy, read `docs/guide/architecture/ports-and-adapters.md`.
- Prefer generic static dispatch; do not add `async_trait`, trait objects, a DI container, or application-owned `Arc` for the current single composition path.
- Do not map HTTP status, log concrete failures, or catch errors that an outer boundary owns.
- Test orchestration with inline fakes through the public use case.
