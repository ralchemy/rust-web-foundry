# Domain Docs

How agents discover, design, review, and preserve this repo's Domain knowledge.

## Before exploring, read these

- **`CONTEXT.md`** at the repo root, or
- **`CONTEXT-MAP.md`** at the repo root if it exists — it points at one `CONTEXT.md` per context. Read each one relevant to the topic.
- **`docs/domain/`** for the relevant capability in a single-context repo, or the context-specific `domain/` directory named by `CONTEXT-MAP.md`.
- **`docs/adr/`** — read ADRs that touch the area you're about to work in. In multi-context repos, also check context-scoped ADR paths named by the relevant context docs.

If any of these files don't exist, **proceed silently**. Don't scaffold them upfront. Create or change Domain docs only when relevant terms, boundaries, or decisions have been confirmed.

## Choose the smallest Domain workflow

| Change | Required outcome | Optional Skill |
| --- | --- | --- |
| New capability or unclear actors, terminology, events, rules, or scope | Confirm the capability's actors, language, commands, facts, rules, scope, and exceptions before implementation | `ddd-discover` |
| Bounded context, semantic ownership, or cross-context translation change | Establish the owner, relationship, published facts, and translation boundary before implementation | `ddd-strategic-design` |
| Aggregate, invariant, state transition, Domain Service, Domain Event, or business type change | Define behavior ownership, legal states, invariants, transitions, events, and type candidates before implementation | `ddd-tactical-design` |
| Substantial Domain model or implementation reaches handoff | Review language, ownership, invariants, state safety, type completeness, boundary leakage, and meaningful acceptance evidence | `ddd-model-review` |

These outcomes compose only when the change needs them. A new, unclear capability may require discovery followed by tactical design; changing one confirmed invariant normally requires only tactical design, with model review added when the resulting model or implementation is substantial. Existing confirmed Domain documents may satisfy discovery. Adapter-only, Infrastructure-only, documentation-only, and small corrective work that preserves business meaning needs no Domain workflow.

An agent with repository Skill support should use the smallest applicable matching `ddd-*` Skill as the procedure. An agent without Skill support must perform the same workflow directly from this document and the authoritative Domain artifacts. Skills are optional execution aids; the outcomes and evidence are not optional.

Complete the selected pre-implementation workflow and the applicable [design checkpoint](../guide/development.md#design-checkpoint), then record their results in the active issue or plan before writing the production implementation. A focused failing test or isolated throwaway spike may precede it only to resolve a named uncertainty; remove or replace that code once the uncertainty is resolved. Writing a capability document or running model review after the implementation is chosen cannot substitute for this ordering.

## Keep authority explicit

Domain workflow outputs and tracked Domain documents own business understanding: language, capabilities, semantic ownership, boundaries, invariants, behaviors, and business type candidates. Root and nearest-local `AGENTS.md` files own standing governance, scope responsibility, and retained hard protection. A Guide chapter reached through a matching Context Pointer owns its conditional engineering contract. Source, manifests, tests, Just recipes, and CI own executable facts. A Domain output cannot override these engineering owners or mandate crate layout, dependencies, Ports, adapters, persistence, transport, resilience, or runtime design.

Record working facts, assumptions, open questions, selected Domain workflow evidence, and the design checkpoint in the active issue or plan. Every material fact marked confirmed must cite the user's request or an authoritative Domain artifact; plausible inference has no authority. Do not infer enum members, ranges, defaults, temporal constraints, permissions, exceptional behavior, or idempotency semantics from a feature name or illustrative example. Record each command's fields and rules independently rather than inheriting them from a sibling operation.

Keep every unsupported fact as an open question or explicit hypothesis. Ask for the smallest missing decision and stop only the affected implementation path when that decision changes the model, public behavior, or external contract. Do not promote an unresolved fact to a capability document or production implementation. Promote only confirmed, durable knowledge:

- Put confirmed terms and meanings in the relevant `CONTEXT.md`; keep it a glossary, not a specification.
- Put stable capability semantics in one focused Markdown file: `docs/domain/<capability>.md` for a single context, or `docs/contexts/<context>/domain/<capability>.md` for multiple contexts.
- Add `CONTEXT-MAP.md` only when multiple bounded contexts are proven necessary; use it to locate each context and record semantic ownership and relationships.
- Write an ADR only for a durable, non-obvious decision with meaningful alternatives.
- Do not preserve generated stage reports as a second source of truth.

Create a capability document lazily when confirmed rules would otherwise live only in `.scratch/`, chat, or code; the ignored `.scratch/` tree must not be their only record. Update it in the same change when its durable semantics change. Delete obsolete statements instead of keeping a history; Git and ADRs preserve history.

## Capability document

Record only sections supported by confirmed business knowledge:

- **Purpose and scope**: business outcome, responsibilities, and non-responsibilities.
- **Model**: Aggregate or behavior owner and the business meaning of its Entity and Value Object types.
- **Invariants**: rule, trigger, owner, and violation outcome.
- **States and behaviors**: legal transitions, rejected transitions, and impossible combinations.
- **Domain facts**: past-tense business facts and their business meaning; omit delivery mechanics.
- **Scenarios**: the smallest concrete examples that disambiguate the rules.

Do not copy working assumptions, implementation tasks, HTTP contracts, database schemas, Port signatures, retries, caches, Outbox design, or deployment details into this document. Keep it concise and split it only when the file owns multiple independently named capabilities.

## File structure

Single-context repo (most repos):

```
/
├── CONTEXT.md
└── docs/
    ├── domain/
    │   └── permission-request.md
    └── adr/
        └── 0001-event-sourced-orders.md
```

Multi-context repo (presence of `CONTEXT-MAP.md` at the root):

```
/
├── CONTEXT-MAP.md
└── docs/
    ├── adr/                           ← system-wide decisions
    └── contexts/
        ├── ordering/
        │   ├── CONTEXT.md
        │   ├── domain/
        │   │   └── place-order.md
        │   └── adr/                   ← context-specific decisions
        └── billing/
            ├── CONTEXT.md
            ├── domain/
            │   └── issue-invoice.md
            └── adr/
```

## Use the glossary's vocabulary

When your output names a domain concept (in an issue title, a refactor proposal, a hypothesis, a test name), use the term as defined in `CONTEXT.md`. Don't drift to synonyms the glossary explicitly avoids.

If the concept you need isn't in the glossary yet, either reconsider the invented language or use `ddd-discover` to resolve a real gap before adding it.

## Flag ADR conflicts

If your output contradicts an existing ADR, surface it explicitly rather than silently overriding:

> _Contradicts ADR-0007 (event-sourced orders) — but worth reopening because…_
