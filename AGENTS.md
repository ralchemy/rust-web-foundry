# Project Rules

## Authority and instruction discovery

- Before modifying a crate or directory, read the nearest nested `AGENTS.md`; do not rely on runtime discovery of descendant instructions.
- Root and nearest-local `AGENTS.md` files are the standing authority for governance, instruction discovery, scope responsibility, and retained hard protection. Optional Skills, extensions, orchestration frameworks, and style preferences are subordinate; a preference for fewer files or types never overrides a required responsibility split or business type.
- Guide and Reference chapters are conditional owners reached through Context Pointers. Ordinary implementation treats them as read-only; only an active issue or specification with explicit Governance or Documentation scope may change them or their routing.

## Context routing

Read root first and the nearest local brief for every touched path. Match every applicable root and nearest-local branch, take their union, and load each target once:

```text
matched_context = unique(root_matches ∪ nearest_local_matches)
```

This is union, not first-match or nearest override. A Pointer routes to its owner; it does not cache or summarize the owner's body. If a standing brief, canonical owner, and executable fact conflict, stop and report the authority seam rather than choosing the convenient interpretation. Use action-first Pointers of the form `<Changing|Adding|Accepting|Handling|Tuning|Spawning|Deploying> <exact branch> → read <repo-root-relative owner>[#anchor]`.

Changing `AGENTS.md`, Guide authority, Context Pointer routing, or generated governance checks → read `docs/guide/development.md#governance-and-documentation-changes`.
Changing crate placement, top-level source layout, module shape, or responsibility directories → read `docs/guide/architecture/project-structure.md`.
Changing a workspace dependency edge, composition-root boundary, or responsibility ownership between crates → read `docs/guide/architecture/README.md`.
Changing a cross-crate error category, concrete-failure ownership, or public error mapping → read `docs/guide/architecture/error-handling.md`.
Adding or changing authentication, authorization, sessions, bearer credentials, passwords, or identity-provider integration → read `docs/guide/reference/authentication-and-authorization.md`.
Accepting a new untrusted input or destination, handling a secret, or changing network exposure → read `docs/guide/security.md`.
Changing public-path acceptance evidence, test placement, a public Just/CI/architecture/acceptance gate, or other cross-crate evidence → read `docs/guide/testing.md`.
Changing a public workflow, Domain behavior, Port, database schema or index, persistence contract, or external integration → read `docs/guide/development.md#design-checkpoint`.
Implementing a confirmed design through multiple dependent tasks → read `docs/guide/development.md#coherent-implementation-slices`.
Adding or replacing a dependency → read `docs/guide/development.md#dependency-selection`.
Changing a semantic type, parser, formatter, boundary conversion, database row, downstream wire type, conversion trait, or public business naming contract → read `docs/guide/reference/idiomatic-rust.md`.
Tuning performance, caching, compression, streaming, pools, blocking work, benchmarks, or profiling → read `docs/guide/reference/performance.md`.
Spawning work, selecting futures, adding a timeout around mutating work, adding a channel loop, periodic/background work, long-lived stream, or listener → read `docs/guide/reference/async-and-cancellation.md`.
Changing an outbound HTTP timeout, retry, idempotency, redirect, response limit, or resilience policy → read `docs/guide/reference/outbound-http.md`.
Adding protobuf, gRPC, Tonic, or streaming RPC contracts → read `docs/guide/reference/grpc.md`.
Adding a Dockerfile, deployment manifest, image/release workflow, release migration sequence, or platform probe configuration → read `docs/guide/reference/deployment.md`.
Changing business meaning, introducing an unclear capability, changing a bounded-context translation, or materially changing a Domain model → read `docs/agents/domain.md`.

## Before production edits

- Before the first production edit, complete every applicable Domain workflow outcome and design checkpoint. Domain modeling is required when business meaning changes, but not for every change; use the smallest applicable outcome. An external workflow's completion never replaces repository evidence.
- Keep material business facts that are not confirmed by the user request or an authoritative Domain artifact unresolved. Do not implement them; stop only the affected path and report the smallest missing decision.
- Keep every named repository gate performing the proof documented by `docs/guide/testing.md`. When changing `AGENTS.md`, a public Just command, CI, or an architecture or acceptance script, report old proof, new proof, and equivalent retained coverage separately. A gate changed by the current work is not completion evidence until its full diff and retained proof have been reviewed.

## Completion

- Drive behavior through public paths and add the smallest regression test that would fail if the behavior broke.
- Before handoff, perform one concise, evidence-backed conformance review; a generic "reviewed" is not evidence. Name: the applicable root/local `AGENTS.md` rules, design checkpoint, and Domain outcome; module and type decisions plus conversion boundaries, including any retained primitive or module-shape exception; one real public path through touched layers including each changed mutation's Domain owner and persistence reconstruction seam or read-model projection; every acceptance item mapped to a named executable test or check; the complete diff, required documentation, and changed gates; and focused tests plus `just check` (also `just verify` when configuration, migrations, lifecycle, or installed routes changed).
- Do not delete, weaken, or move an existing assertion out of its exercised path merely to pass a gate. Reproduce an unexpected failure on the untouched baseline or show that the requested behavior intentionally replaces the old contract. Report pre-existing failures separately; repair them at their owner only when they block required verification, preserving or strengthening proof.
- Fix violations introduced or exposed in the touched responsibility. Record unrelated material debt without expanding scope unless it blocks the requested work.

## Optional agent tooling

- Root/local rules, tracked Guide and Domain documents, and source, manifests, tests, Just recipes, and CI remain authoritative independently of any agent framework.
- The tracked `docs/agents/domain.md` is the framework-neutral owner of Domain workflow outcomes; a compatible Skill may automate that outcome but is not a mandatory procedure.
