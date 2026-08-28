# Compile agent context packs

## Decision

Replace distributed natural-language Context Pointer tables with one finite action catalogue at `docs/agents/context-routes.tsv`. Keep root and nearest-local `AGENTS.md` files limited to standing governance, responsibility, and hard protection.

Before implementation, classify planned touched paths plus the smallest applicable action keys and compile a bounded Context Pack. The pack contains root/local standing briefs and each routed owner once, uses anchored sections when declared, records `path[#anchor]@content-sha`, and fails above a 60,000-byte default ceiling.

Implementation hands off a compact Evidence Pack. Final conformance review starts in fresh context with the request, Context Pack, Evidence Pack, and diff; it reloads authority only when a SHA is stale or the diff exposes an unclassified path or action.

## Why

Progressive disclosure reduced standing prose but still left models responsible for repeatedly interpreting dozens of natural-language pointers during a long tool loop. The same task could therefore re-read owners, keep discovery transcripts, successful build logs, and the full implementation history until final review. Static routed-context budgets measured the ideal unique rule set but not that runtime accumulation.

A finite action vocabulary moves routing identity out of prose-heavy standing files without weakening canonical Guide or Domain ownership. Compilation makes selection, deduplication, anchor extraction, freshness identity, and the byte ceiling deterministic after action classification. The generated pack is deliberately non-authoritative so it cannot drift into a second specification.

Fresh review removes exploration and failed-attempt history from the final reasoning stage while preserving the exact authority identity and implementation evidence needed to audit the diff. This targets lost-in-the-middle risk by bounding each reasoning stage rather than relying on the model's maximum context window.

## Gate change

`just architecture` continues to enforce routed-context scenario budgets and now additionally verifies that:

- action keys are unique and every routed source or anchor resolves;
- large conditional owners are routed only through anchors;
- root `AGENTS.md` stays at or below 6,000 UTF-8 bytes;
- standing briefs contain no legacy `→ read` conditional routes;
- repository Skills consume the Context Pack protocol and do not directly name conditional Guide owners.

The checks validate governance structure and context size mechanics. They do not claim to decide whether a model classified the correct action keys for an arbitrary natural-language request; representative dry runs remain the behavioral test for that seam.
