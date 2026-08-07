# Authentication and authorization

> **Status:** Development reference
>
> **Baseline:** The generated service is unauthenticated. It has no principal, credential verifier, session store, login route, password store, or authorization policy.
>
> **Read when:** Adding sessions, bearer credentials, JWT validation, OAuth/OIDC, passwords, authenticated request context, or access-control policy.
>
> **Authority:** [Security boundaries](../security.md), the fixed public error envelope, and crate dependency rules override examples here.

Authentication turns a credential into a trustworthy principal. Authorization decides whether that principal may perform an action on a resource. Keep those decisions separate: successful authentication is not blanket permission, and possession of a Task ID is not authorization.

## Choose the credential mechanism from the client contract

| Client and trust model | Typical mechanism | Decisions that must exist first |
|---|---|---|
| browser application with server-owned login state | opaque server-side session | cookie scope, CSRF defense, session storage, rotation, expiry, revocation, and logout |
| service/API caller presenting a token | verified bearer credential | issuer, audience, accepted algorithms, key rotation, expiry, replay exposure, and revocation expectations |
| organization delegates identity | OAuth 2.0 / OpenID Connect provider | authorization flow, redirect origins, state/nonce handling, claim mapping, provider outage, and account linking |

Do not start by choosing a Rust crate. First write the credential transport, trust issuer, lifecycle, and public failure contract. A session identifier or bearer token is a credential once issued; never log, trace, or put it in a URL. Browser credentials require TLS and a deliberate cookie/CSRF model. See the OWASP [Authentication](https://cheatsheetseries.owasp.org/cheatsheets/Authentication_Cheat_Sheet.html), [Session Management](https://cheatsheetseries.owasp.org/cheatsheets/Session_Management_Cheat_Sheet.html), and [CSRF](https://cheatsheetseries.owasp.org/cheatsheets/Cross-Site_Request_Forgery_Prevention_Cheat_Sheet.html) guidance.

## Preserve the architecture boundary

The request path should have one explicit conversion:

```text
HTTP credential
    -> HTTP authentication extractor or middleware
    -> Application-owned principal value
    -> Application use case authorization policy
    -> Domain operation / outbound Ports
```

- `http` extracts the credential, maps missing or invalid credentials into the public `401` contract, and stores the resulting request principal in extensions.
- `application` owns the principal shape needed by use cases and the principal/resource/action decision when authorization coordinates business data.
- `domain` owns a pure authorization rule only when it is an invariant expressed entirely in Domain concepts.
- `infrastructure` implements external identity verification or distributed session storage behind an Application Port.
- `app` loads verifier keys, issuer settings, session credentials, and constructs the concrete path.

HTTP route grouping can require that a principal exists, but middleware must not become the only owner of resource-level business policy. Map an authenticated-but-disallowed operation to the stable forbidden category without revealing whether a protected resource exists when that distinction leaks information.

## Credential lifecycle is part of correctness

Stateless token validation still needs pinned algorithms, issuer and audience checks, clock policy, key rotation, and an answer for revocation. Embedding roles in a long-lived token delays permission changes until the token is replaced or an online check occurs. Server-side sessions trade per-request storage access for direct revocation and controlled state.

Password verification is deliberately expensive. Use a current password-hashing algorithm and parameters based on measured production hardware and current OWASP [Password Storage guidance](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html); run bounded hashing work through `spawn_blocking` so it does not block Tokio workers. Never expose whether a username or password was the failed factor, and never record either value.

## Verification

Drive the installed Router and prove missing, malformed, expired/revoked, allowed, and forbidden cases through the same extractor and policy path production uses. Tests should also prove that public errors and telemetry contain no credentials or identity-provider details. Add focused policy tests in Application or Domain for resource/action combinations; do not substitute a handler-only role check for them.
