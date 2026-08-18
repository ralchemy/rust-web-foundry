# Project Rules

## Instruction discovery

Before modifying a crate or directory, read the nearest nested `AGENTS.md` in that path. Do not rely on the active agent runtime to discover descendant instruction files automatically.

Project Rules and the nearest nested `AGENTS.md` are authoritative for repository work. Optional Skills, extensions, orchestration frameworks, and their style defaults are subordinate. Resolve a conflict by following Project Rules; in particular, a preference for fewer files or fewer types cannot override a required responsibility split or a business type required below.

## Repository contract

Before the first production edit, every implementation workflow—whether built into the agent or supplied by an external Skill—must satisfy the applicable preconditions in **Domain modeling workflow** and **Design checkpoint** below. An external workflow's completed status never substitutes for repository evidence.

Keep each named repository gate performing the proof documented in `docs/guide/testing.md`. A change to `AGENTS.md`, a public Just command, CI, or an architecture or acceptance script is explicit governance scope: report the old proof, the new proof, and equivalent coverage separately. A gate changed by the current work is not completion evidence for that work until its full diff and retained proof have been reviewed.

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

Domain modeling outcomes are mandatory when a change alters business meaning, but the full workflow is not mandatory for every change. Read `docs/agents/domain.md` and satisfy only the smallest applicable outcome:

- Discover actors, terminology, events, rules, scope, and exceptional outcomes before implementing a new or unclear business capability.
- Establish semantic ownership and translation before adding or changing a bounded context, cross-context relationship, or translation boundary.
- Model Aggregates, invariants, state transitions, Domain Services, Domain Events, and dedicated business types before materially changing them.
- Review business language, ownership, invariants, state safety, type completeness, and boundary leakage before handing off a substantial Domain model or implementation.

Treat every material business fact that is absent from the user's request and confirmed Domain artifacts as unresolved, not as a conventional default. This includes actors and authorization, states and transitions, time limits, rejection behavior, idempotency semantics, and Domain facts. Before calling a fact confirmed, identify its source in the working design as the user's request or an authoritative Domain artifact; otherwise keep it as an open question or explicitly marked hypothesis. Do not infer enum members, ranges, defaults, temporal constraints, permissions, or exceptional behavior from a feature name or illustrative example. Treat each command contract independently: a field or rule confirmed for one operation does not apply to a sibling operation unless its own contract says so. Do not promote an unresolved fact to a capability document or encode it in production. Ask for the smallest missing decision and stop only the affected implementation when that decision changes the model, public behavior, or external contract.

When the active agent supports the repository's `ddd-*` Skills, use the smallest applicable Skill to produce that outcome. Otherwise follow the same inputs, outputs, ownership rules, and completion criteria in `docs/agents/domain.md` directly. No mandatory Domain rule may exist only in a Skill.

Complete every applicable pre-implementation Domain outcome and the design checkpoint before writing the production implementation. A focused failing test or isolated throwaway spike may come first only to resolve a named uncertainty; remove or replace it once the uncertainty is resolved. A capability document or review written after the implementation is chosen does not retroactively satisfy this ordering.

Existing confirmed Domain docs may satisfy discovery; reuse them instead of regenerating artifacts. Adapter-only, Infrastructure-only, documentation-only, and small corrective changes that preserve business meaning do not require the full DDD workflow. Do not implement a business rule that depends on an unresolved material Domain question.

Keep unresolved and change-local modeling in the active issue or plan. Before declaring substantial Domain work complete, persist confirmed, durable capability semantics in the location defined by `docs/agents/domain.md`; the ignored `.scratch/` tree must not be their only record. Update the semantic document in the same change when its scope, invariants, states, behaviors, or Domain facts change.

DDD workflow outputs own business understanding: facts, language, capabilities, boundaries, invariants, behaviors, and type candidates. Project Rules own code organization, dependency direction, Ports, adapters, persistence, transport, resilience, and runtime design. A DDD output cannot override these Project Rules or mandate a technical pattern.

## Code quality contract

### Design checkpoint

Before implementing a change that adds or changes a public workflow, Domain behavior, Port, persistence, or an external integration, record a short design checkpoint in the active issue or plan:

- **Type map**: identify every value with distinct business meaning, an invariant, a finite set, a unit, a trust distinction, or a risk of confusion with another value using the same primitive. Include, but do not limit the review to, identity, state, authorization, routing, idempotency, validated input, time, and quantity; name the type that owns each value.
- **Conversion seams**: state where raw HTTP, database, configuration, and downstream values become Domain or Application types, and where they are serialized again.
- **Interface**: name the module that owns the workflow and the smallest interface its callers need.
- **Acceptance path**: name the public path and smallest test that prove the behavior.

Documentation-only, message-only, and small expectation-only changes do not need this checkpoint.

The checkpoint must exist before the production implementation begins. Update it when evidence changes the design; do not repeat or re-read it mechanically at every task boundary. Before handoff, reconcile every named interface, module, and acceptance path with the actual code and executable evidence; an aspirational or stale name is not proof.

### Type-driven design

- Any value with distinct business meaning, an invariant, a finite set, a unit, a trust distinction, or a risk of confusion with another value using the same primitive must use a dedicated Domain or Application type at those interfaces. Examples include, but are not limited to, identity, state, authorization, routing, idempotency, validated input, time, and quantity.
- Treat swappable same-primitive arguments, directly constructible invalid values, or repeated validation, string comparison, and unit conversion in callers as evidence that a type is missing.
- Free-form human text, descriptions, reasons, display-only values, and genuinely opaque external payloads that do not participate in business decisions may remain primitives. Human text with an invariant, such as a non-blank or bounded reason, is validated business input and requires an owning type. Do not mechanically wrap values that have no independent business meaning or invariant.
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

- Start a responsibility in one file when it has one known workflow. Two or more related command workflows that can evolve or be tested independently belong in one capability directory from the start rather than separate top-level command files.
- Promote an existing responsibility file when it gains such a workflow. Keep operations together only when they form one inseparable workflow or splitting would add navigation and pass-through glue without clearer ownership; record that exception in the design checkpoint.
- For example, an Application `permission_requests.rs` that owns create, renew, and revoke workflows may become `permission_requests/{mod.rs,create.rs,renew.rs,revoke.rs}`; sibling crates do not mirror that shape unless their own responsibilities require it.
- Split by a complete workflow, Domain rule, external adapter, or lifecycle owner; do not split merely because a file is long or because CRUD verbs exist.
- Keep `mod.rs` as the module interface: declare private submodules, expose only the types callers need, and keep implementation details private.
- Move the complete responsibility into an extracted file, including its behavior and focused tests. Do not leave the behavior in `mod.rs` and create pass-through wrappers in child modules.
- Keep small related operations together when separate files would contain only forwarding code.
- Do not mechanically mirror the same directory or command structure across Domain, Application, HTTP, and Infrastructure. Each crate splits by the responsibility it owns.

## Multi-task implementation

When a confirmed design is implemented through multiple dependent tasks:

- Follow the dependency graph, but group interface-coupled tasks into the smallest coherent buildable slice. A cross-layer signature change may be edited across its owning layers before the slice compiles.
- At a stable task or slice boundary, run the smallest focused check that can expose a broken contract. Do not require a full acceptance run or conformance audit for every internal task.
- Do not add public abstractions, unused wiring, placeholders, or compatibility scaffolding solely to make an intermediate task appear complete.
- Continue automatically after a stable boundary passes; do not request confirmation between tasks in an already confirmed design.
- Stop only when a material business decision is unresolved, required authority or external access is missing, or no remaining task can progress independently. Record the exact blocker and the evidence already completed.

## Completion

Drive behavior through public paths. Add the smallest test that would fail if the behavior broke.

Before handing off the whole change, perform and report one concise rule-conformance review in the final response, active issue, or plan. A generic "reviewed" is not evidence; name:

1. the applicable `AGENTS.md`, design checkpoint, and DDD outcome;
2. the module, type, and conversion-boundary decisions, including any retained primitive or module-shape exception;
3. one real public path traced through the touched layers, including the Domain owner used by each changed mutation and the persistence reconstruction seam or read-model projection;
4. every completed acceptance item mapped to a named executable test or check;
5. the full diff, required documentation, and any changed repository gate inspected; and
6. the focused tests plus `just check`, and `just verify` when configuration, migration, lifecycle, or installed routes changed.

Do not delete, weaken, or move an existing assertion out of the exercised path merely to make a gate pass. First reproduce an unexpected failure on the untouched baseline or demonstrate that the requested behavior intentionally replaces the old contract. Report a pre-existing failure separately; repair it at its owner only when it blocks required verification, preserving or strengthening the proof.

Fix violations introduced or exposed in the touched responsibility. Record material pre-existing debt without expanding the change unless it prevents the requested behavior or required verification.

## Agent skills

### Issue tracker

Issues are tracked as local Markdown files under `.scratch/`. See `docs/agents/issue-tracker.md`.

### Triage labels

Use the default canonical labels: `needs-triage`, `needs-info`, `ready-for-agent`, `ready-for-human`, and `wontfix`. See `docs/agents/triage-labels.md`.

### Domain docs

This is a single-context repository using root `CONTEXT.md` and `docs/adr/`. See `docs/agents/domain.md`.
