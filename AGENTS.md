# Template Maintenance Rules

- This repository maintains a `cargo-generate` template; `*.liquid` files are generated-project payloads.
- Generated projects are code-first: code, tests, manifests, migrations, Just recipes, and CI own executable facts.
- Keep the generated `AGENTS.md` small and keep implementation free of compiled rule bundles or mandatory workflow Skills.
- Preserve the generated Clean Architecture checks, stack documentation, and fresh review integration.
- Validate changes in a freshly generated project with the template workflow and `just ci`.
