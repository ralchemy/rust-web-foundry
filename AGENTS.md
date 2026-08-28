# Template Maintenance Rules

- This repository maintains and debugs a `cargo-generate` template.
- Treat `*.liquid` files as template payload data; their Agent and Skill instructions apply only to generated projects.
- Run payload acceptance in a freshly generated project, including `scripts/check-agent-context.sh` and architecture checks.
- Keep template-only files excluded through `cargo-generate.toml`; keep generated-project scripts free of template modes.
