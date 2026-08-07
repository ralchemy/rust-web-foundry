# Observability

The generated service separates process logs from distributed traces. Logforth is the only `log` facade implementation; fastrace owns spans and W3C context propagation. OpenTelemetry appears only at the optional OTLP trace-export boundary. The template does not install OpenTelemetry logs, metrics, Prometheus, or a `/metrics` route.

## Runtime modes

`app` initializes logging first and then selects one trace mode from `TRACE_EXPORTER`:

| Mode | Reporter | Request context | Outbound `traceparent` |
|---|---|---|---|
| `none` | none | noop | absent |
| `console` | fastrace `ConsoleReporter` | active | present |
| `otlp` | `OpenTelemetryReporter` over OTLP/gRPC | active | present |

`console` is the local default and requires no external service. `otlp` requires `OTEL_EXPORTER_OTLP_ENDPOINT`; an unknown mode, a missing conditional endpoint, or an invalid exporter setting fails startup. `none` disables trace collection without disabling process logs and needs no second enable flag.

The OTLP Resource contains the generated package name and version plus `DEPLOYMENT_ENVIRONMENT`. Reporter construction belongs to [`app::observability`](../../app/src/observability.rs); no exporter type crosses into an architecture crate.

## Trace shape and ownership

The canonical CreateTask path produces this reference trace when tracing is active:

```text
POST /api/v1/tasks                    http server root
└── task.create                       http internal span
    ├── task_policy.check             infrastructure HTTP client span
    └── mysql.task.insert             infrastructure MySQL client span
```

Each owner instruments only the concrete work it understands:

| Owner | Responsibility |
|---|---|
| `app` | initialize Logforth and the selected Reporter, attach service Resource attributes, and flush fastrace during shutdown |
| `http` | extract or start the request context, own the server root and `task.create`, and mark returned `5xx` responses |
| `infrastructure` | own reqwest and MySQL client spans, inject outbound W3C context, and classify concrete dependency failures |
| `application` and `domain` | remain free of Logforth, fastrace, and OpenTelemetry dependencies or side effects |

Health requests use noop HTTP spans so probes do not create continuous traces. Because child spans use the active local parent, readiness database instrumentation is also noop when the health root is noop.

## Context propagation and sampling

[`FastraceLayer`](../../crates/http/src/middleware/trace.rs) accepts a valid inbound W3C `traceparent`, including its sampled flag. A missing or invalid header starts a new random context, which is sampled. The initial template has no sample-ratio setting or custom sampler:

- preserve an upstream sampling decision when one exists;
- sample every trace started by this service when tracing is enabled;
- let an OTLP Collector apply deployment-specific sampling when lower retained volume is required;
- select `none` when no tracing output is wanted.

The outbound TaskPolicy adapter creates its own client span and calls `fastrace_reqwest::traceparent_headers()` inside that span. A bare reqwest call does not create a fastrace span or inject context. With `TRACE_EXPORTER=none`, no active parent exists and the adapter emits no trace header.

## Attributes and failure status

Use low-cardinality operational attributes whose owner can define safely:

- the server root uses the method, URL path without query, matched route, response status, and `span.kind=server`;
- `task.create` uses `span.kind=internal` and records the opaque Task ID only after success;
- `task_policy.check` uses `span.kind=client`, the method, stable service name, route template, and response status;
- `mysql.task.insert` uses `span.kind=client`, `db.system.name=mysql`, `db.operation.name=INSERT`, and `db.collection.name=tasks`.

Mark a returned HTTP `5xx` as a server-span error; a handled `4xx` remains unset. Child failures use stable categories such as `task_policy_unavailable` or `task_persistence`. Do not attach an internal error message or status description.

Logs and spans must never contain secrets, database URLs, Task Titles, request or response bodies, headers, query strings, SQL text, complete downstream URLs, or raw SQLx/reqwest errors. This is a boundary rule, not a formatter feature.

## Process logs and correlation

`RUST_LOG` configures Logforth's RustLog filter and defaults to `info`. `LOG_FORMAT` accepts only `text` or `json`. `FastraceDiagnostic` adds the active trace ID, span ID, and sampled state to a log record without duplicating that record as a trace event. In `none` mode or outside a trace, the same log remains valid without correlation fields.

Only outer crates may emit process logs. Infrastructure can record a safe dependency category or database code before returning a stable Application error; Application and Domain do not log failures they cannot classify concretely.

## Startup, export failure, and shutdown

Invalid log, trace, endpoint, or timeout configuration stops startup. After startup, telemetry export failure may drop telemetry but never changes a business result or the public HTTP error envelope; the OTLP Reporter records export failure through the `log` facade.

Shutdown stops new accepts, drains requests within the configured bound, closes the MySQL pool, and finally calls `fastrace::flush()`. Flush invocation is guaranteed, but the API cannot prove that a remote collector durably accepted every span. See [Runtime](runtime.md) for the complete process lifecycle.

## Verification boundary

Settings tests cover the three exporter values and the conditional OTLP endpoint. The app integration path proves that an active request propagates `traceparent` to the local TaskPolicy stub. `just verify` runs the production server path, checks propagation, sends SIGTERM, and requires clean shutdown.

The baseline does not start an OTLP Collector or assert vendor-specific output. Add such an acceptance path only when a generated service depends on a concrete telemetry backend contract.
