use crate::{
    AppResult, fail,
    settings::{LogFormat, TraceExporter},
};
use fastrace::collector::{Config, ConsoleReporter};
use fastrace_opentelemetry::OpenTelemetryReporter;
use logforth::{
    append::Stdout,
    bridge::log::LogBridge,
    diagnostic::FastraceDiagnostic,
    filter::rustlog::RustLogFilterBuilder,
    layout::{JsonLayout, Layout, TextLayout},
};
use opentelemetry::{InstrumentationScope, KeyValue};
use opentelemetry_otlp::{Protocol, SpanExporter, WithExportConfig};
use opentelemetry_sdk::Resource;
use std::{borrow::Cow, time::Duration};

pub(crate) fn init_logging(format: LogFormat, rust_log: &str) -> AppResult<()> {
    let layout: Box<dyn Layout> = match format {
        LogFormat::Text => Box::new(TextLayout::default()),
        LogFormat::Json => Box::new(JsonLayout::default()),
    };
    let filter = RustLogFilterBuilder::try_from_spec(rust_log)?.build();
    let logger = logforth::core::builder()
        .dispatch(|dispatch| {
            dispatch
                .filter(filter)
                .diagnostic(FastraceDiagnostic::default())
                .append(Stdout::default().with_layout(layout))
        })
        .build();
    log::set_boxed_logger(Box::new(LogBridge::new(logger)))
        .map_err(|_| fail("logging system has already been initialized"))?;
    log::set_max_level(log::LevelFilter::Trace);
    Ok(())
}

pub(crate) fn init_tracing(
    exporter: TraceExporter,
    endpoint: Option<&str>,
    timeout: Duration,
    deployment_environment: &str,
) -> AppResult<bool> {
    match exporter {
        TraceExporter::None => Ok(false),
        TraceExporter::Console => {
            fastrace::set_reporter(ConsoleReporter, Config::default());
            Ok(true)
        }
        TraceExporter::Otlp => {
            init_otlp(
                endpoint.ok_or_else(|| fail("missing OTLP endpoint"))?,
                timeout,
                deployment_environment,
            )?;
            Ok(true)
        }
    }
}

fn init_otlp(endpoint: &str, timeout: Duration, environment: &str) -> AppResult<()> {
    let exporter = SpanExporter::builder()
        .with_tonic()
        .with_endpoint(endpoint.to_owned())
        .with_protocol(Protocol::Grpc)
        .with_timeout(timeout)
        .build()?;
    let resource = Resource::builder()
        .with_attributes([
            KeyValue::new("service.name", env!("CARGO_PKG_NAME")),
            KeyValue::new("service.version", env!("CARGO_PKG_VERSION")),
            KeyValue::new("deployment.environment.name", environment.to_owned()),
        ])
        .build();
    let scope = InstrumentationScope::builder(env!("CARGO_PKG_NAME"))
        .with_version(env!("CARGO_PKG_VERSION"))
        .build();
    fastrace::set_reporter(
        OpenTelemetryReporter::new(exporter, Cow::Owned(resource), scope),
        Config::default(),
    );
    Ok(())
}
