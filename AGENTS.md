# Project Rules

## Authority

- Root and nearest-local `AGENTS.md` files are the standing authority for governance, path responsibility, and hard protection.
- `docs/agents/context-routes.tsv` is the single routing catalogue for conditional engineering context. Guide and Domain documents reached from it remain their own canonical owners; the catalogue only decides when they are needed.
- Source, manifests, tests, Just recipes, and CI own executable facts. If a standing rule, routed owner, and executable fact conflict, stop and report the authority seam instead of choosing the convenient interpretation.
- Optional Skills and orchestration frameworks are procedures only. They never override repository authority.

## Context contract

- Before a production edit, the active context contains one bounded Context Pack whose Context Set covers every planned touched path and applicable action. `scripts/compile-agent-context.sh` is the repository provider; reuse a current pack when its recorded refs remain current and cover the work.
- `bash scripts/compile-agent-context.sh --list-actions` exposes the finite action vocabulary. An action applies only when the task performs the described change; topic similarity is insufficient.
- When planned or touched paths, action classes, or recorded source SHAs change, rebuild the pack and read only newly selected `ref@content-sha` entries. A pack is a generated read view, never a second authority.
- After reading the Pack, keep a short path-and-proof checklist; update it with scope.

If the compiler is unavailable, perform the same selection manually from `docs/agents/context-routes.tsv`: root rules + nearest-local rules for every touched path + each selected routed owner, each unique source once. Do not replace deterministic selection with broad Guide reads.

The default compiled-context ceiling is 60,000 UTF-8 bytes. If a coherent implementation slice exceeds it, split the task at a stable dependency boundary rather than raising the ceiling for convenience.

## Change contract

- A business-meaning change resolves affected actors, terms, rules, and exceptional outcomes in their Domain owner before production editing; business-meaning-neutral changes proceed without Domain modeling.
- Material business facts absent from the request or an authoritative Domain artifact remain unresolved. A feature name does not define enum members, defaults, permissions, temporal rules, idempotency, or exceptional behavior.
- When the nearest public path exposes such a fact, return `needs-decision` before routed context or later Skills.
- New files, abstractions, and dependencies reuse the owners on the nearest existing public path unless the requested behavior establishes a new responsibility.
- Before changing a shared interface or Port, account for every caller and implementation.
- A behavior change carries the smallest regression proof through its public seam.

## Hard protection

- Preserve inward workspace dependency direction and the responsibility split declared by local `AGENTS.md` files. A preference for fewer files, types, or abstractions never overrides an owned business type or boundary.
- Do not delete, weaken, or move an existing assertion out of its exercised path merely to pass a gate. Reproduce unexpected failures on the untouched baseline or show that requested behavior intentionally replaces the old contract.
- Completion evidence for a gate changed by the current work includes its complete diff and retained proof. The caller or active workflow decides whether a separate review is required.
- Fix violations introduced or exposed in the touched responsibility. Record unrelated material debt without expanding scope unless it blocks required verification.
- Guide, Domain authority, context routing, budgets, and governance checks are read-only during ordinary implementation. Changing them requires explicit Governance or Documentation scope and the `governance` action.

## Verification and handoff

- Production-code completion evidence includes the smallest focused owning test and `just check`; it also includes `just verify` when configuration, migrations, lifecycle, checked SQL, composition, or installed routes changed.
- Check evidence records the command, exit status, failing test or first root cause, and log path. Successful logs remain outside the prompt.
- An implementation handoff, when produced, carries a compact Evidence Pack: goal, Context Pack identity, changed paths, material decisions, public acceptance path, checks with outcomes, and unresolved blockers.
- When a caller or active workflow requests conformance review, the reviewer receives the request, Context Pack, Evidence Pack, and complete diff; it reloads source authority only when a recorded SHA is stale or the diff reveals an unclassified path/action. The implementation session does not replay all governance and Guide material before handoff.
- A frozen Review Batch is evidence-only. Before classifying findings, fixing, handing off, or completing, load its stable Standards and Spec artifacts with `scripts/load-review-reports.sh` and require a visible, untruncated `reports_loaded: 2/2` for one batch/snapshot. Missing evidence makes the review incomplete.
- A reviewed handoff retains all findings plus reviewed and post-review diff identities.

## Optional agent tooling

- Repository Skills may automate planning, DDD outcomes, endpoint work, migrations, or review, but they must consume the same compiled Context Pack protocol rather than preload broad Guide chapters.
- Load an optional Skill only when execution reaches its explicit run, use, or invoke step.
- A subagent receives the goal, relevant evidence, current Context Set, and the smallest source slice required for its assignment, not the parent agent's discovery transcript or full conversation history.
