# Runtime

The binary requires an explicit command:

```text
app migrate
app serve
```

`migrate` reads only `MIGRATION_DATABASE_URL`, applies embedded forward migrations, and exits. `serve` reads `DATABASE_URL`, establishes a real MySQL connection before binding HTTP, and never migrates. This lets production use a DML-only runtime account and run schema changes as a separate release step.

The app optionally loads `.env`, while process variables take precedence. Database URLs remain `SecretString` values until connection construction. Invalid settings, credentials, URLs, timeouts, logging formats, exporters, connections, or listener binds fail startup without fallback or retry. See [Configuration](app/configuration.md) for ownership and validation rules and `.env.example` for the complete baseline.

Logforth provides process logs and fastrace provides optional distributed traces. `TRACE_EXPORTER=none|console|otlp` controls trace collection independently from logging. See [Observability](observability.md) for signal ownership, propagation, sampling, redaction, and export behavior.

`/health/live` performs no external I/O. `/health/ready` calls the application `ReadinessProbe`, implemented as a bounded MySQL `SELECT 1`; it does not validate migrations or the policy service.

SIGTERM or Ctrl-C stops new accepts, drains requests within `SHUTDOWN_TIMEOUT_SECS`, closes the pool, then flushes fastrace. A server that stops early or exceeds the drain bound exits with an error.

## Deployment contract

The template defines process behavior, not a packaging platform. Deploy one immutable build with configuration supplied by the target environment, then preserve this sequence:

1. Run `migrate` as a separate release operation with `MIGRATION_DATABASE_URL` and schema-changing credentials.
2. Start `serve` with its runtime settings and a DML-only `DATABASE_URL`.
3. Route traffic only after `/health/ready` succeeds.
4. On replacement or shutdown, send SIGTERM and allow the process to complete its bounded drain, pool close, and trace flush.

`HTTP_ADDR` defaults to loopback for local safety. A container or orchestrator must set an address reachable in its network, commonly `0.0.0.0:3000`, and separately own TLS termination, ingress, and network policy.

The platform termination grace period must exceed `SHUTDOWN_TIMEOUT_SECS` and leave time for pool closure and trace flushing. Otherwise the platform may send SIGKILL before the application finishes its own shutdown contract.

The probes have deliberately narrow meanings. Liveness proves only that the process can answer without external I/O. Readiness proves only that the configured MySQL is reachable within its bound; it does not prove the expected migration version or TaskPolicy availability.

Separate migration execution does not by itself guarantee zero-downtime schema compatibility while old and new instances overlap. Use an expand/contract migration sequence when a real rolling deployment requires both versions to operate against the same schema.

No Dockerfile, Kubernetes manifest, image base, cross-compilation target, registry workflow, or hosting platform is generated. Add deployment artifacts only for the platform that will actually build and run the service; keep this process contract unchanged unless that platform exposes a concrete incompatibility.
