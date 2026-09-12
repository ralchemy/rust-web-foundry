# Security boundaries

The generated default service is an unauthenticated JSON service with health/readiness, MySQL connectivity, configuration, observability, and graceful shutdown. The optional `reference-task` feature adds the Task API, TaskPolicy downstream, and reference schema. Neither shape implies authentication, authorization, browser security, network policy, or complete production hardening.

Security behavior stays with the layer that owns the trust decision. This chapter is an index; linked boundary chapters and executable code/tests own implementation details.

## Trust boundaries

| Input or dependency | Owner | Baseline treatment |
|---|---|---|
| HTTP method, path, headers, and body | `http` | route matching, bounded extraction, transport validation, fixed public errors |
| raw business values | `domain` / `application` | invariant construction and explicit business decisions before persistence |
| configured downstreams | `infrastructure` | validated configuration, bounded I/O, strict decoding, stable failure classification |
| MySQL | `infrastructure` | bound values, schema constraints, checked SQL metadata, concrete-error containment |
| process settings and credentials | `app` | command-scoped parsing, fail-fast validation, narrow secret exposure |
| logs and traces | outer crates | allowlisted operational metadata and fixed redaction rules |

## Installed controls

HTTP DTOs are transport representations, not trusted Domain state. Convert raw business values through Domain constructors before they enter Application. `ApiError` is the only public error authority; dependency messages, SQL, configuration, stack traces, rejected values, and raw validation details do not become response text.

The MySQL adapter uses SQLx checked queries and bound values. Dynamic SQL syntax still requires explicit allowlists. Production keeps migrations separate from `serve`; the local Compose account is development convenience, not a production credential model.

When `reference-task` is enabled, `TASK_POLICY_URL` is deployment configuration rather than request input. The adapter validates the configured endpoint shape, disables redirects, uses a finite timeout, and translates failures to stable Application categories. A future user-selected destination is a new SSRF boundary and needs an explicit destination/network policy.

Database URLs stay wrapped as secrets through app-owned construction. Logs and traces never include SQL text, bind values, request/response bodies, headers, Task titles, database URLs, credentials, complete downstream URLs, or raw dependency errors.

The default HTTP address and local MySQL publish on loopback. The template does not terminate TLS. A real deployment owns ingress/TLS, proxy trust, database transport security, secret delivery, and network policy.

## Capabilities not installed

Do not infer authentication, authorization, CORS, sessions/cookies/CSRF, rate limiting, request deadlines, browser security headers, dependency scanning, or a secret manager from the libraries already present. Add a capability only after its threat model, public contract, owner, configuration, and verification are defined.

In particular:

- authentication turns a credential into a trustworthy principal; authorization remains a separate business decision;
- CORS is a browser interoperability policy, not access control;
- a ULID or trace ID is not caller identity or authorization;
- `SecretString` improves handling but is not a secret manager;
- `DefaultBodyLimit` does not constrain arbitrary streaming/body consumers;
- a timeout does not prove a mutating downstream/database operation was rolled back.

## Extension and verification

Start a new security capability from the threat and ownership boundary rather than from a crate choice. Keep credential extraction and public failures in HTTP, business authorization in Application or Domain, concrete external verification/storage in Infrastructure, and configuration/lifecycle in `app`.

Drive installed controls through the public Router or real process seam. Tests should prove both allowed and rejected behavior, preserve the fixed error envelope and 404/405 semantics, and show sensitive data is absent from responses and telemetry. Finish with `just check`, or `just verify` instead when configuration, middleware, composition, lifecycle, SQL, or the installed route graph changes.
