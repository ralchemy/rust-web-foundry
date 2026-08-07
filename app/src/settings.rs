use crate::{AppResult, fail};
use config::{Config, Environment};
use secrecy::SecretString;
use serde::Deserialize;
use std::{net::SocketAddr, time::Duration};

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum LogFormat {
    Text,
    Json,
}

#[derive(Clone, Copy, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub(crate) enum TraceExporter {
    None,
    Console,
    Otlp,
}

pub(crate) struct MigrateSettings {
    pub(crate) database_url: SecretString,
    pub(crate) log_format: LogFormat,
    pub(crate) rust_log: String,
}

pub(crate) struct ServeSettings {
    pub(crate) database_url: SecretString,
    pub(crate) http_addr: SocketAddr,
    pub(crate) task_policy_url: String,
    pub(crate) task_policy_timeout: Duration,
    pub(crate) shutdown_timeout: Duration,
    pub(crate) deployment_environment: String,
    pub(crate) log_format: LogFormat,
    pub(crate) rust_log: String,
    pub(crate) trace_exporter: TraceExporter,
    pub(crate) otlp_endpoint: Option<String>,
    pub(crate) otlp_timeout: Duration,
}

#[derive(Deserialize)]
struct RawMigrateSettings {
    migration_database_url: SecretString,
    #[serde(default = "default_log_format")]
    log_format: LogFormat,
    #[serde(default = "default_rust_log")]
    rust_log: String,
}

#[derive(Deserialize)]
struct RawServeSettings {
    database_url: SecretString,
    #[serde(default = "default_http_addr")]
    http_addr: SocketAddr,
    task_policy_url: String,
    #[serde(default = "default_policy_timeout")]
    task_policy_timeout_ms: u64,
    #[serde(default = "default_shutdown_timeout")]
    shutdown_timeout_secs: u64,
    deployment_environment: String,
    #[serde(default = "default_log_format")]
    log_format: LogFormat,
    #[serde(default = "default_rust_log")]
    rust_log: String,
    #[serde(default = "default_trace_exporter")]
    trace_exporter: TraceExporter,
    otel_exporter_otlp_endpoint: Option<String>,
    #[serde(default = "default_otlp_timeout")]
    otel_exporter_otlp_timeout: u64,
}

pub(crate) fn load_dotenv() -> AppResult<()> {
    match dotenvy::from_filename(".env") {
        Ok(_) => Ok(()),
        Err(error) if error.not_found() => Ok(()),
        Err(_) => Err(fail("failed to load .env")),
    }
}

pub(crate) fn migrate() -> AppResult<MigrateSettings> {
    migrate_from(environment()?)
}

fn migrate_from(config: Config) -> AppResult<MigrateSettings> {
    let raw: RawMigrateSettings = config.try_deserialize()?;
    Ok(MigrateSettings {
        database_url: raw.migration_database_url,
        log_format: raw.log_format,
        rust_log: raw.rust_log,
    })
}

pub(crate) fn serve() -> AppResult<ServeSettings> {
    serve_from(environment()?)
}

fn serve_from(config: Config) -> AppResult<ServeSettings> {
    let raw: RawServeSettings = config.try_deserialize()?;
    validate_serve(&raw)?;
    Ok(ServeSettings {
        database_url: raw.database_url,
        http_addr: raw.http_addr,
        task_policy_url: raw.task_policy_url,
        task_policy_timeout: Duration::from_millis(raw.task_policy_timeout_ms),
        shutdown_timeout: Duration::from_secs(raw.shutdown_timeout_secs),
        deployment_environment: raw.deployment_environment,
        log_format: raw.log_format,
        rust_log: raw.rust_log,
        trace_exporter: raw.trace_exporter,
        otlp_endpoint: raw.otel_exporter_otlp_endpoint,
        otlp_timeout: Duration::from_millis(raw.otel_exporter_otlp_timeout),
    })
}

fn environment() -> AppResult<Config> {
    Ok(Config::builder()
        .add_source(Environment::default())
        .build()?)
}

fn validate_serve(settings: &RawServeSettings) -> AppResult<()> {
    if settings.task_policy_timeout_ms == 0
        || settings.shutdown_timeout_secs == 0
        || settings.otel_exporter_otlp_timeout == 0
    {
        return Err(fail("timeouts must be positive"));
    }
    if settings.deployment_environment.trim().is_empty() {
        return Err(fail("DEPLOYMENT_ENVIRONMENT must not be empty"));
    }
    if settings.trace_exporter == TraceExporter::Otlp
        && settings
            .otel_exporter_otlp_endpoint
            .as_deref()
            .is_none_or(str::is_empty)
    {
        return Err(fail(
            "OTEL_EXPORTER_OTLP_ENDPOINT is required for TRACE_EXPORTER=otlp",
        ));
    }
    Ok(())
}

fn default_http_addr() -> SocketAddr {
    "127.0.0.1:3000".parse().unwrap()
}

fn default_log_format() -> LogFormat {
    LogFormat::Text
}
fn default_rust_log() -> String {
    "info".into()
}
fn default_trace_exporter() -> TraceExporter {
    TraceExporter::Console
}
fn default_policy_timeout() -> u64 {
    2_000
}
fn default_shutdown_timeout() -> u64 {
    30
}
fn default_otlp_timeout() -> u64 {
    10_000
}

#[cfg(test)]
mod tests {
    use super::*;

    const SERVE_REQUIRED: [(&str, &str); 3] = [
        ("database_url", "mysql://example"),
        ("task_policy_url", "http://policy.example/check"),
        ("deployment_environment", "test"),
    ];

    fn configuration(values: impl IntoIterator<Item = (&'static str, &'static str)>) -> Config {
        let mut builder = Config::builder();
        for (key, value) in values {
            builder = builder.set_override(key, value).unwrap();
        }
        builder.build().unwrap()
    }

    fn serve_configuration(extra: &[(&'static str, &'static str)]) -> Config {
        configuration(SERVE_REQUIRED.into_iter().chain(extra.iter().copied()))
    }

    #[test]
    fn migrate_settings_are_command_scoped() {
        let settings = migrate_from(configuration([(
            "migration_database_url",
            "mysql://migration.example",
        )]))
        .unwrap();

        assert!(matches!(settings.log_format, LogFormat::Text));
        assert_eq!(settings.rust_log, "info");
        assert!(migrate_from(configuration(std::iter::empty::<(&str, &str)>())).is_err());
    }

    #[test]
    fn serve_settings_apply_typed_defaults() {
        let settings = serve_from(serve_configuration(&[])).unwrap();

        assert_eq!(settings.http_addr, "127.0.0.1:3000".parse().unwrap());
        assert_eq!(settings.task_policy_timeout, Duration::from_secs(2));
        assert_eq!(settings.shutdown_timeout, Duration::from_secs(30));
        assert_eq!(settings.otlp_timeout, Duration::from_secs(10));
        assert!(matches!(settings.log_format, LogFormat::Text));
        assert!(matches!(settings.trace_exporter, TraceExporter::Console));
        assert_eq!(settings.rust_log, "info");
    }

    #[test]
    fn serve_settings_reject_invalid_and_incomplete_values() {
        assert!(serve_from(configuration(std::iter::empty::<(&str, &str)>())).is_err());
        for (key, value) in [
            ("task_policy_timeout_ms", "0"),
            ("shutdown_timeout_secs", "0"),
            ("otel_exporter_otlp_timeout", "0"),
            ("deployment_environment", " "),
            ("log_format", "yaml"),
            ("trace_exporter", "zipkin"),
        ] {
            assert!(
                serve_from(serve_configuration(&[(key, value)])).is_err(),
                "accepted {key}={value}"
            );
        }
    }

    #[test]
    fn otlp_endpoint_is_conditional() {
        assert!(serve_from(serve_configuration(&[("trace_exporter", "none")])).is_ok());
        assert!(serve_from(serve_configuration(&[("trace_exporter", "otlp")])).is_err());

        let settings = serve_from(serve_configuration(&[
            ("trace_exporter", "otlp"),
            (
                "otel_exporter_otlp_endpoint",
                "http://collector.example:4317",
            ),
        ]))
        .unwrap();
        assert_eq!(
            settings.otlp_endpoint.as_deref(),
            Some("http://collector.example:4317")
        );
    }
}
