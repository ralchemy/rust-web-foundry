---
name: ddd-discover
description: Discover and clarify a business capability before implementation. Use when a capability is new or its actors, terminology, commands, domain events, rules, scope, or exceptional outcomes are unclear or not confirmed in the project's Domain docs.
---

# Discover a Domain capability

Establish evidence-backed business understanding. Do not design code or infrastructure.

1. Read root `AGENTS.md` and the nearest local `AGENTS.md` for any planned artifact changes. Match the applicable action-first Context Pointers, record or reuse the active plan's Context Set, and load each canonical owner once. Confirm that the Domain exploration and workflow branches select `docs/agents/domain.md#before-exploring-read-these` and `docs/agents/domain.md#choose-the-smallest-domain-workflow`; do not read the whole Domain owner by default.
2. Read the relevant confirmed `CONTEXT.md` or `CONTEXT-MAP.md`, capability documents, ADRs, and active issue or requirement identified by that owner.
3. State the business value, goals, non-goals, actors, and constraints supported by the available evidence.
4. Walk concrete timelines. For each scenario, identify the actor's intent, preconditions, business command, past-tense Domain fact, outcome, and material exception.
5. Classify every material statement as **Fact**, **Assumption**, or **Open Question**. Cite its source or name who must confirm it. Never promote an assumption by repeating it.
6. Identify terminology conflicts, rule and invariant candidates, missing decisions, and possible semantic boundaries. Treat boundaries as clues, not conclusions.
7. Ask only questions whose answers change business behavior, ownership, or acceptance. Do not invent scenarios to meet a quota.

Produce only the sections that add information:

- **Scope**: business value, goals, non-goals, constraints.
- **Scenario timeline**: actor, intent, precondition, command, Domain fact, outcome.
- **Language**: term, meaning, status, source, conflicting term.
- **Rules and questions**: rule candidate, evidence, uncertainty, confirmer.
- **Handoff**: call `ddd-strategic-design` only for a real semantic-boundary decision; call `ddd-tactical-design` when invariants, states, or business types must be designed.

Keep assumptions and open questions in the active issue or plan. Before persisting confirmed terms or capability semantics, route and add the matched `docs/agents/domain.md#keep-authority-explicit` or `#capability-document` owner to the Context Set. Do not choose crates, APIs, tables, messaging, retries, or other technical mechanisms.
