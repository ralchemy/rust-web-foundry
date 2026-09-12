# Testing

Tests follow ownership boundaries instead of a generic pyramid. Put each proof at the narrowest public seam that can fail when the promised behavior breaks, and use real composition only for contracts that cross crates.

## Evidence by boundary

| Surface | Typical location | What it proves |
|---|---|---|
| Domain invariants/transitions | beside the Domain type | valid construction, transitions, rejected-state safety |
| Application orchestration | beside the use case | Port order, short-circuiting, stable workflow outcomes |
| HTTP contract | through `routes::router` | installed routes, extraction, fallbacks, exact public errors |
| Adapter behavior | beside the adapter | concrete classification/conversion owned by that adapter |
| Production composition | `app/tests/` | real wiring and cross-crate behavior that cannot be proven locally |

Do not test a private helper when the public seam can expose the same failure. Do not move every test into `app/tests/`; heavier cross-crate tests often hide which owner broke.

## Default and reference shapes

The default feature set is the user-facing baseline. It exposes health/readiness and does not register Task APIs, require TaskPolicy configuration, or install the reference Task schema.

The default-off `reference-task` feature enables executable architecture/DDD teaching material. Its Task-specific business semantics are not requirements for a generated project's real domain.

## SQLx offline boundary

Committed `.sqlx/` metadata lets checked query macros compile without connecting to MySQL. It does not prove database behavior and does not replace migrations, constraints, or real query execution.

After changing a checked query or reference migration, refresh metadata against the intended development database and verify it against the migrated schema.

## Command boundaries

| Command | Contract |
|---|---|
| `just architecture` | database-free dependency/project-contract checks |
| `just check` | database-free format, compile, Clippy, tests, SQLx style, and project contract for default/reference feature shapes |
| `just test` | workspace tests against an existing configured MySQL |
| `just ci` | assumes MySQL already exists; runs `check`, acceptance/reference integration, SQLx verification, and lifecycle proof |
| `just verify` | starts local Compose MySQL, delegates to `just ci`, then stops Compose |

Use focused evidence while editing. Finish with `just check` for database-free changes. For SQLx metadata, migrations, configuration, production composition, installed routes, or lifecycle/runtime behavior, finish with `just verify` instead; it already includes `just check`, so a second final check run is redundant unless it was useful earlier for fast feedback.

Database-backed commands are not automatically safe merely because they are tests. Their effective URLs and Compose state can be supplied or influenced by the environment. Before autonomously repeating `just test`, `just ci`, migrations, SQLx preparation, or `just verify`, establish that they target the intended disposable or explicitly approved development resources. Otherwise run database-free proof and report what remains.

## Review

Mechanical checks prove explicit executable facts. Fresh review still checks whether requested behavior was understood correctly, semantic responsibility is owned by the right layer, reference semantics were not copied into user requirements, and important tests would fail for a plausible wrong implementation.

The pinned `rust-skills` testing rules apply subject to `.agents/rust-skills-overrides.md`. Generic recommendations do not justify adding mockall, snapshots, property testing, testcontainers, coverage, or another framework without a demonstrated gap.
