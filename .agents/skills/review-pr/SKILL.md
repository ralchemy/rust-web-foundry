---
name: review-pr
description: Review a change to this Rust web workspace for architecture, behavior, boundary safety, and verification. Use for pull requests, branch diffs, or pre-merge review.
---

# Review a change

1. Read the full diff, root `AGENTS.md`, `docs/guide/reference/idiomatic-rust.md`, and each touched crate's `AGENTS.md`.
2. Compare the change with its issue or stated behavior. Trace changed public paths rather than reviewing files in isolation.
3. Report actionable findings first: wrong dependency direction or owner, broken public behavior, hidden failure, secret/data leakage, unsafe migration/lifecycle behavior, or tests that cannot fail.
4. Trace every changed boundary type. Verify that `FromStr`, `TryFrom`, `From`, or a named Domain operation matches its failure semantics and that the adapter owning the external representation owns the conversion.
5. Reject DTOs, database rows, or downstream wire types reused as Domain/Application models merely to avoid conversion. Reject ambiguous booleans when a named business decision is clearer.
6. For fixed SQL, require checked macros, explicit columns, private rows, `TryFrom` reconstruction, and refreshed `.sqlx` metadata. Reject `SELECT *`, `MySqlRow`, `Row::get`, `Row::try_get`, and unchecked fixed `query_as`.
7. Flag needless wrappers, one-use abstractions, helper stacks, shared utility buckets, duplicated authorities, unrequested dependencies, and clones that only appease the borrow checker.
8. Run the smallest relevant test, then `just check`; use `just verify` for composition, configuration, migrations, or runtime behavior.
9. If no findings remain, state that directly and mention any verification not run.
