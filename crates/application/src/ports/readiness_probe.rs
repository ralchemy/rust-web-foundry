use crate::ReadinessError;
use std::future::Future;

pub trait ReadinessProbe: Clone + Send + Sync + 'static {
    fn check(&self) -> impl Future<Output = Result<(), ReadinessError>> + Send;
}
