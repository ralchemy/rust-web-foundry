# Domain modeling

Domain types make business meaning and valid state explicit. Raw values may enter through an adapter, but they cross into Domain only through constructors that establish the required invariants.

The Task creation path demonstrates the boundary:

```text
HTTP CreateTaskRequest { title: String }
    ↓
application::CreateTask::execute(String)
    ↓ TaskTitle::parse
domain::TaskTitle
    ↓ Task::new
domain::Task
```

Application invokes `TaskTitle::parse`, so every inbound adapter that uses the same use case shares the Domain invariant. HTTP validation may reject malformed transport input earlier, but it does not replace Domain construction. See [Validation boundaries](../http/validation.md) for the distinction between transport checks, Domain invariants, and Application decisions.

## Prefer domain types over primitives

Give every named, semantically distinct, or easily confused Domain value its own type, even when validation is currently small. `TaskId` and `TaskTitle` both have compact representations, but their types prevent accidental interchange and make function signatures state their meaning.

Boundary payloads and temporary parsing values may remain primitives. Entity fields should rarely remain a bare `String`, integer, or timestamp when the value represents a business concept. Validation is only one benefit of a newtype; semantic vocabulary and compile-time non-interchangeability are equally important.

Keep a newtype's representation private. Provide only constructors and read access that preserve its invariant. Do not expose mutable access to the wrapped value.

## Parse once at construction

`TaskTitle::parse` trims surrounding whitespace, counts Unicode scalar values, and rejects empty, overlong, or control-character input. Once parsing succeeds, callers can rely on those properties without validating the title again.

Entities follow the same rule. `Task::new` accepts a valid `TaskTitle` and generates its `TaskId` inside Domain. Behavior that belongs to one Entity or Value Object stays on that type. A pure rule spanning several Domain concepts may become a Domain Service as described in [Ports and adapters](../architecture/ports-and-adapters.md).

## Keep serialization at adapters

Domain types do not derive Serde merely for HTTP, database, or message convenience. In particular, a derived `Deserialize` implementation can construct a newtype from its representation without calling the validating constructor. Serde field names, defaults, flattening, and omission rules also describe a wire contract that may change independently of the Domain model.

Use boundary-owned DTOs instead:

```text
HTTP DTO → Application input → Domain constructor
Domain value → explicit HTTP response mapping
```

If a future project determines that serialization is itself a Domain-owned contract, that is an explicit architecture change: update the Project Rules and implement deserialization through invariant-preserving constructors rather than silently adding derives.

## Application inputs are not Domain requests

`CreateTaskRequest` is an HTTP DTO because it describes JSON accepted by one inbound adapter. `CreateTask::execute` currently accepts its single raw title directly, avoiding a one-field wrapper while centralizing Domain parsing in the use case.

When a use case gains several meaningful inputs or multiple inbound adapters need the same operation shape, define an Application-owned `Command`, `Input`, or `Params` type. Do not put a transport-named `Request` in Domain. Domain constructors accept Domain values and express business creation, not how an operation arrived.

## Convert explicitly at boundaries

Use `From` only when conversion cannot fail. Use `TryFrom` or an explicit constructor when external data may violate current invariants. A future database read path must not trust a row merely because it came from MySQL; it reconstructs Domain values through their public validation path and classifies corrupt data at the Infrastructure boundary.

HTTP DTOs, database rows, Application inputs, and Domain entities are separate types when their contracts differ. Do not create duplicate one-field wrappers when there is no distinct boundary meaning, but never reuse one boundary type solely to save a conversion.

Domain errors describe invalid Domain state or rejected Domain operations. They may use a typed error implementation such as `thiserror`, but they do not carry `anyhow`, SQLx, reqwest, Axum, status codes, raw payloads, or other adapter details.
