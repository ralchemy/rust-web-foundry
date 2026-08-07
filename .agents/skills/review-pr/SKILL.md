---
name: review-pr
description: Review a change to this Rust web workspace for architecture, behavior, boundary safety, and verification. Use for pull requests, branch diffs, or pre-merge review.
---

# Review a change

1. Read the full diff, root `AGENTS.md`, and each touched crate's `AGENTS.md`.
2. Compare the change with its issue or stated behavior. Trace changed public paths rather than reviewing files in isolation.
3. Report actionable findings first: wrong dependency direction or owner, broken public behavior, hidden failure, secret/data leakage, unsafe migration/lifecycle behavior, or tests that cannot fail.
4. Flag needless wrappers, one-use abstractions, helper stacks, shared utility buckets, duplicated authorities, and unrequested dependencies.
5. Run the smallest relevant test, then `just check`; use `just verify` for composition, configuration, migrations, or runtime behavior.
6. If no findings remain, state that directly and mention any verification not run.
