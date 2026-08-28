# Performance

> **Status:** Development reference
>
> **Baseline:** The template installs no cache, response compression, streaming layer, pool tuning, benchmark suite, or profiler configuration.
>
> **Read when:** A measured latency, throughput, memory, CPU, connection, or payload problem requires a performance change.
>
> **Authority:** When an applicable action key selects this chapter into the compiled Context Pack, this chapter is the conditional performance contract. Source, tests, and gates own executable facts; root and nearest-local rules retain standing scope responsibility and local hard protection.

Rust and async I/O provide useful building blocks, not a performance guarantee. Optimize a named workload against an observable target; do not infer a bottleneck from code shape alone.

## Begin with a performance contract

Record enough context to reproduce the problem:

- the public operation and representative input;
- concurrency and request rate;
- data volume and database state;
- latency percentile, throughput, resource, or payload target;
- current measurement and environment;
- the profiler, query plan, trace, or load result that locates the cost.

Change one owning seam, repeat the same measurement, and keep a regression check when performance is a product contract. A microbenchmark cannot prove HTTP latency, and a local HTTP load test cannot prove production database capacity.

## Keep blocking work off async workers

The generated request path uses async Axum, reqwest, and SQLx operations. Do not replace them with synchronous I/O inside an async function.

Use `tokio::task::spawn_blocking` only for bounded synchronous work that cannot use an async interface. Tokio documents that started blocking tasks cannot be aborted, and a large number of CPU-bound calls must be concurrency-limited or moved to a dedicated compute executor. This affects shutdown as well as throughput; an unbounded blocking call can outlive the request and delay runtime termination. See the pinned [`spawn_blocking` documentation](https://docs.rs/tokio/1.53.1/tokio/task/fn.spawn_blocking.html).

Examples that may justify an explicit blocking seam include CPU-heavy image processing, a synchronous vendor SDK, or filesystem work with no suitable async path. Password hashing also needs a concurrency bound, but authentication is not part of the baseline.

## Size resource pools from budgets

[`infrastructure::connect`](../../../crates/infrastructure/src/mysql/mod.rs) currently uses SQLx defaults. A SQLx pool is already a cheap shared handle; clone it rather than creating a pool per repository or request.

Tune it only after pool wait time or database connection pressure is measured. Start from the system budget:

```text
maximum application connections
    = maximum replicas × max_connections per replica
```

Leave capacity for migrations, administration, failover, and other clients. `min_connections` trades startup/background connections for warm capacity; `acquire_timeout` bounds queueing but does not create database capacity. Review the pinned [`PoolOptions` interface](https://docs.rs/sqlx/0.9.0/sqlx/pool/struct.PoolOptions.html) before exposing settings, and keep parsing in `app` while Infrastructure owns concrete pool construction.

## Optimize queries where evidence points

For a slow MySQL path:

1. capture the actual bound query shape and representative cardinality without logging sensitive values;
2. inspect its MySQL execution plan;
3. fetch only fields the adapter needs;
4. add or change an index through a forward migration;
5. refresh SQLx metadata and measure the same workload again.

Pagination, batching, denormalization, and caching are product/data consistency decisions, not universal query decorations. Add pagination when a real collection can grow without bound. Add a cache only after defining its owner, key, invalidation, staleness, failure behavior, and measurement target.

## Treat payload work as an HTTP decision

Compression can reduce transfer size for sufficiently large compressible responses while consuming CPU and changing headers. Do not install it for the current small Task response. When payload measurements justify it, HTTP owns negotiation, exclusions, middleware order, and public-path tests; confirm that upstream ingress is not already performing the same work.

Stream a response when buffering creates a measured memory or latency problem and the public contract can represent partial delivery. Streaming changes error semantics: once headers or bytes are sent, the fixed JSON error envelope may no longer be available.

## Choose a tool for the question

| Question | Evidence |
|---|---|
| Did a pure function become faster? | Criterion-style microbenchmark with representative inputs |
| Can the installed Router meet an HTTP target? | Repeatable load test with controlled concurrency and payloads |
| Which code consumes CPU? | Sampling profiler or flame graph |
| Which allocations dominate? | Allocation or heap profiler |
| Why are async tasks delayed? | Tokio runtime instrumentation such as tokio-console |
| Why is a database operation slow? | Adapter span timing, pool wait evidence, and MySQL query plan |

Tools used only during diagnosis need not become runtime dependencies. Commit a benchmark or load scenario only when the performance contract must be protected continuously.

## Ownership of a performance change

| Problem | Owner |
|---|---|
| Domain algorithm or invariant cost | `domain` |
| Use-case call count or sequencing | `application` |
| Payload, compression, streaming, middleware | `http` |
| SQL, index, pool, downstream client behavior | `infrastructure` |
| Runtime threads, settings, lifecycle, deployment resources | `app` or deployment artifacts |

Do not compensate in an outer layer for cost owned inward. For example, caching an incorrect repeated repository call in HTTP hides the Application orchestration problem instead of fixing it.
