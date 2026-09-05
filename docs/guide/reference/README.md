# Development reference

> These chapters describe conditional engineering choices. A capability is not installed or required unless the manifests, source, tests, and Baseline Guide establish it.

This directory preserves reusable Rust Web knowledge without placing it in every implementation context. Read a chapter only when the current production path does not answer a concrete question raised by the task.

## Chapters

- [Rust quality baseline](idiomatic-rust.md): the pinned `rust-skills` review integration and project authority boundary.
- [Performance](performance.md): measurement, async blocking, pool sizing, query work, HTTP payload cost, and tool selection.
- [Async and cancellation](async-and-cancellation.md): dropped futures, channels, background work, timeouts, task ownership, blocking work, and locks.
- [Authentication and authorization](authentication-and-authorization.md): credential mechanisms, principal construction, policy ownership, revocation, and browser security.
- [Outbound HTTP](outbound-http.md): client ownership, time budgets, safe retries, idempotency, resilience, and cross-system consistency.
- [gRPC](grpc.md): adapter placement, protobuf ownership, status mapping, streaming, and process lifecycle.
- [Typestate](typestate.md): when compile-time state transitions repay their type and conversion cost.
- [Deployment](deployment.md): image and platform decisions, external configuration, probes, migrations, rollouts, and termination.
- [API design](api-design.md): resource contracts, compatibility, pagination, filtering, sorting, and OpenAPI selection.

## Using a reference

1. Trace the current public path and its tests first.
2. State the requirement, failure, workload, or deployment target that makes the reference relevant.
3. Select the smallest option that satisfies that requirement at its owning seam.
4. Add observable evidence for the choice.
5. Update baseline documentation only after the capability becomes part of the generated service.

Reference material must not introduce an empty module, dependency, configuration switch, public contract, or background subsystem merely because an option is documented.
