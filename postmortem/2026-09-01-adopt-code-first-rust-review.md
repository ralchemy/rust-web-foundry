# Adopt code-first implementation and pinned Rust review

## Decision

Generated projects use a small root contract and executable repository evidence instead of compiling Guide chapters into implementation context. Remove generated Context Pack routing, compiler/review-batch scripts, nested Agent rules, and implementation workflow Skills. Keep the Guide as cold human/reference documentation.

Pin `leonardomso/rust-skills` for explicit use in a fresh review session. The review keeps behavior, project architecture/stack, and generic Rust quality as separate axes. Project authority and a short override file win when generic rules recommend a conflicting module shape, telemetry stack, error strategy, mock framework, dependency, or Cargo profile.

## Evidence

Three paired Pi runs implemented the same cross-layer Task partial-update endpoint. Both current and code-first conditions passed `just check` and `just verify` in all three runs. Code-first reduced the median prompt before the first production edit from 70,716 to 38,833 tokens, median live prompt from 110,066 to 70,610, read output by 34.5%, and assistant calls by 29.9%.

The previous workflow invoked the context compiler 9, 16, and 13 times across its three runs, while no complete Pack read was detected and two runs still read Guide material directly. The largest current run reached 287,442 prompt tokens. This showed that the orchestration control plane was not reliably delivering its data and created substantial tail risk.

Code-first still read slightly more source files, while total read output fell. The gain therefore came from replacing procedural governance text with direct production evidence, not from understanding less code.

## Boundaries

The generic Rust Skill does not own feature semantics, Clean Architecture responsibility, selected dependencies, HTTP contracts, SQLx/MySQL consistency, production composition, security policy, or lifecycle. Those remain with the request, code/tests, manifests, Guide, and executable gates.

The Rust baseline is not copied into `AGENTS.md` and is not loaded during implementation. A pinned checkout under ignored `.scratch/` is installed on demand for review, and only rule files relevant to the diff are read.

## Follow-up

Measure implementation-stage growth separately. The ablation removed most pre-edit expansion, but long edit/test/fix loops can still produce large final contexts. Prefer fresh review, compact successful command output, targeted rereads, and stable diff/log files rather than reintroducing standing prose.
