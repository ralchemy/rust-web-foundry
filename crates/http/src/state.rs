use application::ReadinessProbe;
use axum::extract::FromRef;

#[cfg(feature = "reference-task")]
use application::{CreateTask, GetTask, StartTask, TaskPolicy, TaskRepository};

#[derive(Clone)]
pub(crate) struct HealthState<H>(pub(crate) H);

#[cfg(feature = "reference-task")]
#[derive(Clone)]
pub(crate) struct HttpState<P, R, H> {
    task: TaskState<P, R>,
    health: HealthState<H>,
}

#[cfg(feature = "reference-task")]
#[derive(Clone)]
pub(crate) struct TaskState<P, R> {
    pub(crate) create: CreateTask<P, R>,
    pub(crate) get: GetTask<R>,
    pub(crate) start: StartTask<R>,
}

#[cfg(feature = "reference-task")]
impl<P, R, H> HttpState<P, R, H>
where
    P: TaskPolicy,
    R: TaskRepository,
    H: ReadinessProbe,
{
    pub(crate) fn new(create_task: CreateTask<P, R>, get_task: GetTask<R>, start_task: StartTask<R>, readiness: H) -> Self {
        Self { task: TaskState { create: create_task, get: get_task, start: start_task }, health: HealthState(readiness) }
    }
}

#[cfg(feature = "reference-task")]
impl<P, R, H> FromRef<HttpState<P, R, H>> for TaskState<P, R>
where P: TaskPolicy, R: TaskRepository {
    fn from_ref(state: &HttpState<P, R, H>) -> Self { state.task.clone() }
}

#[cfg(feature = "reference-task")]
impl<P, R, H> FromRef<HttpState<P, R, H>> for HealthState<H>
where H: ReadinessProbe {
    fn from_ref(state: &HttpState<P, R, H>) -> Self { state.health.clone() }
}
