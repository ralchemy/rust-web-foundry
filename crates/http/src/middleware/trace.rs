use axum::{extract::Request, middleware::Next, response::Response};
use fastrace::{collector::SpanContext, local::LocalSpan};
use fastrace_axum::{FastraceLayer, TRACEPARENT_HEADER};

pub(crate) fn trace_layer(enabled: bool) -> FastraceLayer {
    FastraceLayer::default().with_span_context_extractor(move |request| {
        if !enabled || request.uri().path().starts_with("/health/") {
            return None;
        }
        request
            .headers()
            .get(TRACEPARENT_HEADER)
            .and_then(|header| SpanContext::decode_w3c_traceparent(header.to_str().ok()?))
            .or_else(|| Some(SpanContext::random()))
    })
}

pub(crate) async fn mark_server_error(request: Request, next: Next) -> Response {
    LocalSpan::add_property(|| ("span.kind", "server"));
    let response = next.run(request).await;
    if response.status().is_server_error() {
        LocalSpan::add_properties(|| {
            [
                ("span.status_code", "error"),
                ("error.type", "server_error"),
            ]
        });
    }
    response
}
