# Security boundaries

The generated service is an unauthenticated JSON API with one configured downstream service and one MySQL database. Its baseline protects the boundaries it actually implements; it is not a claim that authentication, authorization, browser security, network policy, or production hardening already exists.

Security behavior stays with the layer that owns the relevant trust decision. This chapter is an index of those decisions, while the linked boundary chapters remain authoritative for implementation details.

## Trust boundaries

| Input or dependency | Owner | Baseline treatment |
|---|---|---|
| HTTP method, path, headers, and body | `http` | route matching, bounded JSON extraction, transport validation, and fixed public errors |
| raw business values | `domain` and `application` | invariant construction and explicit policy decisions before persistence |
| configured TaskPolicy service | `infrastructure` | validated scheme, redirects disabled, finite timeout, strict response decoding, and stable failure classification |
| MySQL | `infrastructure` | bound SQL values, schema constraints, checked query metadata, and concrete-error containment |
| process settings and credentials | `app` | command-scoped parsing, fail-fast validation, narrow secret exposure, and concrete construction |
| logs and traces | outer crates | allowlisted operational metadata and fixed redaction rules |

## Installed baseline

### HTTP input and Domain construction

JSON handlers registered under the installed Router inherit its 8 KiB `DefaultBodyLimit` when they use Axum's bounded body extractors. Each request DTO independently defines whether unknown fields are accepted. Direct `Body` consumption or a future streaming endpoint must define and test its own limit. See [Validation](http/validation.md) and Axum's [`DefaultBodyLimit` contract](https://docs.rs/axum/0.8.9/axum/extract/struct.DefaultBodyLimit.html).

Successful JSON decoding produces an HTTP DTO, not trusted Domain state. In the canonical CreateTask flow, Application constructs `TaskTitle` through its validating Domain constructor before calling TaskPolicy or MySQL. Transport validation and Domain invariants do not authenticate a caller or authorize an operation.

[`ApiError`](../../crates/http/src/errors/mod.rs) is the only public error authority. Dependency messages, configuration, SQL, paths, stack traces, rejected values, and raw validation details never become response text. See [Error handling](architecture/error-handling.md).

### Persistence and outbound HTTP

The MySQL adapter uses checked SQLx macros and bound values. Query macros verify SQL shape and parameter types; they do not make interpolated SQL syntax safe. Dynamic identifiers, sort directions, or clauses must come from an explicit allowlist rather than request text. Database constraints remain the final persistence boundary. See [Database](infrastructure/database.md).

Production runs `migrate` separately from `serve` so the runtime database account can be limited to application DML. The local Compose account intentionally combines permissions for development convenience and is not a production credential model.

`TASK_POLICY_URL` is deployment configuration, not request input. [`HttpTaskPolicy`](../../crates/infrastructure/src/outbound_http/task_policy.rs) accepts only HTTP or HTTPS, disables redirects, applies a finite timeout, and rejects unexpected response shapes. This is not a defense for a future user-supplied URL feature: such a feature must define allowed destinations, DNS and network policy, redirect behavior, and response limits as a new trust boundary. See the [OWASP SSRF guidance](https://cheatsheetseries.owasp.org/cheatsheets/Server_Side_Request_Forgery_Prevention_Cheat_Sheet.html).

### Secrets and operational data

`.env` is ignored by Git, and `.env.example` contains only fixed local-development values. Database URLs deserialize into `SecretString` and are exposed only where `app` constructs the concrete connection. `SecretString` improves debug redaction and memory cleanup; it is not a secret manager, access-control mechanism, or defense against a privileged process. See [Configuration](app/configuration.md).

Application instrumentation is fastrace-only. Do not add a direct `tracing` dependency, subscriber, or bridge merely to capture SQLx dependency events. Database visibility comes from explicit Infrastructure-owned fastrace spans with safe properties such as operation and collection names. Logs and spans never include SQL text, bind values, Task Titles, bodies, headers, database URLs, complete downstream URLs, credentials, or raw dependency errors. See [Observability](observability.md).

### Local exposure

The generated HTTP address defaults to `127.0.0.1:3000`, and local Compose publishes MySQL only on `127.0.0.1:3306`. The fixed `app` and `root` database passwords are local test data; do not deploy `compose.yaml` or reuse those credentials in a shared environment. Docker otherwise publishes an address-less port mapping on all host addresses; see [Docker's port-publishing documentation](https://docs.docker.com/engine/network/port-publishing/).

The service does not terminate TLS. A production deployment owns inbound TLS and MySQL transport security explicitly; binding the application to a non-loopback address is a deployment decision, not an environment-name side effect.

## Capabilities not installed

Do not infer the following capabilities from Axum, ULIDs, validation, CORS examples, or the fixed error envelope:

| Capability | Add only after defining |
|---|---|
| authentication | credential type, issuer or verifier, rotation, expiry, revocation, and public failure contract |
| authorization | principal, resource, action, ownership rules, and the Application or Domain policy that decides them |
| TLS | termination owner, trusted proxies, certificate lifecycle, and database TLS requirements |
| CORS | browser origins, credentials mode, methods, headers, and preflight behavior; CORS is not access control |
| sessions, cookies, and CSRF | browser credential mechanism, cookie scope, storage, expiry, and state-changing request defense |
| rate limiting | quota key, trusted client identity, replica consistency, storage, and `429` contract |
| request deadlines | cancellation and commit semantics for each operation, especially mutations |
| security response headers | whether the service or ingress serves browser-rendered content and which layer owns HTTPS and header policy |
| dependency vulnerability scanning | CI tool, update ownership, advisory exception policy, and failure behavior |

Cookie-authenticated browser mutations normally require a deliberate CSRF defense; `SameSite` is defense in depth rather than a universal replacement. Bearer credentials that browsers do not attach automatically have a different threat model. See the [OWASP CSRF guidance](https://cheatsheetseries.owasp.org/cheatsheets/Cross-Site_Request_Forgery_Prevention_Cheat_Sheet.html) and [Middleware](http/middleware.md) for conditional Axum ownership examples.

## Non-guarantees

- A ULID is an identifier, not proof that the caller may access the resource.
- Valid JSON and a valid Domain value do not authenticate or authorize the caller.
- `SecretString` does not retrieve, rotate, or control access to a secret.
- `query!` does not make request-selected SQL syntax safe.
- `DefaultBodyLimit` does not constrain every possible body consumer or provide general denial-of-service protection.
- CORS does not protect a non-browser client from calling an endpoint.
- A trace ID is correlation metadata, not caller identity.

## Extension and verification

Start a new security capability by defining its threat and ownership boundary, not by selecting a crate. Keep credential extraction and public failures in HTTP, business authorization in Application or Domain, concrete external verification or distributed storage in Infrastructure, and configuration plus lifecycle in `app`. Read [Authentication and authorization](reference/authentication-and-authorization.md) before choosing a credential mechanism or principal model.

Drive any installed control through the public Router. Tests must prove both rejection and allowed behavior, preserve 404/405 and the fixed error envelope, and ensure sensitive values are absent from responses and recorded telemetry. Run `just check`; use `just verify` when configuration, middleware, composition, lifecycle, or the installed route graph changes.

The existing `add-endpoint` and `review-pr` Skills already cover public-path testing and secret/data leakage. Add no Security Skill until a distinct security workflow repeats often enough to need its own procedure.
