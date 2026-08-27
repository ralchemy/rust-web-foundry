---
name: ddd-strategic-design
description: Design or revise strategic Domain boundaries. Use before adding or changing a bounded context, business capability ownership, semantic owner, cross-context relationship, shared term with different meanings, or translation boundary.
---

# Design strategic Domain boundaries

Define semantic ownership without turning every capability into a context, crate, or service.

1. Read root `AGENTS.md` and the nearest local `AGENTS.md` for planned artifact changes. Match the applicable action-first Context Pointers, record or reuse the active plan's Context Set, and load each canonical owner once. Confirm that the Domain branches select `docs/agents/domain.md#before-exploring-read-these` and `docs/agents/domain.md#choose-the-smallest-domain-workflow`; do not read the whole Domain owner by default.
2. Read the confirmed glossary, context map, capability documents, ADRs, discovery evidence, and relevant implementation selected by those owners. Separate facts from assumptions and open questions.
3. Identify business capabilities and the decisions or lifecycle each capability owns. Classify Core, Supporting, or Generic only when the classification changes investment, ownership, or sourcing.
4. Default to one bounded context. Split only when evidence shows different meanings, policies, lifecycles, ownership, or an unavoidable translation boundary.
5. For each necessary context, define responsibilities, non-responsibilities, ubiquitous language, semantic owner, and information it may expose. Do not infer a boundary from team structure or tables alone.
6. For each cross-context relationship, name the upstream and downstream semantic owners, the business contract, and any required translation or Anti-Corruption Layer. Leave protocols and failure handling to Project Rules.
7. Record unresolved material questions in the active issue or plan. Before persisting confirmed context relationships, route and add `docs/agents/domain.md#keep-authority-explicit` to the Context Set; write an ADR only for a durable, non-obvious choice.

Produce a concise decision with supporting evidence:

- **Boundary decision**: keep one context or split, and why.
- **Capabilities**: capability, owned decisions, optional strategic classification.
- **Contexts**: responsibility, non-responsibility, language, semantic owner.
- **Relationships**: direction, business contract, translation need.
- **Open questions**: impact and confirmer.

Do not prescribe microservices, crate trees, Ports, repositories, transports, Outbox, retries, caching, or deployment. Do not use target counts for subdomains, contexts, collaboration points, or failure modes.
