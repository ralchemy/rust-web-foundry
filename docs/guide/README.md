# Guide

The Baseline explains decisions embodied by the generated service. Development Reference chapters preserve optional knowledge for capabilities that are not installed by default.

## Authority

1. The requested behavior and acceptance tests own feature semantics.
2. Source, manifests, migrations, tests, Just recipes, and CI own executable facts about the installed service.
3. Root `AGENTS.md` owns only the small Clean Architecture, trust-boundary, selected-stack, and verification contract that cannot be inferred reliably from a single file.
4. This Guide explains the design and alternatives but is not automatically loaded for ordinary implementation.
5. The explicit fresh-review Skill applies the pinned `rust-skills` baseline after project authority and project overrides.

When sources disagree, stop at the concrete authority seam instead of choosing the most convenient prose. Do not infer new business behavior from an architecture example.

## Guiding principles

- Point dependencies toward business policy.
- Use types and constructors to preserve Domain invariants.
- Keep handlers and adapters thin, with their wire/database representations private.
- Touch only the layers required by the behavior.
- Add an abstraction or dependency only for a current responsibility.
- Treat the Task slice as executable architecture documentation, not a universal task-management model.

## Using this Guide

Start with the requested behavior, nearest production slice, and tests. Read a Guide chapter only when they expose a concrete unanswered design question, or when adding a capability with no existing example. Normal endpoint work does not need to preload the Guide.

## Baseline

- [Architecture](architecture/README.md): crate ownership, dependencies, and where shared behavior belongs.
- [Project structure](architecture/project-structure.md): workspace layout, visibility, file placement, and test location.
- [Ports and adapters](architecture/ports-and-adapters.md): Port ownership, Domain Services, dispatch choices, and failure translation.
- [Error handling](architecture/error-handling.md): typed failure ownership, cross-layer conversion, public contracts, and safe recording.
- [Domain modeling](domain/modeling.md): semantic types, invariant construction, serialization boundaries, and conversions.
- [Selected stack](stack.md): the preferred integration families and dependency decision rules.
- [Configuration](app/configuration.md): command-scoped settings, source precedence, validation, defaults, and secret exposure.
- [Observability](observability.md): Logforth logs, fastrace spans, exporter modes, propagation, redaction, and flush guarantees.
- [Security](security.md): implemented trust boundaries, explicit non-guarantees, local exposure, and conditional capabilities.
- [State management](http/state-management.md): composition ownership, focused `FromRef` substates, request context, cloning, and mutation.
- [Routing and handlers](http/routing-and-handlers.md): versioned routes, translation, extractor ordering, fallbacks, and installed-Router tests.
- [Validation](http/validation.md): transport checks, Domain construction, business decisions, `axum-valid`, and public errors.
- [Middleware](http/middleware.md): installed Layer order, fastrace context, body limits, error handling, and conditional extensions.
- [Database](infrastructure/database.md): MySQL ownership, compile-time queries, migrations, transactions, and external consistency.
- [Task flow](task-flow.md): the canonical CreateTask slice, transformations, boundary ownership, and verification.
- [Runtime](runtime.md): migrations, health, shutdown, and the platform-neutral deployment contract.
- [Testing](testing.md): boundary-owned tests, real adapters, SQLx offline limits, and quality gates.
- [Development](development.md): the code-first workflow, checks, dependencies, and handoff.
- [Reviewing](reviewing.md): fresh three-axis review with the pinned Rust baseline.

## Development reference

These chapters are on-demand knowledge, not installed features or default recommendations. See the [Reference index](reference/README.md).
