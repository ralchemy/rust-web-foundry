use crate::state::HealthState;
use application::ReadinessProbe;
use axum::{extract::State, http::StatusCode};

pub(crate) async fn live() -> StatusCode {
    StatusCode::OK
}

pub(crate) async fn ready<H>(State(HealthState(readiness)): State<HealthState<H>>) -> StatusCode
where
    H: ReadinessProbe,
{
    match readiness.check().await {
        Ok(()) => StatusCode::OK,
        Err(_) => StatusCode::SERVICE_UNAVAILABLE,
    }
}
