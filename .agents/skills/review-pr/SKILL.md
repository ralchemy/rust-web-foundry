---
name: review-pr
description: Review a change to this Rust web workspace for architecture, behavior, boundary safety, and verification. Use for pull requests, branch diffs, or pre-merge review.
---

# Review a change

1. Review from a fresh context. Start with the request or issue, the implementation Context Pack, the compact Evidence Pack, and the complete diff. Do not inherit or request the implementation session's search transcript, failed attempts, successful build logs, or full conversation history.
2. Verify every recorded `path[#anchor]@content-sha` against the current tree. If the diff contains an unclassified touched path/action or a routed source is stale, rebuild the Context Pack and read only the newly selected or changed sources.
3. Compare the change with its stated behavior. Trace changed public paths rather than reviewing files in isolation.
4. Report actionable findings first: wrong dependency direction or owner, broken public behavior, hidden failure, secret/data leakage, unsafe migration/lifecycle behavior, or tests that cannot fail.
5. Trace every changed boundary type. Use only the conversion contracts selected by the Context Pack to verify that `FromStr`, `TryFrom`, `From`, or a named Domain operation matches its failure semantics and that the adapter owning the external representation owns the conversion.
6. Reject DTOs, database rows, or downstream wire types reused as Domain/Application models merely to avoid conversion. Reject ambiguous booleans when a named business decision is clearer.
7. For fixed SQL, require checked macros, explicit columns, private rows, `TryFrom` reconstruction, and refreshed `.sqlx` metadata. Reject `SELECT *`, `MySqlRow`, `Row::get`, `Row::try_get`, and unchecked fixed `query_as`.
8. Flag needless wrappers, one-use abstractions, helper stacks, shared utility buckets, duplicated authorities, unrequested dependencies, and clones that only appease the borrow checker.
9. Reconcile Evidence Pack claims with executable evidence. Run the smallest relevant test, then `just check`; use `just verify` for composition, configuration, migrations, checked SQL, or runtime behavior. Keep full logs out of review context unless a failure requires a focused excerpt.
10. If no findings remain, state that directly and mention any verification not run. The review output is evidence, not a second copy of repository rules.
