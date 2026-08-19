use application::{ReadinessError, ReadinessProbe};
use fastrace::{future::FutureExt, prelude::Span};
use sqlx::MySqlPool;
use std::time::Duration;

#[derive(Clone)]
pub struct MySqlReadinessProbe {
    pool: MySqlPool,
    timeout: Duration,
}

impl MySqlReadinessProbe {
    pub fn new(pool: MySqlPool, timeout: Duration) -> Self {
        Self { pool, timeout }
    }
}

impl ReadinessProbe for MySqlReadinessProbe {
    async fn check(&self) -> Result<(), ReadinessError> {
        let span =
            Span::enter_with_local_parent("mysql.ready").with_property(|| ("span.kind", "client"));
        tokio::time::timeout(
            self.timeout,
            sqlx::query_scalar!("SELECT 1")
                .fetch_one(&self.pool)
                .in_span(span),
        )
        .await
        .map_err(|_| ReadinessError)?
        .map(|_| ())
        .map_err(|_| ReadinessError)
    }
}
