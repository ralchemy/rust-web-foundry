# Make agent evidence deterministic

## Decision

Make Context Packs immutable and content-addressed. Scope expansion creates a union Pack, and handoff verifies that its current sources and declared paths/actions cover the final change.

Treat multi-axis review notifications as previews. A repository loader validates and prints the stable Standards and Spec artifacts together; finding classification begins only after the untruncated `reports_loaded: 2/2` marker. Frozen reviewers use only dispatcher-provided evidence, and handoff retains every finding plus reviewed and post-review diff identities.

Keep these gates in repository authority so generated services remain safe even when external Skills or personal agent configuration change. Detailed procedure stays in the Development Guide; root rules carry only admission, coverage, and completion protection.

## Why

A narrower repair Pack previously replaced the implementation Pack and no longer covered every changed path. In the same run, one full reviewer artifact was never loaded, so a hard testing finding disappeared from the final handoff. Both failures came from accepting mutable or preview evidence as complete.

Content identity, union extension, final coverage verification, and one 2/2 report loader turn those completion assumptions into observable gates without changing external tooling or adding dependencies.

## Gate change

`just architecture` now exercises Pack identity, immutability, union extension, source freshness, changed-path/action coverage, and positive and negative report loading. Template CI also verifies that the loader and this decision record are copied into a generated service.

The checks cannot infer the correct action keys from natural language and cannot create a stable reviewer artifact when a transport provides only a preview. Those cases remain explicit agent decisions or an incomplete Review Batch.
