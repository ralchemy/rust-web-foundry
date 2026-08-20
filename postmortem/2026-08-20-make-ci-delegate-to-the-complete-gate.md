# Make CI delegate to the complete public gate

## Decision

Make `just ci` own the idiomatic-Rust boundary check and the full lifecycle harness. Template and generated-service workflows provide tools and MySQL, then invoke only `just ci`.

## Why

The intended quality contract named `just ci` as the complete remote equivalent of local `just verify`, but the implementation left the idiomatic-Rust check in workflow YAML and omitted the drain/timeout lifecycle proof from the public gate. This let local and remote callers of the same command prove less than the workflow name promised, while duplicating one check in CI-specific wiring.

Keeping the proof in Just gives local and remote execution one owner. `just verify` still differs only by starting and stopping Compose; GitHub Actions supplies the equivalent MySQL 8.4 service.

## Rejected alternatives

- Add lifecycle commands directly to both workflows: that would duplicate gate composition and allow local and remote proof to drift again.
- Leave the script as a workflow-only step: `just check`, `just ci`, and `just verify` would continue to omit a documented database-free boundary check.
- Make CI run `just verify`: GitHub already owns its MySQL service, so starting Compose would duplicate infrastructure rather than proof.
