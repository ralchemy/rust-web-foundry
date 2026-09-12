# Performance

> **Status:** Development reference
>
> **Baseline:** The template installs no cache, response compression, streaming layer, pool tuning, benchmark suite, or profiler configuration.
>
> **Read when:** A measured latency, throughput, memory, CPU, connection, or payload problem requires a performance change.
>
> **Authority:** Requested behavior and confirmed decisions own intended performance targets; current source, tests, manifests, and gates own executable facts. This chapter is on-demand guidance only and adds no workflow gate.

Rust and async I/O provide useful building blocks, not a performance guarantee. Optimize a named workload against an observable target; do not infer a bottleneck from code shape alone.

## Start from evidence

Record the public operation, representative input, concurrency/load, data size, target metric, current measurement, and the profiler/query-plan/trace/load evidence that locates the cost. Change one owning seam, repeat the same measurement, and keep a regression check only when performance is part of the product contract.

A microbenchmark cannot prove end-to-end HTTP latency, and a local HTTP load test cannot prove production database capacity.

## Preserve async and resource ownership

The request path already uses async Axum, reqwest, and SQLx. Do not replace them with synchronous I/O inside async functions. Use `spawn_blocking` only for bounded synchronous work that cannot use an async interface, and account for its non-abortable execution during shutdown.

SQLx pools and reqwest clients are shared handles. Reuse them rather than creating one per request. Tune pool sizes only after pool wait time or connection pressure is measured against an explicit system budget.

## Optimize only the demonstrated cost

Prefer the smallest change that addresses measured work: query shape/indexes for database cost, payload or allocation changes for serialization cost, bounded concurrency for CPU-heavy work, and transport compression only when payload size and CPU trade-offs justify it. Do not add caches, alternative allocators, custom hashers, blanket `#[inline]`, Cargo profile changes, or new performance dependencies because a generic rule recommends them.

Performance changes must preserve project boundaries: Domain and Application remain free of framework-specific tuning; Infrastructure owns database/downstream implementation details; HTTP owns transport behavior; `app` owns runtime construction and lifecycle.

## Verification

Keep before/after evidence with the same workload and environment. Run the smallest functional tests affected by the change and the normal project completion gate. Add a benchmark or load test to the repository only when it protects a stable, repeatable performance contract rather than a one-off investigation.
