mod readiness;
mod repositories;

pub use readiness::MySqlReadinessProbe;
pub use repositories::MySqlTaskRepository;
#[cfg(feature = "reference-task")]
use sqlx::migrate::Migrator;
use sqlx::{MySqlPool, mysql::MySqlPoolOptions};

#[cfg(feature = "reference-task")]
pub static MIGRATOR: Migrator = sqlx::migrate!("./migrations/reference-task");

pub async fn connect(database_url: &str) -> Result<MySqlPool, sqlx::Error> {
    MySqlPoolOptions::new().connect(database_url).await
}
