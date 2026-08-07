# Application Rules

This crate owns use cases, outbound Port traits, orchestration, and stable application error categories. Keep them under `use_cases/`, `ports/`, or `errors/`.

- Depend only on `domain` inside the workspace.
- Ports describe capabilities needed by use cases and expose no adapter/framework types.
- Construct Domain values before calling Ports. Treat an external acceptability check as a business Policy, not as Domain or transport validation, and short-circuit later calls on failure.
- Before adding or changing a Port or its dispatch strategy, read `docs/guide/architecture/ports-and-adapters.md`.
- Prefer generic static dispatch; do not add `async_trait`, trait objects, a DI container, or application-owned `Arc` for the current single composition path.
- Do not map HTTP status, log concrete failures, or catch errors that an outer boundary owns.
- Test orchestration with inline fakes through the public use case.
