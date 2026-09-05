# Concurrency-sensitive mutation reference

This reference describes the next Task write slice and the responsibility boundary it must demonstrate. The Task rules are illustrative. A generated project adopts the pattern only when its own requirement establishes a concurrency-sensitive invariant.

## Reference behavior

For the reference Task only:

- `start` is legal only from `pending`;
- the caller supplies the revision it observed;
- a successful start advances status to `in_progress` and revision by one;
- a stale revision is a conflict and does not execute a second successful transition;
- a rejected Domain transition and a stale write leave persisted state unchanged.

## Ownership

HTTP owns path/body encodings and maps a stale revision to the reference public conflict response. Application owns a `StartTask` command and an atomic mutation Port expressed in Domain/Application terms. Domain owns `Task::start`. Infrastructure owns the MySQL transaction or compare-and-set mechanism, reconstructs the Task, invokes the Domain transition exactly once inside the atomic boundary, and persists the resulting snapshot. `app` installs the concrete adapter.

The Application use case must not preflight `Task::start` and then ask Infrastructure to run it again. The Port must be strong enough to represent the promised atomicity; a `find` followed by an unrelated `save` Port is not sufficient when the requirement is compare-and-set behavior.

## Required evidence

A completed executable slice must prove:

1. `pending@1 -> start(expected=1) -> in_progress@2`;
2. an illegal Domain transition does not mutate the object or row;
3. two attempts based on revision 1 cannot both succeed;
4. stale revision is distinguishable from not-found, Domain rejection, and infrastructure unavailability;
5. the production `app` composition installs the concrete adapter and the public Router path.

Do not add Domain Events, CQRS, event sourcing, an Outbox, Saga, Specification, or a generic transaction abstraction merely to make this example look more DDD-like. Those patterns require a separate demonstrated need.
