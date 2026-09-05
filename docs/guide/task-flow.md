# Task reference path

The canonical Task slice is executable architecture and DDD reference material, not a universal task-management model. It is disabled from the generated runtime by default and becomes part of the service only with the `reference-task` feature.

The example deliberately contains enough write/read behavior and state to demonstrate trust, representation, and semantic boundaries. Its business rules are illustrative. A generated project must not infer its own lifecycle, authorization, idempotency, or consistency rules from this example.

## Domain shape

The reference `Task` owns:

- `TaskId` and optional `AssigneeId`, which cannot be accidentally substituted;
- normalized `TaskTitle` and optional validated `TaskDescription`;
- finite `TaskPriority` and `TaskStatus` enums;
- optional `TaskEstimateMinutes`, whose unit and bounds are part of its type;
- positive `TaskRevision`;
- private fields plus separate `Task::create(NewTask)` and `Task::reconstitute(TaskSnapshot)` operations;
- named `start` and `complete` state transitions that advance revision only on success.

Reference creation owns the new Task ID, initial `pending` state, and initial revision. The transition example demonstrates a stronger invariant than field validation: a rejected operation leaves the entity unchanged. It does not claim that every user domain should have the same states or transitions.

## Create flow

```text
POST /api/v1/tasks                         # reference-task only
  ↓ JSON
http::CreateTaskRequest
  ↓ TryFrom
application::CreateTaskCommand
  ↓ TaskPolicyInput
infrastructure::PolicyRequestWire
  ↓ downstream HTTP
infrastructure::PolicyResponseWire
  ↓ TryFrom
application::TaskPolicyDecision
  ↓ Domain construction
domain::NewTask → domain::Task::create
  ↓ TaskRepository
sqlx::query! INSERT
  ↓ From
application::TaskView
  ↓ From
http::TaskResponse
```

Raw strings stop at the HTTP adapter. Application receives validated Domain values. The policy adapter owns its request and response wire types. The MySQL adapter owns SQL representations.

The request keeps `description`, `priority`, `assignee_id`, and `estimate_minutes` optional so the minimal smoke request remains valid. Missing priority becomes the typed `TaskPriority::Normal` at the HTTP conversion boundary rather than leaking an `Option<String>` inward.

## Lookup flow

```text
GET /api/v1/tasks/{task_id}                # reference-task only
  ↓ FromStr / TryFrom
domain::TaskId
  ↓ application::GetTask
application::TaskRepository::find
  ↓ sqlx::query_as!
infrastructure::TaskRow
  ↓ TryFrom
domain::Task::reconstitute(TaskSnapshot)
  ↓ From
application::TaskView
  ↓ From
http::TaskResponse
```

`TaskRow` is private to Infrastructure. A checked query proves column names and SQL/Rust types, but it does not prove that stored IDs, text, enums, quantities, or revisions satisfy current Domain rules. `TryFrom<TaskRow> for Task` parses every value and classifies invalid rows as corrupt persistence before anything crosses the Port.

## Runtime isolation

Without `reference-task`, the generated server exposes health/readiness only, does not require TaskPolicy configuration, and does not run Task schema migrations. Reference migrations live under `crates/infrastructure/migrations/reference-task/` and are embedded only by the enabled reference configuration.

The default-off feature is a runtime boundary, not an instruction to preload all reference code into every AI task. Start from the user's requested behavior and nearest production path. Read the reference slice only when it demonstrates a concrete pattern that the current change needs.

## Type ownership

| Type | Owner | Meaning |
|---|---|---|
| `CreateTaskRequest`, `TaskPath`, `TaskResponse` | HTTP | reference public JSON and path contracts |
| `CreateTaskCommand`, `TaskView` | Application | reference use-case input and approved result |
| `Task`, `TaskSnapshot`, and value objects | Domain | reference business meaning and valid state |
| `TaskRow` | MySQL adapter | selected database columns |
| `PolicyRequestWire`, `PolicyResponseWire` | outbound adapter | downstream protocol |

Types exist because contract, representation, trust, lifecycle, or semantic meaning differs. Layers do not duplicate models merely to produce a symmetric directory tree.

## Verification

Focused tests prove Value Object invariants, named Domain transitions and rejected-state safety, policy-before-persistence orchestration, request/path conversions and error mapping, create-then-get behavior through the installed reference Router, private row reconstruction, downstream wire conversion, real MySQL persistence, timeout/availability classification, and trace propagation.

`just check` proves both the default and reference compile/test shapes without requiring MySQL. `just ci` and `just verify` additionally prove that the default server has no Task route and that the opt-in reference path works end to end.
