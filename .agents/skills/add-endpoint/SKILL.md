---
name: add-endpoint
description: Add or change an HTTP endpoint in this five-crate Rust web workspace. Use when a request affects an Axum route, HTTP contract, use case, domain rule, Port, or adapter.
---

# Add an endpoint

1. Read the root `AGENTS.md`, `CONTEXT.md`, `docs/guide/task-flow.md`, and each touched crate's `AGENTS.md`.
2. Trace the nearest existing public route through application and adapters. Reuse its owners before adding files or abstractions.
3. Write one failing test at the innermost changed public seam. Touch only layers required by the behavior.
4. Work inside out through only the required owners: Domain invariant; Application use case or Port when orchestration or a business decision exists; Infrastructure adapter when an external capability exists; HTTP DTO/error/handler/route; then app wiring when construction changes.
5. Keep external details at adapters and fixed public failures in HTTP. Add every public route to `docs/guide/http/routing-and-handlers.md`. Update another Guide chapter only when its stable rule or explanation becomes false; do not repeat the route catalogue across cross-cutting chapters.
6. Run the focused test, then `just check`. Run `just verify` when wiring, configuration, lifecycle, or the installed route graph changed.
