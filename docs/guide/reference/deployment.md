# Deployment

> **Status:** Development reference
>
> **Baseline:** The template defines a process deployment contract but generates no Dockerfile, Kubernetes manifest, registry workflow, release automation, or hosting-platform configuration.
>
> **Read when:** Selecting an image, CPU/OS target, container runtime, orchestrator, migration rollout, probe configuration, secret injection, or release workflow.
>
> **Authority:** The [Runtime deployment contract](../runtime.md#deployment-contract), App Project Rules, and the target platform's verified behavior override generic examples.

Deployment is the adapter between the built process and a specific operating platform. Add one concrete delivery path when the target environment is known; do not try to make one generated manifest represent every registry, architecture, libc, ingress, secret store, and rollout policy.

## Define the target first

Before writing an artifact, record:

- target OS and CPU architecture;
- libc and dynamic-library requirements;
- how outbound TLS obtains trusted roots;
- whether the filesystem is read-only and which paths are writable;
- runtime UID/GID and privilege policy;
- how configuration and secrets are injected;
- how the platform sends termination and enforces its grace period;
- how migrations run once per release;
- how liveness, readiness, logs, and traces are observed.

A successful local image build is not evidence that the registry artifact runs on the production architecture or receives signals as expected.

## Container image choices

A multi-stage build separates compilation from the runtime image and copies only required artifacts forward. Docker documents the mechanism in its [multi-stage build guide](https://docs.docker.com/build/building/multi-stage/). The builder should honor `rust-toolchain.toml`, use the committed lockfile, and produce the exact target the runtime can execute.

Choose the runtime base from operational requirements:

| Runtime choice | Trade-off |
|---|---|
| Distribution slim image | Provides libc, certificates, user tools, and a familiar debugging base, but increases image contents |
| Distroless-style image | Reduces runtime surface while retaining selected libraries/assets; debugging requires external tooling |
| `scratch` with a static binary | Smallest filesystem, but every certificate, timezone, identity, and debugging requirement must be supplied deliberately |

Do not assume a Rust binary is fully static or that outbound HTTPS works in an empty image. Inspect the produced binary and test the real TaskPolicy TLS path in the final runtime image. Run the process as a non-root user when privileged access is unnecessary; Docker's [build best practices](https://docs.docker.com/build/building/best-practices/) also recommend excluding unnecessary packages and using `USER` for unprivileged services.

For multi-architecture delivery, build and test each advertised platform. Emulation can prove packaging but may hide performance and native-library issues; Docker documents emulation, native builders, and cross-compilation as distinct [multi-platform strategies](https://docs.docker.com/build/building/multi-platform/).

## Keep configuration outside the artifact

Build one artifact and supply command-scoped settings at deployment. Run `migrate` with `MIGRATION_DATABASE_URL`; run `serve` with `DATABASE_URL`, `TASK_POLICY_URL`, telemetry settings, and an `HTTP_ADDR` reachable in the selected network.

Do not bake `.env`, credentials, environment names, or collector endpoints into an image. `DEPLOYMENT_ENVIRONMENT` labels telemetry; it does not select hidden application behavior.

The platform owns TLS termination, ingress, DNS, service discovery, network policy, secret delivery, CPU/memory resources, and restart policy. Add those settings to the deployment artifact, not to Domain or Application.

## Probes are routing and restart contracts

Map `/health/live` to liveness and `/health/ready` to readiness:

- liveness must not restart healthy processes merely because MySQL or another dependency is unavailable;
- readiness removes an instance from traffic while its bounded MySQL check fails;
- neither probe proves migration compatibility or TaskPolicy availability;
- a startup probe is conditional platform protection for applications whose legitimate startup exceeds ordinary liveness timing; it does not repair a failing startup.

Kubernetes documents the different consequences of [liveness, readiness, and startup probes](https://kubernetes.io/docs/concepts/workloads/pods/probes/). Configure initial delay, period, timeout, and failure threshold from observed startup and failure behavior rather than copying generic YAML.

## Separate schema release from serving

Run migrations as an explicit release job or equivalent one-shot operation before the new application receives traffic. It needs schema-changing credentials; steady-state `serve` should use a narrower runtime account.

During a rolling deployment, old and new instances may overlap. Separate execution alone does not make a schema change compatible with both versions. Use expand/contract sequencing when overlap exists:

1. expand the schema in a backward-compatible migration;
2. deploy code that can use the expanded shape;
3. migrate/backfill data with an observable process when required;
4. stop old-version use;
5. contract obsolete schema in a later release.

Destructive migrations need a recovery, lock, and compatibility plan specific to their data and platform; the template cannot supply one generically.

## Coordinate termination

The platform must send SIGTERM to the Rust process, stop routing new traffic, and allow more time than `SHUTDOWN_TIMEOUT_SECS` so pool closure and fastrace flushing retain margin. Avoid shell entrypoints that intercept signals without forwarding them.

[`app::server`](../../../app/src/server.rs) treats early server exit, signal installation failure, Axum failure, and drain timeout as process errors. The deployment should surface nonzero exit and failed readiness instead of masking them with an unconditional restart-success wrapper.

Rolling platforms can temporarily run old, new, and terminating instances together. Capacity, database pool budgets, migration compatibility, and telemetry labels must account for that overlap; see the Kubernetes [Deployment rollout behavior](https://kubernetes.io/docs/concepts/workloads/controllers/deployment/) for one concrete platform model.

## Acceptance path for a deployment artifact

Before treating an artifact as supported, verify on the target architecture that it:

1. starts with platform-injected configuration and no embedded secret;
2. runs `migrate` separately and starts `serve` without schema privileges;
3. binds the intended interface and passes both probes;
4. reaches MySQL and the TaskPolicy HTTPS endpoint with the final trust store;
5. executes the canonical CreateTask smoke path through the installed Router;
6. receives SIGTERM directly, drains, closes resources, flushes tracing, and exits zero within the platform grace period;
7. records failures without exposing secret or request content;
8. was built and pushed for the architecture actually scheduled.

Keep this verification beside the concrete deployment artifacts. Do not add Docker/Kubernetes acceptance commands to the baseline Just interface until those artifacts become supported generated output.
