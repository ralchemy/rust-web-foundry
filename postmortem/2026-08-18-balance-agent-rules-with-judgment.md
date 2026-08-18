# Balance agent rules with implementation judgment

## Decision

Keep dependency direction, layer ownership, typed business interfaces, adapter-owned conversion, security rules, and objective checks mandatory. Related command workflows that can evolve or be tested independently must share a capability directory; inseparable operations may remain together with a recorded reason. Pre-implementation Domain design remains required, while an isolated spike may precede it to resolve a named uncertainty.

Require one evidence-backed conformance review at handoff. Internal task boundaries need only the focused check that protects the current coherent buildable slice. Fix violations introduced or exposed in the touched responsibility; record unrelated legacy debt without absorbing it.

External implementation Skills may organize execution but do not replace repository preconditions or evidence. Keep the documented meaning of named repository gates stable during feature work; when governance itself changes, review the old and new proof separately before treating the changed gate as evidence.

For semantic ownership, require the handoff to trace a real production path rather than merely point at a Domain type: changed mutations invoke their named Domain owner, persistence reconstructs that owner at the adapter boundary, promised atomicity is representable by the Port, and configured composition installs the selected adapter. Read-only projections remain free to select only the fields they need.

Remove the hard Application root file-placement check from `just architecture`. Keep the dependency, forbidden-framework, and top-level multi-entry-point checks because they test narrow objective facts rather than inferring semantic ownership from a path.

## Why

The stricter rules improved type completeness and capability-directory structure in Pi Goal runs, but repeated eight-part audits and per-task full gates shifted attention from business design to compliance text. Cross-layer signature changes also form one buildable slice and should not need temporary compatibility scaffolding merely to make each internal task compile alone.

A clean regression with the directory rule phrased only as a default produced three related top-level Command files and then wrote a design checkpoint claiming they were in a capability directory. Strong types and all checks passed, so neither compilation nor the final self-review caught the contradiction. The directory shape is therefore a required semantic result with a narrow inseparable-workflow exception, while the execution procedure and audit frequency remain relaxed.

A second clean regression restored only that semantic result while keeping the relaxed execution procedure. Pi Goal with LiteLLM `gpt-5.6-luna` at `max` and Ponytail full recorded one design checkpoint, produced `temporary_access_grants/{mod.rs,request.rs,approve.rs,revoke.rs}`, kept Application inputs typed, and passed an independent `just check`. This is the intended balance: constrain durable code outcomes, not the agent's internal task choreography.

A paired Goal run then exposed two remaining semantic loopholes. With a detailed user contract, the agent created the capability directory and typed model but filled it with same-shaped pass-through use cases, used unchecked `MySqlRow` queries and `SELECT *`, and left a stale Port name in the design checkpoint. With only “request, approve, revoke, query,” it invented a 24-hour maximum, lifecycle, rejection, authentication scope, and idempotency behavior, then labelled those facts confirmed instead of asking. The rules now define those outputs as failures without prescribing the agent framework's task loop.

Repeating the vague prompt after the rule change produced the intended repository behavior: the agent recorded the missing actors, authorization target, permission set, lifecycle, validity, repeat-operation semantics, query scope, and HTTP contract as `needs-info`; it made no tracked production change and `just check` passed on the untouched baseline. Pi Goal nevertheless kept the goal active and repeated the same blocker until the run was stopped. That termination loop belongs to the agent framework; adding Pi-specific goal-state instructions to the repository would couple code governance to one runtime without improving the generated code.

A fully specified prompt produced strong Domain types, a capability directory, private checked SQL rows, SQLx metadata, a projection that excluded the large blob, a real production composition test, and a passing `just verify`. It also exposed a conflict in the rules: the Application guidance told a command module to own a Domain decision while the transaction guidance required the locked adapter to reconstruct the Aggregate and invoke that decision atomically. During final review the agent loaded and preflighted the Aggregate in Application, then invoked the same transition again inside Infrastructure. The rules now assign command orchestration and the atomic Port contract to Application, the decision to Domain, and its single invocation under the required lock to Infrastructure. A short use case that coordinates an independent clock with that atomic operation is substantive; duplicating the decision is not.

The same run copied audit evidence from request into approve and revoke even though the confirmed command payloads excluded it, and preserved an old Router signature by installing production `Noop` capabilities behind a second constructor. Command contracts are now explicitly independent, and HTTP has one production Router constructor whose callers must change together. These are code-governance rules, not Pi Goal procedure.

The same fresh-generation verification exposed two older acceptance-test defects. The delayed Policy case ran after its stub had been stopped, so it could not prove timeout behavior, and `_accept` contained stray block-file setup from the separate cancellation test: without a file it failed as unbound, while creating the file would deadlock the successful smoke request until timeout. The fix retains every assertion, runs the delayed case while the stub is alive, and keeps block-file behavior only in `lifecycle` where it is intentionally released.

File placement and Domain adequacy require judgment. Hard-checking the current Application root layout would freeze an implementation detail and reject future responsibility directories without proving an architecture violation. The remaining gates protect boundaries with low ambiguity while Project Rules state the preferred shape and require a concrete reason for exceptions.

## Rejected alternatives

- Remove the type and module rules: returns to primitive-heavy, flat generated code.
- Remove every architecture gate: prose alone already failed to prevent objective dependency and multi-workflow violations.
- Permit undocumented exceptions: makes defaults optional in practice and leaves reviewers unable to distinguish judgment from drift.
