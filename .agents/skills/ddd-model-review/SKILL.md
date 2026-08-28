---
name: ddd-model-review
description: Review a substantial Domain model or Domain implementation for business-language consistency, semantic ownership, invariants, state safety, type completeness, and architecture-boundary leakage. Use before declaring substantial Domain work complete or when asked to assess an existing model or implementation.
---

# Review a Domain model

Review business correctness and model quality with evidence. Do not replace a code review or invent numeric scores.

1. Start from the implementation handoff's Context Pack and Evidence Pack. Verify recorded SHAs against the current diff; rebuild the Context Pack only when the diff reveals an unclassified path/action or a source SHA is stale. Do not replay the implementation discovery transcript.
2. Read the confirmed glossary, context map, capability documents and ADRs, relevant design output, tests, and actual implementation selected by that context.
3. Trace representative business scenarios through the model. When code exists, trace one real public path through HTTP, Application, Domain, and the adapter.
4. Check language consistency, semantic and Aggregate ownership, invariant enforcement, legal transitions, event completeness, and acceptance coverage. Separate acquisition of current time, roles, or external facts from the business decision that uses them: Application or adapters may acquire facts, while pure business rules remain in Domain behavior or a Domain Policy.
5. Inspect changed public interfaces for primitive business values, swappable same-primitive arguments, invalid states that callers can construct, repeated validation or encoding, boolean state combinations, and anemic data containers.
6. Check boundaries: private HTTP DTOs and Infrastructure rows must not cross inward; Domain/Application types may be used by outer adapters; external encodings must remain at their owning adapter.
7. Confirm that durable capability semantics are committed at the authority selected by the Context Pack and agree with the implementation. A substantial model whose only semantic record is `.scratch/`, chat, or code is not Ready.
8. Distinguish Domain problems from engineering problems. Route missing language or scenarios to `ddd-discover`, semantic boundaries to `ddd-strategic-design`, invariants and types to `ddd-tactical-design`, and code organization or adapter problems to Project Rules.

Report findings first, ordered by severity:

- **P1**: business rule can be violated, material semantic ownership is wrong, or an invalid state is representable through a public path.
- **P2**: missing business type, leaking encoding, anemic ownership, inconsistent language, or incomplete meaningful acceptance coverage.
- **P3**: localized readability or modeling weakness with low immediate risk.

For every finding, cite the artifact or file and line, explain the business impact, name the correct owner, and recommend the smallest correction. Then list open questions and conclude **Ready** or **Not Ready** with blockers. If no actionable findings exist, say so explicitly.

Do not use unsupported percentages, quotas, or 0-10 scores. Do not require patterns such as Repository, Factory, Specification, event sourcing, Outbox, or eventual consistency without current evidence. Do not implement fixes unless requested.
