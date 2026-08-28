# Application Rules

## Responsibility

This crate owns use cases, outbound Port traits, orchestration, and stable application error categories. Keep them under `use_cases/`, `ports/`, or `errors/`.

## Port contract

- Depend only on `domain` inside the workspace.
- Ports describe capabilities needed by use cases and expose no adapter/framework types.
- Do not expose HTTP, database, configuration, or downstream wire encodings through a Port. Convert them at the adapter that owns the encoding.
- Port methods must not provide default implementations that return `Unsupported`, `Unavailable`, or another placeholder error. An adapter either implements the declared interface or does not implement that Port.
- A Port exposes only the coherent capability required by its callers. Split unrelated capabilities instead of forcing adapters to pretend to support them, but do not create one-method traits for hypothetical substitution.
- When a use case promises atomicity or idempotency across related state changes, expose that consistency boundary as one coherent Port operation. Separate Port calls are valid only when partial completion is part of the defined semantics; orchestration alone cannot make independent adapter calls atomic. Never load or preflight a mutable Aggregate outside that Port operation and then treat it as authoritative inside the transaction.

## Use-case contract

- Application commands and results are explicit contracts. Do not return an Aggregate directly to an adapter when an Application-owned result should select the data approved for that boundary.
- A use-case name must describe all behavior it owns. Do not place queries, callbacks, renewals, retries, or revocations behind a create use case merely because they concern the same Entity.
- A type whose public operation only forwards the same-shaped command to one Port and returns its result owns no use case. Remove that wrapper or give it real orchestration, such as obtaining time from an independent Port before invoking one atomic persistence operation; never create pass-through command files merely to satisfy the capability-directory rule.
- When a mutation requires a database lock, the Infrastructure adapter reconstructs the Aggregate and invokes its Domain behavior inside the atomic Port operation. Do not duplicate or preflight that behavior in Application merely to make a short use case look substantial; Domain behavior runs exactly once at the correctness boundary that owns the required state.
- Construct Domain values before calling Ports. Treat an external acceptability check as a business Policy, not as Domain or transport validation, and short-circuit later calls on failure.
- Prefer generic static dispatch; do not add `async_trait`, trait objects, a DI container, or application-owned `Arc` for the current single composition path.
- Do not map HTTP status, log concrete failures, or catch errors that an outer boundary owns.

## Proof

- Orchestration changes are proved with inline fakes through the public use case.
