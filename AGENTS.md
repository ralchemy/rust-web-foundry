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

## Completion

Drive behavior through public paths. Add the smallest test that would fail if the behavior broke, then run `just check`. For changes to configuration, migrations, lifecycle, or the installed route graph, run `just verify`.

## Agent skills

### Issue tracker

Issues are tracked as local Markdown files under `.scratch/`. See `docs/agents/issue-tracker.md`.

### Triage labels

Use the default canonical labels: `needs-triage`, `needs-info`, `ready-for-agent`, `ready-for-human`, and `wontfix`. See `docs/agents/triage-labels.md`.

### Domain docs

This is a single-context repository using root `CONTEXT.md` and `docs/adr/`. See `docs/agents/domain.md`.
