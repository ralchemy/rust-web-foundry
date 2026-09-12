# API design

> **Status:** Development reference
>
> **Baseline:** The default generated service exposes only health/readiness. The optional `reference-task` feature exposes versioned Task routes and the fixed public error envelope. This chapter installs no routes, serializers, or dependencies.
>
> **Read when:** Adding a resource endpoint, evolving a public response, introducing pagination/filtering/sorting, or evaluating an API description format.
>
> **Authority:** The installed Router, DTOs, public Router tests, root project contract, and HTTP Baseline chapters override this conditional guidance.

## Start with the public contract

Treat an HTTP API as a compatibility boundary, not a direct serialization of Domain or database types. Define the caller-visible operation, method, path, success shape, failure shape, and compatibility expectation before changing routing. Keep request/response DTOs in HTTP and convert explicitly to Domain/Application types.

Do not expose SQL rows, Port errors, downstream wire values, internal `Debug` output, or fields merely because they are serializable. Public response fields need an owner and a test.

Use an existing API namespace only when extending a compatible installed contract. Add a new version only for a deliberate incompatible public contract; a version prefix does not create a new architecture layer.

## Resource and response conventions

Prefer resource-oriented paths and method/status semantics that match the operation. Use `201 Created` for persisted creation when that is the public contract; choose `200`, `202`, `204`, `404`, or conflict outcomes from their actual semantics rather than convention alone. Add `Location` only when the API commits to a canonical resource URL.

Keep the service's public error envelope consistent. Do not introduce an ad-hoc response wrapper or leak internal error text to make one endpoint easier to debug. Redact secrets, credentials, raw downstream details, and sensitive submitted values from responses and telemetry.

## Collections

Add pagination, filtering, or sorting only when the collection contract needs them. Choose cursor versus offset semantics from caller behavior and data-change characteristics. Treat accepted filter/sort fields as an allowlisted public contract; do not pass arbitrary field names or SQL fragments through to persistence.

## Documentation formats

OpenAPI or another schema format is useful when real clients, tooling, or compatibility review need a machine-readable API description. Do not add an API-description dependency solely because the framework can generate one. The installed Router and tested public DTO/error behavior remain the executable authority.

## Verification

Drive API changes through the installed Router and assert the exact public contract. For a route/composition change, finish with `just verify` instead of running a separate final `just check`, because `just verify` already includes the check gate.
