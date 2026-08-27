---
name: review-pr
description: Review a change to this Rust web workspace for architecture, behavior, boundary safety, and verification. Use for pull requests, branch diffs, or pre-merge review.
---

# Review a change

1. Read the full diff, root `AGENTS.md`, and the nearest local `AGENTS.md` for every changed path. Match the applicable action-first Context Pointers from the behavior and boundaries actually changed, record or reuse the review plan's Context Set, and load each canonical owner once. Do not unconditionally load the complete Idiomatic Rust or Guide chapters.
2. Compare the change with its issue or stated behavior. Trace changed public paths rather than reviewing files in isolation. If the trace exposes additional paths or action branches, update the Context Set and route only newly matched owners.
3. Report actionable findings first: wrong dependency direction or owner, broken public behavior, hidden failure, secret/data leakage, unsafe migration/lifecycle behavior, or tests that cannot fail.
4. Trace every changed boundary type. Use the matched anchored conversion owner to verify that `FromStr`, `TryFrom`, `From`, or a named Domain operation matches its failure semantics and that the adapter owning the external representation owns the conversion.
5. Reject DTOs, database rows, or downstream wire types reused as Domain/Application models merely to avoid conversion. Reject ambiguous booleans when a named business decision is clearer.
6. For fixed SQL, require checked macros, explicit columns, private rows, `TryFrom` reconstruction, and refreshed `.sqlx` metadata. Reject `SELECT *`, `MySqlRow`, `Row::get`, `Row::try_get`, and unchecked fixed `query_as`.
7. Flag needless wrappers, one-use abstractions, helper stacks, shared utility buckets, duplicated authorities, unrequested dependencies, and clones that only appease the borrow checker.
8. Run the smallest relevant test, then `just check`; use `just verify` for composition, configuration, migrations, or runtime behavior.
9. If no findings remain, state that directly and mention any verification not run.
