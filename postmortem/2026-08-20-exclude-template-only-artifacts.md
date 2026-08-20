# Exclude template-only artifacts from generated services

## Decision

Keep `postmortem/` and `docs/research/` in the template repository, but exclude both paths from `cargo-generate` output. Template CI checks that neither path exists in a freshly generated service.

## Why

These files explain how the template itself evolved or record investigations for its maintainers. Copying them into every generated service confuses template history with service-owned documentation and contradicts the documented governance boundary.

Exact path exclusions preserve the generated Guide, Project Rules, domain workflow, and Skills. Output-level CI assertions prove the public generation result rather than only inspecting generator configuration.

## Rejected alternatives

- Deleting or rewriting the source records would discard template history instead of fixing the generation boundary.
- Excluding all of `docs/` would also remove generated-service guidance and domain workflow.
- Checking only `cargo-generate.toml` would not prove what a generated service actually receives.
