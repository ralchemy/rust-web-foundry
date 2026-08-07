# Separate rules from development reference

## Decision

Always-applicable constraints remain in root and crate-local `AGENTS.md` files. The old Book's reusable but conditional engineering knowledge lives under `docs/guide/reference/` and is loaded through task-specific trigger links in Project Rules.

Baseline Guide chapters explain installed behavior. Development Reference chapters explain when and how to introduce a conditional concern without claiming that it is already generated. Skills remain recurring procedures and may read both surfaces without copying their contents.

## Why

Restricting the Guide to installed code made the generated service easy to describe but removed knowledge an agent needs to extend a real Rust Web project. Keeping every Book chapter always active creates the opposite failure: generic alternatives look mandatory and cause speculative dependencies and layers.

The trigger link is the small interface to a deeper body of knowledge. An agent sees the universally required rule first and reads the detailed rationale, alternatives, examples, and source links only when the change crosses that seam.

## Promotion and demotion

- Promote repeated, always-invalid behavior from Reference into the applicable `AGENTS.md` file.
- Demote a rule when workload, platform, or product context turns it into a conditional choice.
- Update the Baseline Guide when a referenced capability becomes installed generated behavior.
- Keep executable facts in Cargo, code, tests, Just, and CI rather than restating them as prose authority.

## Rejected alternatives

- Placing critical rules only under a custom `.agents/rules/` directory would rely on a nonstandard file being discovered and loaded.
- Restoring the full mdBook unchanged would reintroduce the old single-crate, PostgreSQL, startup-migration, and tracing assumptions.
- Copying reference checklists into Skills would create competing owners and load irrelevant material during routine changes.
