# Focus HTTP state through FromRef

## Decision

The Router keeps one private generic aggregate, while handlers extract local Task or Health capability states through explicit `FromRef` implementations.

## Why

Passing the aggregate to every handler made unrelated capabilities and generic parameters visible. Separate sub-Routers would narrow the state with less code, but the template intentionally demonstrates Axum's substate mechanism for projects that retain one shared Router state.

Axum's `FromRef` derive does not support the generic state required by the template's static Port dispatch. Local wrapper types and manual implementations preserve static dispatch without exposing concrete adapters or introducing a DI container.
