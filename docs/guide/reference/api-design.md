# API design

> **Status:** Development reference
>
> **Baseline:** The generated service exposes a small versioned Task route and a fixed public error envelope. This chapter does not add routes, serializers, or dependencies.
>
> **Read when:** Adding a resource endpoint, evolving a public response, introducing pagination or filtering, or evaluating an API description format.
>
> **Authority:** The installed Router, DTOs, public Router tests, Project Rules, and HTTP Baseline chapters override this conditional guidance.

## Start with the public contract

Treat an HTTP API as a compatibility boundary, not as a direct serialization of Domain or database types. Name the resource and operation, choose the method and status semantics, and write the success and failure shapes before changing the route. Keep request DTOs and response DTOs in `http`; convert explicitly to Domain and Application types. Do not expose SQL rows, Port errors, downstream wire values, or internal identifiers merely because they are serializable.

Use the existing `/api/v1` namespace for compatible Task operations. Add a new version only for a deliberate incompatible contract; a version prefix does not create a new architecture layer. Preserve the fixed error envelope and stable public codes when extending an existing operation. A message, field, status, or pagination change can be breaking even when Rust still compiles, so update public Router tests with the contract.

## Resource and response conventions

Prefer nouns and resource-oriented paths, predictable method semantics, and one documented representation per operation. Return a representation that is sufficient for the caller without mirroring persistence. Use `201 Created` for successful creation when the resource is persisted, and select `200`, `202`, `204`, or `404` only when their semantics match the operation. Include a `Location` header only when the API has a meaningful canonical resource URL and the contract commits to it.

Keep envelope conventions consistent within the service. Do not add an ad-hoc `{data, errors, meta}` wrapper to one endpoint or leak `Debug` output to make failures useful. Public response fields should have an owner, a compatibility expectation, and a test. Redact secrets, credentials, raw downstream details, and user-submitted sensitive values from both responses and telemetry.

## Pagination

Add pagination only when a collection endpoint has a measured or contractual need. Choose cursor or offset semantics deliberately:

- Cursor pagination is preferable when ordering must remain stable while rows are inserted or deleted; define an opaque, expiring cursor and its sort key.
- Offset pagination is simpler for small, stable datasets but can skip or repeat rows as data changes and becomes expensive at large offsets.

Define default and maximum page sizes, deterministic ordering including a tie-breaker, invalid-cursor behavior, and whether totals are returned. Bound the query and response; never allow an untrusted page size or cursor to become unbounded SQL or a raw query fragment. Keep cursor encoding and decoding at the HTTP/application boundary and pass typed limits and ordering inward.

## Filtering and sorting

Allowlist filter fields, operators, and sort keys in DTO parsing or a dedicated application value type. Convert them to typed query intent; do not concatenate field names, operators, or direction strings into SQL. Decide whether unknown filters are rejected or ignored and test that choice. Define how filters combine, how nulls are ordered, and whether filtering is applied before pagination. A collection contract should document stable default ordering and bounds.

Filtering and sorting are conditional capabilities. Do not add a generic query builder, dynamic SQL abstraction, or shared `utils` module before a second concrete endpoint requires it. Keep SQL construction in Infrastructure and keep the application-facing query model free of SQLx types.

## OpenAPI and generated descriptions

An OpenAPI document is useful when external consumers, SDK generation, governance, or compatibility review justify its maintenance cost. It is not required by this template baseline. If adopted, keep the description at the HTTP boundary, derive or update it alongside DTOs and route tests, and verify that it does not publish internal errors, credentials, uninstalled routes, or undocumented optional fields. Add the dependency only to the owning HTTP crate after the dependency-selection review, and make the generated document part of the public-contract checks.

Do not adopt OpenAPI solely to create a route catalogue: the installed Router, source, tests, and Guide routing table remain authoritative until an explicit API-description contract is introduced.

## Verification checklist

For a new or changed public endpoint, record:

1. method, path/version, request and response schemas, status codes, and error codes;
2. compatibility impact and whether a version change is actually required;
3. bounds and allowlists for pagination, filtering, sorting, and payload size;
4. the owning DTO, Application use case/Port, Domain rule, and Infrastructure query seam;
5. public Router tests for success, malformed/unknown input, and stable errors;
6. `just check`, plus `just verify` for installed-route or composition changes.

Optional API design is guidance, not a new baseline requirement. Implement only the capability the current endpoint and product contract need.
