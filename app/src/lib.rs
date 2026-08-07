mod command;
mod observability;
mod server;
mod settings;

use application::CreateTask;
use infrastructure::{HttpTaskPolicy, MySqlReadinessProbe, MySqlTaskRepository};
use secrecy::{ExposeSecret, SecretString};
use sqlx::MySqlPool;
use std::{error::Error, io, time::Duration};

pub type AppResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

pub struct BuildConfig {
    pub database_url: SecretString,
    pub task_policy_url: String,
    pub task_policy_timeout: Duration,
    pub tracing_enabled: bool,
}

pub struct BuiltService {
    pub(crate) router: axum::Router,
    pub(crate) pool: MySqlPool,
}

impl BuiltService {
    pub fn router(&self) -> axum::Router {
        self.router.clone()
    }

    pub async fn close(self) {
        self.pool.close().await;
        fastrace::flush();
    }
}

pub async fn build(config: BuildConfig) -> AppResult<BuiltService> {
    let policy = HttpTaskPolicy::new(config.task_policy_url, config.task_policy_timeout)?;
    let pool = infrastructure::connect(config.database_url.expose_secret()).await?;
    let repository = MySqlTaskRepository::new(pool.clone());
    let readiness = MySqlReadinessProbe::new(pool.clone(), Duration::from_secs(1));
    let router = http::router(
        CreateTask::new(policy, repository),
        readiness,
        config.tracing_enabled,
    );
    Ok(BuiltService { router, pool })
}

pub async fn run() -> AppResult<()> {
    let command = command::parse()?;
    settings::load_dotenv()?;
    match command {
        command::Command::Migrate => migrate().await,
        command::Command::Serve => serve().await,
    }
}

async fn migrate() -> AppResult<()> {
    let settings = settings::migrate()?;
    observability::init_logging(settings.log_format, &settings.rust_log)?;
    let pool = infrastructure::connect(settings.database_url.expose_secret()).await?;
    let result = infrastructure::MIGRATOR.run(&pool).await;
    pool.close().await;
    result?;
    log::info!("database migrations applied");
    Ok(())
}

async fn serve() -> AppResult<()> {
    let settings = settings::serve()?;
    observability::init_logging(settings.log_format, &settings.rust_log)?;
    let tracing_enabled = observability::init_tracing(
        settings.trace_exporter,
        settings.otlp_endpoint.as_deref(),
        settings.otlp_timeout,
        &settings.deployment_environment,
    )?;
    let service = build(BuildConfig {
        database_url: settings.database_url,
        task_policy_url: settings.task_policy_url,
        task_policy_timeout: settings.task_policy_timeout,
        tracing_enabled,
    })
    .await?;
    server::serve(service, settings.http_addr, settings.shutdown_timeout).await
}

pub(crate) fn fail(message: &'static str) -> Box<dyn Error + Send + Sync> {
    Box::new(io::Error::other(message))
}
