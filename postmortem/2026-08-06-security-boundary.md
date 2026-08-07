# Make the security boundary explicit

## Decision

The generated Guide includes one cross-cutting Security chapter that lists the controls the template actually implements, their limits, and the security capabilities that remain absent. Detailed rules stay in their owning boundary chapters and Project Rules.

Local Compose publishes MySQL as `127.0.0.1:3306:3306`. Fixed development credentials and schema-changing access remain local conveniences rather than a remotely reachable default.

Application instrumentation remains fastrace-only. The template does not add a direct `tracing` dependency, subscriber, or bridge to surface dependency-internal SQLx events.

## Why

Security spans HTTP input, Domain construction, outbound calls, persistence, configuration, telemetry, and deployment. Without one discoverable index, an agent can mistake a local control for a complete guarantee—for example, treating ULID as authorization, `SecretString` as secret management, or SQLx checked macros as protection for interpolated SQL syntax.

An address-less Docker port publication listens on all host addresses by default. Every generated local command already connects through `127.0.0.1`, so exposing the fixed MySQL development account beyond loopback serves no current workflow.

## Rejected alternatives

- A broad production-security checklist would describe authentication, TLS, CORS, CSRF, rate limiting, headers, and dependency scanning that the generated service does not install or verify.
- Repeating full security rules in one chapter would create a second authority beside Configuration, Validation, Database, Error Handling, Middleware, and Observability.
- Leaving MySQL published on every host interface would preserve remote development access that the template does not use while widening exposure of fixed credentials.
- Adding a tracing subscriber or SQLx logging bridge would create a second observability stack and risk recording dependency details that the fastrace adapter spans deliberately omit.
