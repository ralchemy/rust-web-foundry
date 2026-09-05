# AI reference benchmark

This directory defines a repeatable ablation for the template's actual product goal: improve AI-assisted delivery quality without rebuilding a large standing context.

## Comparison

Run the same held-out requirement against two fixed repository commits and the same model/runtime settings:

- **minimal reference**: a commit where the Task reference demonstrates create/get but not the target state/concurrency pattern;
- **enriched reference**: the candidate commit containing the additional executable/reference pattern.

The held-out requirement must be similar enough to need the design decision but must not reuse Task names, routes, business rules, or exact schema. This detects copying rather than transfer.

Use at least three clean runs per condition. Do not reuse implementation transcripts between runs. Final review starts in a fresh session from the original requirement and fixed diff.

## Record

For each run record:

- model/runtime/version and reasoning setting;
- base commit and candidate commit;
- first production edit token/read count when available;
- peak live context when available;
- total input/read volume and tool calls;
- whether the implementation invented an unconfirmed business rule;
- independent review findings by behavior, architecture/stack, and Rust quality;
- human corrections required before acceptance;
- focused checks and final `just check` / `just verify` outcome.

Do not collapse these into one score. A reference change is promising when semantic errors and human correction fall without unacceptable context/read growth.

## Suggested held-out tasks

1. A lifecycle mutation with an expected version and two concurrent attempts.
2. A simple information-maintenance endpoint that does **not** need a rich Aggregate, to detect over-modeling.
3. A new external policy decision with different business semantics from TaskPolicy, to detect semantic copying.

Store run summaries under `benchmarks/runs/`; keep full transcripts and command logs outside the repository when tooling provides stable links or artifacts.
