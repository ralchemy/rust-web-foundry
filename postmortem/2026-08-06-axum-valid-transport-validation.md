# Use axum-valid for transport validation

## Decision

The HTTP crate preconfigures `axum-valid` for validator-backed JSON, query, and form DTOs. Generated services use `axum_valid::Valid` rather than implementing local generic validated extractors, while HTTP still maps `ValidRejection` into the fixed `ApiError` contract.

## Why

Extraction followed by synchronous DTO validation is generic integration code already owned and tested by `axum-valid`. Reimplementing it would add maintenance without creating a service-specific responsibility. The current Task request does not use it because `TaskTitle::parse` is the Domain invariant authority; the dependency is a template baseline for later transport-only constraints.

`axum-valid` 0.25.0 supports Axum 0.8 but depends on validator 0.20.x. The workspace therefore uses validator 0.20.0 so DTO derives and `Valid` share one `Validate` trait. Only JSON, query, form, and validator features are enabled; the crate's response-shaping features remain disabled because `ApiError` owns public status and JSON.

## Rejected alternatives

- Local `ValidatedJson`, `ValidatedQuery`, and `ValidatedForm` extractors duplicate library behavior.
- Keeping validator 0.21.0 would resolve two validator versions whose traits are not interchangeable.
- Enabling `into_json` or `422` would let a dependency define a response outside the fixed HTTP error envelope.
