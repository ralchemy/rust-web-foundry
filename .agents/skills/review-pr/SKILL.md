---
name: review-pr
description: Review a change to this Rust web workspace for architecture, behavior, boundary safety, and verification. Use for pull requests, branch diffs, or pre-merge review.
---

# Review a change

1. Choose the mode from the supplied inputs. A frozen batch has a `batchId`, `snapshotId`, and explicit evidence paths; a standalone review does not.
2. For a frozen batch, start the output with its exact IDs, read only the supplied request, Context Pack, Evidence Pack, diff, and axis evidence, and return `incomplete` when any item is missing or unreadable. Treat those files as the complete snapshot: do not inspect the live tree or run commands.
3. For a standalone review, start from fresh context with the request, implementation Context Pack, Evidence Pack, and complete diff. Verify recorded source SHAs against the current tree; rebuild only for a stale source or unclassified path/action.
4. Compare the change with its stated behavior. Trace changed public paths rather than reviewing files in isolation.
5. Report actionable findings first: wrong dependency direction or owner, broken public behavior, hidden failure, secret/data leakage, unsafe migration/lifecycle behavior, or tests that cannot fail.
6. Trace every changed boundary type. Use only the conversion contracts selected by the Context Pack to verify that `FromStr`, `TryFrom`, `From`, or a named Domain operation matches its failure semantics and owner.
7. Reject adapter DTOs, database rows, or downstream wire types reused as Domain/Application models, and reject ambiguous booleans when a named decision is clearer. For fixed SQL, require checked macros, explicit columns, private rows, `TryFrom` reconstruction, and refreshed `.sqlx` metadata; reject `SELECT *`, `MySqlRow`, `Row::get`, `Row::try_get`, and unchecked fixed `query_as`.
8. Flag needless wrappers, one-use abstractions, helper stacks, shared utility buckets, duplicated authorities, unrequested dependencies, and clones that only appease the borrow checker.
9. In standalone mode, reconcile Evidence Pack claims by running the smallest relevant test, then `just check`; use `just verify` for composition, configuration, migrations, checked SQL, or runtime behavior. Keep successful logs outside the review context and include only a focused failure excerpt.
10. Emit one `## Review` section. State findings or no findings, any incomplete evidence, and standalone verification not run; the report is evidence, not a copy of repository rules.
