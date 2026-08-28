# Project Rules

## Authority

- Root and nearest-local `AGENTS.md` files are the standing authority for governance, path responsibility, and hard protection.
- `docs/agents/context-routes.tsv` is the single routing catalogue for conditional engineering context. Guide and Domain documents reached from it remain their own canonical owners; the catalogue only decides when they are needed.
- Source, manifests, tests, Just recipes, and CI own executable facts. If a standing rule, routed owner, and executable fact conflict, stop and report the authority seam instead of choosing the convenient interpretation.
- Optional Skills and orchestration frameworks are procedures only. They never override repository authority.

## Compile context before implementation

Before the first production edit:

1. name the smallest planned touched paths;
2. classify only the applicable action keys from `docs/agents/context-routes.tsv`;
3. compile a bounded Context Pack with `bash scripts/compile-agent-context.sh --path <path> [--path ...] --action <key> [--action ...] --goal '<goal>'`;
4. read the generated pack once and use its `path[#anchor]@content-sha` Context Set as the read ledger for the implementation stage.

Use `bash scripts/compile-agent-context.sh --list-actions` to inspect the finite action vocabulary. Add an action only when the task actually performs the described change; topic similarity is not enough. When planned or touched paths or action classes expand, rebuild the pack and read only newly selected `ref@content-sha` entries. A pack is a generated read view, never a second authority.

If the compiler is unavailable, perform the same selection manually from `docs/agents/context-routes.tsv`: root rules + nearest-local rules for every touched path + each selected routed owner, each unique source once. Do not replace deterministic selection with broad Guide reads.

The default compiled-context ceiling is 60,000 UTF-8 bytes. If a coherent implementation slice exceeds it, split the task at a stable dependency boundary rather than raising the ceiling for convenience.

## Before production edits

- Complete every routed Domain workflow outcome and design checkpoint before production edits. Domain modeling is required only when business meaning changes; choose the smallest applicable outcome.
- Keep material business facts not confirmed by the request or an authoritative Domain artifact unresolved. Do not infer enum members, defaults, permissions, temporal rules, idempotency, or exceptional behavior from a feature name.
- Trace the nearest existing public path and reuse its owners before adding files, abstractions, or dependencies.
- Drive behavior through a public seam and add the smallest regression test that would fail if the requested behavior broke.

## Hard protection

- Preserve inward workspace dependency direction and the responsibility split declared by local `AGENTS.md` files. A preference for fewer files, types, or abstractions never overrides an owned business type or boundary.
- Do not delete, weaken, or move an existing assertion out of its exercised path merely to pass a gate. Reproduce unexpected failures on the untouched baseline or show that requested behavior intentionally replaces the old contract.
- A gate changed by the current work is not completion evidence until its complete diff and retained proof have been reviewed.
- Fix violations introduced or exposed in the touched responsibility. Record unrelated material debt without expanding scope unless it blocks required verification.
- Guide, Domain authority, context routing, budgets, and governance checks are read-only during ordinary implementation. Changing them requires explicit Governance or Documentation scope and the `governance` action.

## Verification and handoff

- Run the smallest focused test while editing, then `just check` before implementation handoff. Run `just verify` when configuration, migrations, lifecycle, checked SQL, composition, or installed routes changed.
- Keep full command logs outside the prompt when tooling permits. Carry forward the command, exit status, failing test or first root cause, and the smallest useful excerpt; do not repeatedly inject successful build output.
- Implementation handoff produces a compact Evidence Pack: goal, Context Pack identity, changed paths, material decisions, public acceptance path, checks with outcomes, and unresolved blockers. Do not copy routed owner prose or discovery transcripts into it.
- Final conformance review is a fresh review stage. The reviewer receives the request, Context Pack, Evidence Pack, and complete diff; it reloads source authority only when a recorded SHA is stale or the diff reveals an unclassified path/action. Do not make the implementation session replay all governance and Guide material before handoff.

## Optional agent tooling

- Repository Skills may automate planning, DDD outcomes, endpoint work, migrations, or review, but they must consume the same compiled Context Pack protocol rather than preload broad Guide chapters.
- A subagent receives the goal, relevant evidence, current Context Set, and the smallest source slice required for its assignment, not the parent agent's discovery transcript or full conversation history.
