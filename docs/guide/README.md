# Guide

This Guide has two parts: the Baseline explains decisions embodied by the generated service, while Development Reference chapters preserve conditional Rust Web knowledge for extending it. Cargo, code, tests, CI, Project Rules, and Just recipes remain authoritative.

## Guiding principles

- Point dependencies toward business policy.
- Use types and constructors to preserve domain invariants.
- Keep handlers and adapters thin.
- Touch only the layers a behavior actually needs.
- Add an abstraction only for a current responsibility.
- Optimize code for predictable ownership, readability, and navigation.
- The generated service intentionally includes the canonical Task reference slice; it demonstrates the baseline boundaries rather than defining a universal task-management domain.

## How to use this Guide

Read the Baseline chapter for the responsibility being changed. Project Rules define required constraints, Skills define recurring procedures, and trigger lines in `AGENTS.md` point to a Development Reference only when the task creates that concern. The Guide is not a mandatory linear tutorial.

## Baseline

- [Architecture](architecture/README.md): crate ownership, dependencies, and where shared behavior belongs.
- [Project structure](architecture/project-structure.md): fixed workspace layout, visibility, file placement, and test location.
- [Ports and adapters](architecture/ports-and-adapters.md): Port ownership, Domain Services, dispatch choices, and failure translation.
- [Error handling](architecture/error-handling.md): typed failure ownership, cross-layer conversion, public contracts, and safe recording.
- [Domain modeling](domain/modeling.md): semantic types, invariant construction, serialization boundaries, and conversions.
- [Configuration](app/configuration.md): command-scoped settings, source precedence, validation, defaults, and secret exposure.
- [Observability](observability.md): Logforth logs, fastrace spans, exporter modes, propagation, sampling, redaction, and flush guarantees.
- [Security](security.md): implemented trust boundaries, explicit non-guarantees, local exposure, and conditional security capabilities.
- [State management](http/state-management.md): composition ownership, focused `FromRef` substates, request context, cloning, and mutation.
- [Routing and handlers](http/routing-and-handlers.md): versioned business routes, handler translation, extractor ordering, fallbacks, and installed-Router tests.
- [Validation](http/validation.md): transport checks, Domain invariant construction, business decisions, `axum-valid`, and public errors.
- [Middleware](http/middleware.md): installed Layer order, fastrace context, body limits, error handling, and conditional extension examples.
- [Database](infrastructure/database.md): MySQL ownership, compile-time queries, migrations, transactions, and external consistency.
- [Task flow](task-flow.md): the canonical CreateTask slice, composition, type transformations, boundary ownership, and verification.
- [Runtime](runtime.md): configuration, migrations, logging/tracing, health, shutdown, and the platform-neutral deployment contract.
- [Testing](testing.md): boundary-owned tests, real adapters, SQLx offline limits, and Just quality gates.
- [Development](development.md): local workflow, migrations, CI, Project Rules, and Skills.

## Development reference

These chapters are on-demand knowledge, not installed features or default recommendations. Start with the [Reference index](reference/README.md) for its selection and promotion rules.

- [Performance](reference/performance.md): measurement, blocking work, pools, queries, payload cost, and diagnostic tools.
- [Async and cancellation](reference/async-and-cancellation.md): dropped Futures, concurrency primitives, timeout semantics, task ownership, and locks.
- [Authentication and authorization](reference/authentication-and-authorization.md): credential selection, principal construction, business policy, revocation, and browser boundaries.
- [Outbound HTTP](reference/outbound-http.md): client ownership, time budgets, retry safety, idempotency, and cross-system consistency.
- [gRPC](reference/grpc.md): adapter placement, protobuf contracts, error mapping, streaming, and lifecycle.
- [Typestate](reference/typestate.md): selection boundaries for compile-time state transitions and simpler alternatives.
- [Deployment](reference/deployment.md): packaging targets, images, configuration, probes, migration rollout, and termination.
