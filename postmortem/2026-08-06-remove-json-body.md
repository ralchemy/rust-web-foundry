# Remove the JsonBody extractor

## Decision

The Task handler uses Axum `Json<T>` directly as a `Result` extractor and converts `JsonRejection` through `ApiError`. The local `JsonBody<T>` extractor and its directory are removed.

## Why

`JsonBody<T>` delegated all extraction to Axum and added only rejection conversion for one caller. Its interface mirrored `Json<T>`, so deleting it removes a shallow module without losing behavior: `ApiError` now owns the same 400, 413, and 415 classification, while Router tests continue to prove the public envelope.

The Router's body limit and the DTO's unknown-field rule are independent of this wrapper. Validator-backed DTOs use `axum_valid::Valid` and map `ValidRejection` through the same HTTP error authority.

## Rejected alternatives

- Using Axum's default rejection response would break the fixed JSON error envelope.
- Renaming the wrapper or replacing it with another generic local extractor would preserve the same shallow interface.
- Adding `axum-extra::WithRejection` for one route would introduce another dependency and a more complex handler type without removing the required `ApiError` classification.
