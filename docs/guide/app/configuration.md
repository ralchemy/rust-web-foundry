# Configuration

Configuration is a process concern owned entirely by `app`. The host translates external strings into typed, validated values and passes only the values required to construct concrete adapters and runtime resources. Domain, Application, HTTP, and Infrastructure never load process configuration.

## Loading flow

```text
process arguments
    → Command
    → optional .env
    → process environment
    → RawMigrateSettings or RawServeSettings
    → validation and type conversion
    → MigrateSettings or ServeSettings
    → adapter and runtime construction
```

The command is parsed first. This keeps the configuration interface narrow: `migrate` cannot fail because a serve-only variable is absent, and `serve` never asks for migration credentials.

Raw settings are private Serde input types. They preserve external units and names such as milliseconds and seconds. Validated settings convert those values into `Duration`, `SocketAddr`, enums, and `SecretString` before any listener or request processing starts. Do not pass raw settings to another module or validate a required value lazily in a handler.

## Sources and precedence

The template uses two sources only:

1. An optional repository-root `.env` for local development.
2. Process environment variables for every environment.

`dotenvy::from_filename` keeps an already-set process variable instead of replacing it with the `.env` value. This lets a shell, CI job, or deployment platform override local defaults. Missing `.env` is valid; any other load error fails startup rather than being swallowed.

`.env` is ignored by Git. `.env.example` contains working local values and documents the supported names, but it is not loaded automatically and must not contain production secrets.

Do not add YAML/TOML files, remote configuration, or source-merging rules until a current deployment requires them. More sources make the effective value harder to explain and verify.

## Command-scoped settings

`MigrateSettings` contains only the migration database URL and logging settings. `ServeSettings` contains only values needed to build and run the HTTP process. There is no global Config passed through the workspace and no environment-dependent service locator.

`DEPLOYMENT_ENVIRONMENT` is a required telemetry resource label. It is not a hidden switch for development, staging, or production behavior. When behavior must vary, name and validate the actual choice directly rather than branching on an environment label.

## Defaults and validation

Provide a default only when one value is safe and unsurprising for every generated local service. The baseline defaults cover the bind address, finite timeouts, log format, log filter, and trace exporter. Credentials, database URLs, policy URLs, and the deployment label remain required.

Validation happens before resource construction:

- enum deserialization rejects unknown log formats and trace exporters;
- addresses and integer units must parse into their declared types;
- all timeouts must be positive;
- the deployment label must not be empty;
- OTLP requires an endpoint only when `TRACE_EXPORTER=otlp`.

Startup then validates values owned by concrete adapters, such as the Task Policy URL and database connection. Any failure propagates to the process boundary; the template has no silent fallback, retry loop, or partially configured server.

## Secrets

Database URLs deserialize directly into `SecretString`. This makes debug output redacted, makes access explicit through `ExposeSecret`, and arranges for the owned secret memory to be zeroized on drop. It does not prevent every memory-copy, swap, core-dump, or privileged-process threat.

Call `expose_secret()` only in the expression that constructs the concrete database connection. Do not convert the secret to a plain owned `String`, include it in error context, log the settings object, or pass it through HTTP, Application, or Domain.

If a future adapter needs another credential, keep it wrapped through the same app-owned construction seam. Add a secret-manager client only when the application itself must retrieve or renew secrets; deployment-provided environment values need no extra runtime abstraction.

## Testing

Environment mutation is process-global and makes parallel tests interfere with one another. The settings module therefore separates environment collection from pure deserialization and validation. Tests build an in-memory `config::Config` and exercise the same private conversion functions used by production without modifying the test process environment.

The contract tests cover command isolation, defaults, missing required values, invalid enums, zero timeouts, empty deployment labels, and conditional OTLP configuration. Command parsing uses the same approach: production supplies `std::env::args`, while tests pass explicit iterators to the pure parser.
