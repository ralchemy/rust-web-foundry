# Task golden path

The canonical Task slice is executable architecture documentation, not a universal task-management
model. It deliberately contains enough fields and both write and read paths to demonstrate every
trust and representation boundary.

## Domain shape

The reference `Task` owns:

- `TaskId` and optional `AssigneeId`, which cannot be accidentally substituted;
- normalized `TaskTitle` and optional validated `TaskDescription`;
- finite `TaskPriority` and `TaskStatus` enums;
- optional `TaskEstimateMinutes`, whose unit and bounds are part of its type;
- positive `TaskRevision`;
- private fields plus separate `Task::create(NewTask)` and
  `Task::reconstitute(TaskSnapshot)` operations.

HTTP callers choose title, description, priority, assignee, and estimate. Domain creation owns the
new Task ID, initial `pending` status, and initial revision.

## Create flow

```text
POST /api/v1/tasks
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

Raw strings stop at the HTTP adapter. Application receives validated Domain values. The policy
adapter owns its request and response wire types. The MySQL adapter owns SQL representations.

The request keeps `description`, `priority`, `assignee_id`, and `estimate_minutes` optional so the
minimal smoke request remains valid. Missing priority becomes the typed `TaskPriority::Normal` at
the HTTP conversion boundary rather than leaking an `Option<String>` inward.

## Lookup flow

```text
GET /api/v1/tasks/{task_id}
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

`TaskRow` is private to Infrastructure. A checked query proves column names and SQL/Rust types, but
it does not prove that stored IDs, text, enums, quantities, or revisions satisfy current Domain
rules. `TryFrom<TaskRow> for Task` parses every value, rejects non-canonical persisted text and IDs,
and classifies invalid rows as corrupt persistence before anything crosses the Port.

`fetch_optional` represents a normal missing row. `row.map(Task::try_from).transpose()` preserves
the difference between not found and corrupt data.

## Type ownership

| Type | Owner | Meaning |
|---|---|---|
| `CreateTaskRequest`, `TaskPath`, `TaskResponse` | HTTP | public JSON and path contracts |
| `CreateTaskCommand`, `TaskView` | Application | use-case input and approved result |
| `Task`, `TaskSnapshot`, and value objects | Domain | business meaning and valid state |
| `TaskRow` | MySQL adapter | selected database columns |
| `PolicyRequestWire`, `PolicyResponseWire` | outbound adapter | downstream protocol |

Types exist because contract, representation, trust, lifecycle, or semantic meaning differs. The
layers do not duplicate models merely to produce a symmetric directory tree.

## Verification

Focused tests prove:

- every Value Object invariant and canonical representation;
- policy-before-persistence orchestration and short-circuiting;
- request/path conversions, defaults, and public error mapping;
- create-then-get behavior through the installed Router;
- private row reconstruction, including corrupt and non-canonical stored values;
- downstream wire decision conversion;
- real MySQL insert and `query_as!` lookup through the production composition root;
- downstream timeout, malformed response, availability classification, and trace propagation.
