# Reviewing changes

Implementation is code-first. Start from the requested behavior and the nearest complete production path; use source, tests, manifests, migrations, Just recipes, and compiler output rather than loading a separate rule bundle. The Guide is reference material when the code does not answer a concrete design question.

Final review runs in a fresh context so implementation exploration and failed attempts do not compete with the diff.

## Install the pinned Rust baseline

```sh
bash scripts/install-rust-skills.sh
bash scripts/install-rust-skills.sh --check
```

The checkout is installed under ignored `.scratch/rust-skills` at the commit recorded in `.agents/rust-skills.lock`. The project does not copy the 37 KB rule index into `AGENTS.md`, and normal implementation sessions do not load it.

## Run the project review Skill

Invoke `.agents/skills/review-rust-web/SKILL.md` explicitly in a fresh session and provide the request/specification plus the complete branch or working-tree diff. The Skill reviews three separate axes:

1. behavior and acceptance evidence;
2. project Clean Architecture, selected stack, persistence/security/lifecycle contracts where changed;
3. applicable `rust-skills` rules, selected progressively from the pinned index.

The requested behavior and acceptance tests override every general convention. Project code, manifests, gates, and `.agents/rust-skills-overrides.md` override generic `rust-skills` recommendations.

A review is incomplete when the diff scope, originating request, pinned rules checkout, or required verification is unavailable. Findings cite a project contract or rust-skills rule ID and distinguish correctness errors from warnings and optional advice.

## Verification

Run the smallest owning test and `just check`. Run `just verify` when a change affects SQLx metadata, migrations, installed routes or production composition, configuration, or runtime/lifecycle behavior. Keep complete successful logs outside the review prompt; include the command, result, and only the smallest useful failure excerpt.

## Updating rust-skills

Upgrade the commit and version in `.agents/rust-skills.lock` in a dedicated change. Review the upstream changelog and diff, update project overrides for new conflicts, reinstall the checkout, and validate a fresh generated project before merging.
