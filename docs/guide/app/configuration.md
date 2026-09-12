# Configuration

Configuration is a process concern owned by `app`. The host translates external strings into typed, validated values and passes only the values needed to construct concrete adapters and runtime resources. Domain, Application, HTTP, and Infrastructure do not load process configuration.

## Loading flow

```text
process arguments
    → Command
    → optional .env
    → process environment
    → command/feature-scoped raw settings
    → validation and type conversion
    → typed settings
    → adapter/runtime construction
```

The command is parsed first so `migrate` cannot fail because serve-only values are absent. Feature-gated capabilities also own feature-gated configuration: the default service does not require TaskPolicy settings, while `reference-task` adds `TASK_POLICY_URL` and its timeout.

Raw settings are private Serde inputs. Validated settings convert external values into `Duration`, `SocketAddr`, enums, and `SecretString` before resource construction. Do not pass raw settings to another architecture crate or defer required validation into handlers.

## Sources and precedence

The template uses an optional repository-root `.env` for local development and process environment variables for runtime configuration. Process variables override `.env`. Missing `.env` is valid; other load failures stop startup.

`.env` is ignored by Git. `.env.example` documents supported local values and must not contain production secrets.

Do not add YAML/TOML files, remote configuration, or a general source-merging abstraction until a real deployment requires them.

## Command and feature scope

`MigrateSettings` contains only migration database and logging settings. `ServeSettings` contains only values required by the selected runtime shape.

Default `serve` requires its database URL and deployment label plus validated defaults/optional telemetry settings. With `reference-task`, the TaskPolicy URL and timeout are additionally required. `DEPLOYMENT_ENVIRONMENT` is a telemetry resource label, not a hidden behavior switch.

Provide a default only when one value is safe and unsurprising for generated local services. Validate positive timeouts, enum values, non-empty deployment labels, and conditional OTLP configuration before constructing resources.

## Secrets

Database URLs deserialize into `SecretString`. This improves debug redaction and owned-memory cleanup but is not a secret manager or privileged-process defense.

Expose a secret only in the narrow expression that constructs the concrete resource. Do not copy it into plain owned strings, add it to error context, log settings, or pass it through HTTP, Application, or Domain.

If another adapter needs credentials, keep the same app-owned construction boundary. Add a secret-manager client only when the application itself must retrieve or renew secrets.

## Testing

Environment mutation is process-global. Configuration tests therefore exercise the same deserialization/validation functions using in-memory config values rather than mutating the test process environment.

Cover command isolation, default/reference feature requirements, defaults, missing required values, invalid enums, zero timeouts, empty deployment labels, and conditional OTLP configuration.
