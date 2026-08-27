---
name: add-endpoint
description: Add or change an HTTP endpoint in this five-crate Rust web workspace. Use when a request affects an Axum route, HTTP contract, use case, domain rule, Port, or adapter.
---

# Add an endpoint

1. Read root `AGENTS.md` and the nearest local `AGENTS.md` for every planned touched path. Match the applicable action-first Context Pointers as a union, record or reuse the active plan's Context Set, and load each canonical owner once. Do not read a Guide chapter merely because this Skill names its topic.
2. Trace the nearest existing public route through Application and adapters. Reuse its owners before adding files or abstractions. When the trace expands the touched paths or action branches, update the Context Set and route only the newly matched owners.
3. Complete the matched Domain outcome and design checkpoint before production edits. Record the boundary conversion matrix: source type, target type, `FromStr`/`TryFrom`/`From` or named Domain operation, conversion owner, and failure owner.
4. Write one failing test at the innermost changed public seam. Touch only layers required by the behavior.
5. Work inside out through only the required owners: Domain invariant; Application use case or Port when orchestration or a business decision exists; Infrastructure adapter when an external capability exists; HTTP DTO/error/handler/route; then app wiring when construction changes.
6. Keep external details at adapters and fixed public failures in HTTP. Treat Guide and Context Pointer owners as read-only unless the active task explicitly declares Governance or Documentation scope; do not add route catalogues or copied rules opportunistically.
7. Run the focused test, then `just check`. Run `just verify` when wiring, configuration, lifecycle, migrations, checked SQL, or the installed route graph changed.
