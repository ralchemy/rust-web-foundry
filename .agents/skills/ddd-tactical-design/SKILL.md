---
name: ddd-tactical-design
description: Design or revise a tactical Domain model before implementation. Use when adding or materially changing an Aggregate, Entity, Value Object, invariant, state transition, Domain Service, Domain Event, business Policy, or dedicated business type.
---

# Design a tactical Domain model

Make business rules explicit and invalid states difficult or impossible to represent.

1. Read the confirmed glossary, context map, capability documents, discovery or strategic output, root rules, relevant crate rules, and the existing model. Do not replace established language without evidence.
2. Express each business rule as an invariant, including its trigger, owner, valid outcome, and violation behavior.
3. Build a type map. Give a dedicated Domain or Application type to values with distinct business meaning, an invariant, a finite set, a unit, a trust distinction, or same-primitive confusion. Let a Value Object own its intrinsic rules; let Entity or Aggregate behavior own rules relating multiple values or states. Leave free-form display text and genuinely opaque payloads primitive when they have no independent rule.
4. Assign behavior to the Entity or Value Object that owns the state and rule. Use a Domain Service only for a pure Domain rule with no natural Entity or Value Object owner. Use a Factory only when creation itself is a coherent, non-trivial Domain responsibility.
5. Propose an Aggregate boundary only when business invariants require an atomic consistency owner. Identify the root and references, and explain the boundary; do not derive it from ORM relations or a size quota.
6. Define legal transitions, rejected transitions, and impossible state combinations. Prefer explicit states and behaviors over public mutation, string comparisons, or boolean flag combinations.
7. Keep fact acquisition separate from business decisions. Application or an adapter may obtain current time, roles, or external facts, but supply them as typed values to Domain behavior or a pure Domain Policy when they participate in a business rule.
8. Name Domain Events as past-tense business facts emitted by Domain behavior. Do not attach transport, delivery, replay, or Outbox policy to them.
9. Define acceptance scenarios that prove the invariants. Persist confirmed, durable semantics in the capability document defined by `docs/agents/domain.md`; keep assumptions and open questions in the active issue or plan.
10. If implementation is requested, hand this model to the existing Project Rules instead of inventing a parallel architecture.

Produce the smallest useful model:

- **Invariants**: rule, trigger, owner, violation behavior.
- **Type map**: business value, owning type, invariant or distinction, raw conversion seams.
- **Behavior and states**: operation, pre-state, result, rejected states, emitted fact.
- **Aggregate decision**: boundary and evidence, or why no Aggregate change is needed.
- **Acceptance and questions**: business examples plus unresolved material questions.

Do not design repository interfaces, Ports, SQL rows, HTTP DTOs, retries, compensation, locking, or event delivery. Do not put HTTP, database, configuration, or downstream encodings into Domain types.
