# Generate human governance anchors

## Decision

Track one shared `CONTEXT.md` glossary, a template-facing `CONTRIBUTING.md`, and a generated-service `CONTRIBUTING.md.liquid`. Keep the full knowledge-ownership index in the Development Guide, link both contribution entries to it, and make template CI check the generated anchors and the README-to-contribution path.

## Why

Generated services already carried detailed Project Rules and optional Skills, but a human maintainer had no thin entry into those requirements and the promised Domain glossary was absent. One shared glossary keeps confirmed Task reference language consistent, while separate contribution entries avoid teaching generated-service maintainers how to edit the template source.

Direct file and link checks prove the stable navigation contract without freezing prose or introducing a documentation build system. Template history remains template-owned and is still excluded from generated output.

## Rejected alternatives

- Copy one contribution guide to both repositories: it would mix template rendering work with ordinary service changes.
- Put the missing guidance only in `AGENTS.md` or a Skill: human contributors might never reach it, and optional tooling would become the sole owner of mandatory knowledge.
- Add semantic documentation lint or snapshots: brittle prose checks would not prove correctness and would make harmless edits contractual.
- Add a Task capability document: the Task slice is executable architecture documentation, not a universal business capability.
