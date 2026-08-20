# Contributing to Rust Web Foundry

This repository is the template source. Changes can affect template maintainers, newly generated services, or both.

## Start here

- Read the [Project Rules](AGENTS.md) and [Task reference glossary](CONTEXT.md).
- Use the [Development guide](docs/guide/development.md) to find the authority for each kind of change and the smallest evidence path.
- Read the [Domain workflow](docs/agents/domain.md) before changing business meaning and [Testing](docs/guide/testing.md) before changing test placement or public quality commands.

## Template and generated files

Root `README.md`, `TEMPLATE.md`, and this file describe the template repository. Their `.liquid` counterparts describe a generated service. Shared files—including the Guide, Project Rules, Domain workflow, and `CONTEXT.md`—are copied into generated services and are written from that service's perspective.

Change the owning source instead of patching generated output. After changing a generated or shared asset, render a fresh service outside this working tree and inspect the result for missing files or unresolved Liquid syntax.

## Evidence

Run the smallest focused check while editing, then run the generated service's `just check`. Run `just verify` when configuration, migrations, composition, lifecycle, or installed routes change. Inspect the full diff and report any changed CI or public gate as old proof, new proof, and retained equivalent coverage.
