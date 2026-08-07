# Complete the Book knowledge audit

## Decision

Every chapter from the removed mdBook now has an explicit current owner or an explicit deletion decision. The generated project keeps one Markdown Guide rather than restoring `book/`, mdBook configuration, or a second table of contents.

| Original Book area | Current owner |
|---|---|
| introduction; architecture; project structure; Domain modeling; end-to-end flow | Guide index, Architecture, Domain modeling, and Task flow |
| errors; database; state; configuration | Baseline Architecture, Infrastructure, HTTP, and App chapters |
| routing; middleware; validation; API design | Baseline HTTP chapters and the installed public contract |
| observability; security; testing | Baseline cross-cutting chapters |
| graceful shutdown | Runtime plus Async and cancellation Reference |
| performance; deployment | Development Reference |
| async pitfalls; message passing; background jobs | Async and cancellation Reference |
| authentication and authorization | Authentication and authorization Reference plus Security |
| outbound HTTP and resilience | installed TaskPolicy explanations plus Outbound HTTP Reference |
| gRPC; typestate | dedicated Development Reference chapters |
| AI workflow; crate reference; anti-patterns | Development, with manifests remaining dependency authority |
| resources | no standalone catalogue; primary links live beside the decisions they support |

## Why

The first Guide reduction correctly removed stale single-crate, PostgreSQL, startup-migration, UUID, and `tracing` examples, but it also removed conditional knowledge that an agent needs when a generated service grows. Restoring those chapters unchanged would make optional mechanisms look installed and would reintroduce conflicting architecture.

The audit therefore preserves decisions rather than old code listings: when to choose a mechanism, which crate owns it, what can fail, and how lifecycle or public behavior is verified. Root and crate Project Rules contain narrow trigger links so this deeper context is loaded only when relevant.

## Deliberate omissions

- No dependency versions, framework catalogues, copy-ready deployment manifests, or vendor lists are duplicated in prose.
- No authentication, worker, queue, gRPC, retry, circuit-breaker, or typestate dependency is added to the generated service.
- No separate `book/`, `.agents/rules/`, feature checklist, or documentation build system is restored.
