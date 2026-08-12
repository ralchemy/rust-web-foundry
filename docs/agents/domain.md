# Domain Docs

How agents discover, design, review, and preserve this repo's Domain knowledge.

## Before exploring, read these

- **`CONTEXT.md`** at the repo root, or
- **`CONTEXT-MAP.md`** at the repo root if it exists — it points at one `CONTEXT.md` per context. Read each one relevant to the topic.
- **`docs/domain/`** for the relevant capability in a single-context repo, or the context-specific `domain/` directory named by `CONTEXT-MAP.md`.
- **`docs/adr/`** — read ADRs that touch the area you're about to work in. In multi-context repos, also check context-scoped ADR paths named by the relevant context docs.

If any of these files don't exist, **proceed silently**. Don't scaffold them upfront. Create or change Domain docs only when relevant terms, boundaries, or decisions have been confirmed.

## Choose the smallest Domain workflow

| Change | Required Skill |
| --- | --- |
| New capability or unclear actors, terminology, events, rules, or scope | `ddd-discover` |
| Bounded context, semantic ownership, or cross-context translation change | `ddd-strategic-design` |
| Aggregate, invariant, state transition, Domain Service, Domain Event, or business type change | `ddd-tactical-design` |
| Substantial Domain model or implementation reaches completion | `ddd-model-review` |

Skills compose only when the change needs them. A new, unclear capability may require discovery followed by tactical design; changing one confirmed invariant normally requires only tactical design, with model review added when the resulting model or implementation is substantial. Adapter-only or Infrastructure-only work that preserves business meaning needs neither.

## Keep authority explicit

DDD Skills decide business language, semantic ownership, invariants, behaviors, and type candidates. They do not decide crate layout, dependency direction, Port shape, persistence, HTTP contracts, retries, caches, Outbox usage, or deployment. Root and crate `AGENTS.md` files own those engineering decisions.

Record working facts, assumptions, open questions, and the design checkpoint in the active issue or plan. Promote only confirmed, durable knowledge:

- Put confirmed terms and meanings in the relevant `CONTEXT.md`; keep it a glossary, not a specification.
- Put stable capability semantics in one focused Markdown file: `docs/domain/<capability>.md` for a single context, or `docs/contexts/<context>/domain/<capability>.md` for multiple contexts.
- Add `CONTEXT-MAP.md` only when multiple bounded contexts are proven necessary; use it to locate each context and record semantic ownership and relationships.
- Write an ADR only for a durable, non-obvious decision with meaningful alternatives.
- Do not preserve generated stage reports as a second source of truth.

Create a capability document lazily when confirmed rules would otherwise live only in `.scratch/`, chat, or code. Update it in the same change when its durable semantics change. Delete obsolete statements instead of keeping a history; Git and ADRs preserve history.

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
