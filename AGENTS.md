# Project Rules

## Dependency direction

Source dependencies point inward:

```text
http ────────────┐
                 ├──> application ───> domain
infrastructure ──┘

app ──> http + infrastructure + application + domain
```

`app` is the only composition root. `http` and `infrastructure` are sibling adapters and must never depend on each other.

Keep the four architecture crates under `crates/`. Keep the executable host at root `app/`; it is a process boundary, not another architecture layer. Do not introduce `apps/` until a second executable host exists.

Before moving a crate, adding a top-level source directory, or introducing a shared crate, read `docs/guide/architecture/project-structure.md`.
Before changing a workspace dependency edge or moving a responsibility between crates, read `docs/guide/architecture/README.md`.
Before adding an error category that crosses a crate boundary or changing a public error mapping, read `docs/guide/architecture/error-handling.md`.
Before adding authentication, authorization, sessions, bearer credentials, passwords, or identity-provider integration, read `docs/guide/security.md` and `docs/guide/reference/authentication-and-authorization.md`.
Before accepting a new untrusted input or destination, handling another secret, or changing network exposure, read `docs/guide/security.md`.
Before changing test placement, public Just quality commands, or cross-crate acceptance evidence, read `docs/guide/testing.md`.
Before adding or replacing a dependency, read `docs/guide/development.md#dependency-selection`.
Before performance tuning, caching, compression, streaming, pool changes, benchmarking, or profiling, read `docs/guide/reference/performance.md`.
Before adding `tokio::spawn`, `spawn_blocking`, `select!`, a timeout around mutating work, a channel loop, periodic work, a background job, or another long-lived subsystem, read `docs/guide/reference/async-and-cancellation.md`.
Before changing outbound HTTP timeout, retry, idempotency, redirect, or resilience policy, read `docs/guide/reference/outbound-http.md`.
Before adding protobuf, gRPC, Tonic, streaming RPCs, or another listener, read `docs/guide/reference/grpc.md` and `docs/guide/reference/async-and-cancellation.md`.
Before adding a Dockerfile, deployment manifest, image workflow, release migration sequence, or platform probe configuration, read `docs/guide/reference/deployment.md`.

## Ownership

- Put entities, value objects, invariants, and domain errors in `domain`.
- Put use cases, outbound Ports, and application error categories in `application`.
- Put Axum routes, DTOs, extractors, middleware, and public error translation in `http`.
- Put SQLx/MySQL and reqwest implementations of application Ports in `infrastructure`.
- Put configuration, concrete wiring, commands, logging, tracing setup, and process lifecycle in `app`.

Keep the responsibility directories already used in each crate. Do not flatten a crate or create cosmetic files. Do not create `common`, `shared`, `utils`, or `helpers`: move a business concept inward, model an external capability as a Port, keep adapter conversion at its boundary, or tolerate a few duplicated lines until a coherent owner exists.

## Boundaries

- Domain and application must not expose Axum, Tokio, SQLx, reqwest, serde, fastrace, or Logforth types.
- Only HTTP converts failures to status codes and the fixed public error envelope.
- Only infrastructure logs concrete SQLx or reqwest failures; return stable application categories inward.
- Never log or trace secrets, database URLs, Task Titles, bodies, headers, query strings, SQL text, or raw downstream errors.
- Use parameterized SQL. `serve` never runs migrations; `migrate` is the only production migration command.
- Add an abstraction only for a current second use or a real boundary. Prefer direct static dispatch and existing dependencies.
- Require a reproducible workload, metric, or profile before performance work. Do not preconfigure caching, compression, pool tuning, blocking pools, streaming, benchmarks, or profiling dependencies for hypothetical load.
- Treat dropping an incomplete Future as cancellation. Before racing or timing out mutating work, define its partial-side-effect, idempotency, retry, and public-error semantics; a timeout does not prove rollback.
- Do not detach production tasks. The lifecycle owner must retain and await each spawned task and propagate panic, early exit, cancellation, or timeout.

## Domain modeling workflow

Domain modeling is mandatory when a change alters business meaning, but the full workflow is not mandatory for every change. Read `docs/agents/domain.md` and use only the smallest applicable Skill:

- Use `ddd-discover` before implementation when adding a business capability or when its actors, terminology, events, rules, scope, or exceptional outcomes are not already confirmed in the Domain docs.
- Use `ddd-strategic-design` before adding or changing a bounded context, semantic owner, cross-context relationship, or translation boundary.
- Use `ddd-tactical-design` before adding or materially changing an Aggregate, invariant, state transition, Domain Service, Domain Event, or business type.
- Use `ddd-model-review` before declaring a substantial Domain model or Domain implementation complete.

Existing confirmed Domain docs may satisfy discovery; reuse them instead of regenerating artifacts. Adapter-only, Infrastructure-only, documentation-only, and small corrective changes that preserve business meaning do not require the full DDD workflow. Do not implement a business rule that depends on an unresolved material Domain question.

Keep unresolved and change-local modeling in the active issue or plan. Before declaring substantial Domain work complete, persist confirmed, durable capability semantics in the location defined by `docs/agents/domain.md`; the ignored `.scratch/` tree must not be their only record. Update the semantic document in the same change when its scope, invariants, states, behaviors, or Domain facts change.

DDD Skills own business understanding: facts, language, capabilities, boundaries, invariants, behaviors, and type candidates. Project Rules own code organization, dependency direction, Ports, adapters, persistence, transport, resilience, and runtime design. A DDD output cannot override these Project Rules or mandate a technical pattern.

## Code quality contract

### Design checkpoint

Before implementing a change that adds or changes a public workflow, Domain behavior, Port, persistence, or an external integration, record a short design checkpoint in the active issue or plan:

- **Type map**: identify every value with distinct business meaning, an invariant, a finite set, a unit, a trust distinction, or a risk of confusion with another value using the same primitive. Include, but do not limit the review to, identity, state, authorization, routing, idempotency, validated input, time, and quantity; name the type that owns each value.
- **Conversion seams**: state where raw HTTP, database, configuration, and downstream values become Domain or Application types, and where they are serialized again.
- **Interface**: name the module that owns the workflow and the smallest interface its callers need.
- **Acceptance path**: name the public path and smallest test that prove the behavior.

Documentation-only, message-only, and small expectation-only changes do not need this checkpoint.

### Type-driven design

- Any value with distinct business meaning, an invariant, a finite set, a unit, a trust distinction, or a risk of confusion with another value using the same primitive must use a dedicated Domain or Application type at those interfaces. Examples include, but are not limited to, identity, state, authorization, routing, idempotency, validated input, time, and quantity.
- Treat swappable same-primitive arguments, directly constructible invalid values, or repeated validation, string comparison, and unit conversion in callers as evidence that a type is missing.
- Free-form human text, descriptions, reasons, display-only values, and genuinely opaque external payloads that do not participate in business decisions may remain primitives. Do not mechanically wrap values that have no independent business meaning or invariant.
- Convert raw transport, persistence, configuration, and downstream values at their owning adapter. Do not convert a typed value back into a primitive while passing it between Domain and Application.
- A value type owns its parsing, validation, and formatting rules. Do not repeat string validation, status matching, or unit conversion across callers.
- Prefer enums and explicit transitions to boolean flag combinations or string comparisons.

### Human-readable code

- Name types with Domain nouns, functions with Domain verb phrases, booleans as predicates, and collections with plural nouns.
- Do not use placeholder names such as `data`, `info`, `obj`, `tmp`, `x`, `v`, `r`, or `a` for business values. Conventional short names are acceptable only when their meaning is obvious in a tiny local scope.
- Keep a function at one level of abstraction so orchestration reads as a sequence of Domain operations rather than transport, persistence, and business details mixed together.
- Treat approximately 30 lines as a review trigger, not a quota. Extract only a coherent responsibility, repeated rule, external adapter, or lifecycle owner.
- Do not create pass-through helpers, accessors, wrappers, or modules that merely rename another call. File length alone is not a reason to split a module.
- Prefer a deep module with a small interface over several shallow modules whose interfaces expose their implementation steps.

### Module shape

- Start a responsibility in one file. Promote it to a directory module only when it contains multiple independently named workflows or stable responsibilities.
- For example, an Application `permission_requests.rs` that owns create, renew, and revoke workflows may become `permission_requests/{mod.rs,create.rs,renew.rs,revoke.rs}`; sibling crates do not mirror that shape unless their own responsibilities require it.
- Split by a complete workflow, Domain rule, external adapter, or lifecycle owner; do not split merely because a file is long or because CRUD verbs exist.
- Keep `mod.rs` as the module interface: declare private submodules, expose only the types callers need, and keep implementation details private.
- Move the complete responsibility into an extracted file, including its behavior and focused tests. Do not leave the behavior in `mod.rs` and create pass-through wrappers in child modules.
- Keep small related operations together when separate files would contain only forwarding code.
- Do not mechanically mirror the same directory or command structure across Domain, Application, HTTP, and Infrastructure. Each crate splits by the responsibility it owns.

## Completion

Drive behavior through public paths. Add the smallest test that would fail if the behavior broke.

Before declaring a change complete:

1. Read the full diff.
2. Inspect every changed public struct, enum, trait, and function signature for primitive representations of control-flow values.
3. Trace one real public path from HTTP through Application to its adapter and confirm conversion occurs only at the owning seams.
4. Remove unused code, placeholder implementations, pass-through wrappers, duplicated conversions, and unnecessary lint allowances introduced or exposed by the change.
5. Confirm names describe Domain meaning rather than implementation position.
6. Run `just check`. For changes to configuration, migrations, lifecycle, or the installed route graph, run `just verify`.
7. Report any deliberate exception to these rules and explain why it remains.

## Agent skills

### Issue tracker

Issues are tracked as local Markdown files under `.scratch/`. See `docs/agents/issue-tracker.md`.

### Triage labels

Use the default canonical labels: `needs-triage`, `needs-info`, `ready-for-agent`, `ready-for-human`, and `wontfix`. See `docs/agents/triage-labels.md`.

### Domain docs

This is a single-context repository using root `CONTEXT.md` and `docs/adr/`. See `docs/agents/domain.md`.
