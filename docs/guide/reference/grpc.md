# gRPC

> **Status:** Development reference
>
> **Baseline:** The generated service exposes only Axum HTTP/JSON. It has no protobuf schema, generated transport code, Tonic dependency, streaming RPC, gRPC listener, or reflection service.
>
> **Read when:** A controlled service-to-service client needs a protobuf contract, code generation, or streaming semantics that the existing HTTP API does not provide.
>
> **Authority:** The five-crate dependency graph, Application error categories, and app lifecycle rules override transport examples here.

Choose gRPC for a real consumer contract: cross-language generated clients, protobuf compatibility, or streaming. HTTP/JSON remains simpler for browsers, public APIs, manual inspection, and ordinary request/response CRUD. Do not add a second protocol merely because Tonic and Axum both use Tokio and Tower.

## Add a sibling inbound adapter

When gRPC becomes real, add a transport crate such as `crates/grpc` beside `crates/http`:

```text
grpc -> application -> domain
app  -> grpc + http + infrastructure
```

The gRPC crate owns generated protobuf types, RPC request/response conversion, metadata extraction, interceptors, and `tonic::Status` mapping. It must not depend on `http` or `infrastructure`. Both inbound adapters call the same Application use cases, but each maps stable Application failures independently; HTTP-owned `ApiError` must never become the common error type.

Keep `.proto` packages explicitly versioned and treat field numbers and wire compatibility as a public contract. Protobuf messages are transport DTOs, not Domain entities. Parse them into Application inputs and Domain constructors; never derive Domain behavior from generated code or leak generated types into Ports.

Pin Tonic, Prost, and code-generation dependencies together in the manifests when the adapter is introduced, and make fresh code generation part of CI. The manifests and generated compilation path—not this Guide—own exact versions and feature flags. Tonic's current [crate documentation](https://docs.rs/tonic/latest/tonic/) is the primary API reference.

## Status, metadata, and streaming

Define one gRPC status mapping at the adapter boundary. Invalid transport values, missing authentication, rejected authorization, conflicts, unavailable dependencies, and internal failures need stable codes and safe messages just as HTTP errors do. Record concrete adapter failures outside the response and preserve the same redaction rules.

An interceptor can inspect metadata or reject a request, but Tonic documents it as less flexible than Tower middleware. Keep authentication extraction at the transport boundary and resource authorization in Application or Domain. Use Tower layers only when their failure path still becomes a valid gRPC status. See Tonic's [interceptor](https://docs.rs/tonic/latest/tonic/service/trait.Interceptor.html) and [server layer](https://docs.rs/tonic/latest/tonic/transport/server/struct.Server.html#method.layer) contracts.

Streaming introduces backpressure, client disconnect cancellation, per-message validation, partial delivery, and shutdown semantics. Define those before selecting client-, server-, or bidirectional streaming. A dropped stream may leave external effects completed, so the [Async and cancellation](async-and-cancellation.md) rules apply to every message-processing await point.

## Composition and verification

Prefer a separate listener first: it gives each protocol an explicit address, health surface, and failure boundary. `app` must own both server Futures, coordinate shutdown, propagate unexpected exit or panic, and flush observability after both stop. Adding the listener also requires a concrete deployment/network contract; see [Deployment](deployment.md).

Test the installed Tonic service with a generated client so code generation, conversion, status mapping, middleware, and lifecycle are exercised together. Add compatibility checks for any schema consumed outside the repository. Reflection is optional development exposure and must be enabled deliberately rather than assumed safe in production.
