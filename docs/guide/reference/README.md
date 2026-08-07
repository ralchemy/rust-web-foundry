# Development reference

> **Status:** Development reference
>
> **Baseline:** These chapters describe conditional engineering choices. A capability is not installed or required unless the manifests, source, and Baseline Guide say so.
>
> **Authority:** Cargo, source code, tests, Just, CI, and applicable `AGENTS.md` files override this material.

This directory preserves reusable Rust Web knowledge without turning every option into generated architecture. Root and crate Project Rules link here through narrow triggers, so an agent reads a chapter when a change creates its subject rather than loading the entire reference for every task.

## Chapters

- [Performance](performance.md): measurement, async blocking, pool sizing, query work, HTTP payload cost, and tool selection.
- [Async and cancellation](async-and-cancellation.md): dropped futures, channels, background work, timeouts, task ownership, blocking work, and locks.
- [Authentication and authorization](authentication-and-authorization.md): credential mechanisms, principal construction, policy ownership, revocation, and browser security.
- [Outbound HTTP](outbound-http.md): client ownership, time budgets, safe retries, idempotency, resilience, and cross-system consistency.
- [gRPC](grpc.md): when to add a sibling inbound adapter, protobuf ownership, status mapping, streaming, and process lifecycle.
- [Typestate](typestate.md): when compile-time state transitions repay their type and conversion cost.
- [Deployment](deployment.md): image and platform decisions, external configuration, probes, migrations, rollouts, and termination.
- [API design](api-design.md): resource contracts, compatibility, pagination, filtering, sorting, and OpenAPI selection.

## How to use a reference

1. Trace the current public path and read its Baseline Guide and applicable Project Rules first.
2. State the concrete requirement, failure, workload, or deployment target that triggered the reference.
3. Select the smallest option that satisfies that requirement at its owning seam.
4. Add the observable test, measurement, or acceptance path that proves the choice.
5. Update the Baseline Guide only after the capability becomes part of the generated service.

Reference material can explain alternatives and show conditional examples. It must not introduce an empty module, dependency, configuration switch, or background subsystem merely because the option is documented.

## Moving knowledge between layers

Promote a statement into `AGENTS.md` when it is an always-applicable constraint whose violation repeatedly produces invalid code. Keep rationale and alternatives here, with the Project Rule linking to the relevant chapter instead of copying it.

Demote a Project Rule into this directory when it stops being universally required and becomes a choice that depends on workload, platform, or product behavior. Skills remain procedures: they may read a reference, but they do not become a second owner of its rules.
