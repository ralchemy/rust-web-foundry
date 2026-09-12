# Reviewing changes

Implementation is code-first. Start from the requested behavior and the nearest complete production path; use source, tests, manifests, migrations, Just recipes, and compiler output rather than loading a separate rule bundle. The Guide is reference material when the code does not answer a concrete design question.

Final review runs in a fresh context so implementation exploration and failed attempts do not compete with the diff.

## Install the pinned Rust baseline

```sh
bash scripts/install-rust-skills.sh
bash scripts/install-rust-skills.sh --check
```

The checkout is installed under ignored `.scratch/rust-skills` at the commit recorded in `.agents/rust-skills.lock`. The project does not copy the generic rule index into `AGENTS.md`, and normal implementation sessions do not load it.

## Run the project review Skill

Invoke `.agents/skills/review-rust-web/SKILL.md` explicitly in a fresh session and provide the request/specification plus the complete branch or working-tree diff. OpenAI-compatible hosts also receive `agents/openai.yaml` with implicit invocation disabled.

The Skill reviews three separate axes:

1. behavior and acceptance evidence;
2. project Clean Architecture, selected stack, persistence/security/lifecycle contracts where changed;
3. applicable `rust-skills` rules, selected progressively from the pinned checkout by diff concern and rule filename.

Only requested behavior and explicitly confirmed acceptance criteria own intended behavior. Tests created during implementation are evidence unless the request or a confirmed decision explicitly promotes them to acceptance criteria. Project code, manifests, gates, and `.agents/rust-skills-overrides.md` override generic `rust-skills` recommendations.

A formal review is incomplete when the diff scope, originating request, pinned rules checkout, or required verification is unavailable. Missing evidence blocks only conclusions that depend on it: complete independent review axes when enough evidence exists, state exactly what remains unverified, and never substitute another generic ruleset for the pinned baseline.

## Verification

Run the smallest owning test while editing or investigating. Finish with `just check`, or with `just verify` instead when a change affects SQLx metadata, migrations, installed routes or production composition, configuration, or runtime/lifecycle behavior. `just verify` already delegates to the complete check gate; do not require a separate successful `just check` immediately before it unless an earlier fast check is useful for feedback.

Keep complete successful logs outside the review prompt; include the command, result, and only the smallest useful failure excerpt.

## Updating rust-skills

Upgrade the commit and version in `.agents/rust-skills.lock` in a dedicated change. Review the upstream changelog and diff, update project overrides for new conflicts, reinstall the checkout, and validate a fresh generated project before merging.
