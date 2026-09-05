# Reference pattern index

Reference code is executable teaching material, not authority for a generated project's business semantics. Start from the user's requested behavior and nearest production path. Read a reference only when it answers a concrete design question.

| Question | Start here | What to copy | What not to copy |
|---|---|---|---|
| How is an invariant represented? | `crates/domain/src/value_objects/task_title.rs` | private representation, validating construction | Task title rules |
| How does an entity own a state transition? | `crates/domain/src/entities/task.rs` | named operation, rejected transition leaves state unchanged | Task lifecycle |
| How does Application cross an external boundary? | `crates/application/src/use_cases/task/create.rs` | use-case-owned Port orchestration | TaskPolicy semantics |
| How is persistence reconstructed safely? | `crates/infrastructure/src/mysql/repositories/task.rs` | private row, checked conversion into Domain | Task schema |
| How is a complete public path installed? | `app/tests/create_task.rs` | real composition and adapter-boundary proof | Task HTTP contract |
| How should concurrency-sensitive mutations be shaped? | `docs/guide/reference-concurrency.md` | expected revision, atomic Port contract, Domain decision under the atomic boundary | optimistic locking when the user's requirement does not need it |

Keep this index short. Do not turn it into a context bundle or preload every referenced file. Add a row only when a new reference demonstrates a materially different design decision that cannot already be learned from an existing row.
