mod readiness;
mod repositories;

pub use readiness::MySqlReadinessProbe;
pub use repositories::MySqlTaskRepository;
use sqlx::{MySqlPool, migrate::Migrator, mysql::MySqlPoolOptions};

pub static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

pub async fn connect(database_url: &str) -> Result<MySqlPool, sqlx::Error> {
    MySqlPoolOptions::new().connect(database_url).await
}
