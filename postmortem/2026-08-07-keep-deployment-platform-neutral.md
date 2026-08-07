# Keep deployment platform neutral

## Decision

The generated service defines a deployment process contract but emits no Dockerfile, Kubernetes manifest, image workflow, cross-compilation target, or hosting-platform configuration.

The contract is: build one immutable artifact, run `migrate` separately with schema credentials, start `serve` with runtime credentials, gate traffic with the health endpoints, and grant SIGTERM enough time for bounded request draining, pool closure, and fastrace flushing.

## Why

Packaging details are platform decisions. A Docker base image, CPU target, libc choice, TLS termination model, registry, and orchestrator manifest cannot be selected correctly from the application architecture alone. Preselecting them would make the template look production-ready while leaving unverified operational assumptions.

The process already exposes the stable seams a deployment needs. Keeping those seams independent of packaging lets a generated project add one concrete delivery path without changing Application, HTTP, Domain, or Infrastructure ownership.

## Limits

- Loopback is the safe local bind default; a deployment must explicitly choose its reachable interface and network controls.
- Readiness checks MySQL reachability, not migration version or TaskPolicy availability.
- Separate migrations do not guarantee rolling schema compatibility; expand/contract sequencing belongs to a deployment that actually overlaps versions.
- The orchestrator grace period must exceed the configured application drain and leave cleanup margin.

## Rejected alternatives

- A generic multi-stage Dockerfile would still encode an unverified base image, target architecture, libc, certificate, and build-cache policy.
- Kubernetes examples would create manifests without owning ingress, secrets, resources, rollout policy, or the target cluster contract.
- Running migrations during `serve` would couple runtime availability to schema privileges and make concurrent replicas race on release work.
