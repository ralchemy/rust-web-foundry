# Domain modeling

Domain types make business meaning and valid state explicit. Raw values may enter through an adapter, but they cross into Domain only through constructors or parsers that establish the required invariants.

The Task creation path demonstrates the boundary:

```text
HTTP CreateTaskRequest
    → TryFrom / FromStr at the HTTP boundary
Application CreateTaskCommand containing Domain values
    → CreateTask use case
Domain NewTask → Task::create
```

The HTTP adapter owns wire decoding and converts raw strings and numbers into `TaskTitle`, `TaskPriority`, `AssigneeId`, `TaskEstimateMinutes`, and other Domain values. Application receives valid Domain values and orchestrates the current workflow. Domain constructors remain the single authority for Domain validity.

## Model semantic differences

Use a distinct Domain type when a value has business meaning, an invariant, a finite set, a unit, a trust distinction, or a risk of same-primitive confusion. `TaskId`, `TaskTitle`, `TaskPriority`, and `TaskRevision` are separate because callers must not interchange them and because each owns different construction rules.

Keep a newtype's representation private. Provide constructors, parsing, formatting and read access that preserve its invariant. Do not expose mutable access to the wrapped value.

Do not create a wrapper merely to make every layer look symmetric. HTTP DTOs, Application commands/results, Domain entities, database rows and downstream wire types are separate only when their contract, representation, trust or lifecycle differs.

## Keep representation at adapters

Domain types do not derive Serde merely for HTTP, database or message convenience. A derived wire constructor can bypass Domain construction, and its field names/defaults belong to an external representation that may evolve independently.

Use adapter-owned DTOs and rows:

```text
HTTP DTO → validated Domain/Application input
private database row → TryFrom → Domain entity or Application projection
Domain/Application result → explicit HTTP response conversion
```

If serialization itself becomes a business-owned contract, establish that requirement explicitly and preserve invariant-validating construction. Do not introduce it as a convenience refactor.

## Commands and operations

An Application command is appropriate when one workflow has several coherent inputs or several inbound adapters share the same operation shape. It should contain Domain/Application values, not raw transport encodings. `CreateTaskCommand` is such a contract.

Keep creation, reconstitution and state transitions as named Domain operations such as `Task::create(NewTask)` and `Task::reconstitute(TaskSnapshot)`. Use `From`, `TryFrom` and `FromStr` for representation conversions, not for operations with business semantics.

Domain errors describe invalid Domain state or rejected Domain behavior. They never carry Axum, SQLx, reqwest, configuration, status codes, raw payloads or other adapter details.
