# Reference complexity budget

Reference code exists to teach decisions that are difficult to infer reliably from a minimal service. Repository size is not the goal.

A new reference pattern is accepted only when all of these are true:

1. it demonstrates a materially different engineering or Domain decision not already covered by the reference index;
2. the behavior is stated explicitly as illustrative rather than a default user-domain rule;
3. it has an executable or reviewable proof at the narrowest useful seam;
4. it is discoverable through the short reference index without requiring global preloading;
5. it does not add a framework, DDD pattern, or infrastructure mechanism solely for completeness.

Prefer one dense vertical slice over several same-shaped CRUD examples. Remove or merge a reference when its teaching value is duplicated.

## Patterns intentionally deferred

Domain Services, Domain Events, CQRS, event sourcing, Outbox, Saga/process managers, Specifications, plugin architectures, and generic transaction abstractions are not baseline reference requirements. Add one only after a real task demonstrates a recurring decision that existing examples do not answer.

## Evaluation

Reference expansion is successful only if it improves delivery quality at acceptable context cost. Measure semantic/review findings and human correction alongside token/read cost; do not optimize repository or prompt size in isolation.
