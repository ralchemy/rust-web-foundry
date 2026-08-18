# Keep agent rules framework-neutral

> Superseded in part by `2026-08-18-balance-agent-rules-with-judgment.md`: the Application root file-placement gate was removed; dependency, framework-boundary, and multi-entry-point checks remain.

## Decision

Project Rules define required code structure, Domain outcomes, task-boundary completion, and stopping conditions without depending on a Goal implementation, orchestration framework, or mandatory Skill runtime. Repository Skills remain optional procedures that produce the same documented outcomes.

Confirmed multi-task designs may run autonomously: each dependency-ready task must pass its own design, implementation, acceptance, checks, and review boundary before the next task begins, but no human confirmation is required between passing tasks.

Executable gates remain narrow. `just architecture` checks the fixed dependency graph, forbidden outer-framework dependencies in Domain and Application, keeps Application source inside its responsibility directories, and enforces one objective module-shape threshold: a top-level use-case file cannot expose more than one public `execute` entry point. Type completeness, naming, DDD ownership, and other semantic module boundaries stay in Project Rules and review because mechanical checks would create false positives.

Project Rules explicitly override optional framework, extension, and Skill style defaults during repository work. Completion requires an evidence-backed conformance audit; an orchestration framework's completed status or a generic self-review is not repository evidence.

## Why

Generated projects may be implemented by different coding-agent frameworks. Binding correctness to one framework's Goal or Skill protocol makes the template less portable, while prose alone cannot reject an objectively invalid dependency graph. Separating portable outcomes from optional procedures and narrow executable facts gives every framework the same acceptance boundary without prescribing its control loop.

A local Pi Goal validation exposed why both additions are necessary: the agent read root and nested instructions, preserved dependency boundaries, and completed every task, but an always-on fewer-files preference won over the Application workflow split, and a generic final audit missed constrained primitives, unused public variants, and duplicated tests. The session used about 43K live context in a 1.05M window, so instruction competition and weak completion evidence—not compaction—explained the misses.

A second clean run read the strengthened rules, performed tactical design before source edits, and created the missing `RevocationReason`, but still returned three public `execute` workflows in one file and raw `&str`/`u16` Application inputs after `just check` passed. The generated baseline itself taught `CreateTask::execute(String)`, so its executable example contradicted the type rule. The fix aligns that example with adapter-owned conversion and adds the one structural check whose false-positive boundary is explicit.

A third run confirmed the type correction: every Application workflow accepted Domain types. It also exposed a remaining wording loophole by placing the three related commands in separate top-level files. Project Rules now require a capability directory from the start when several related commands are already confirmed; the per-file gate remains a narrow backstop rather than pretending to infer semantic relationships between filenames.

The fourth run produced the intended capability directory and strongly typed Application interfaces while the conflicting fewer-files extension remained active. It did not record its design checkpoint before source edits, so prose plus Goal continuity still cannot guarantee every semantic workflow step. Repository checks can reject objective structure and dependency violations; ordering, naming, and Domain adequacy still require the explicit evidence audit and human or agent review.

## Rejected alternatives

- Ship a project-specific implement or Goal Skill: couples generated repositories to one orchestration model.
- Require human confirmation after every task: prevents safe unattended implementation of an already confirmed design.
- Turn file length, primitive use, naming, DDD ownership, or general module depth into hard scripts: these require semantic judgment and would reward code written to satisfy heuristics.
