use crate::{CreateTaskError, TaskPolicy, TaskPolicyError, TaskRepository};
use domain::{Task, TaskTitle};

#[derive(Clone)]
pub struct CreateTask<P, R> {
    policy: P,
    repository: R,
}

impl<P, R> CreateTask<P, R>
where
    P: TaskPolicy,
    R: TaskRepository,
{
    pub fn new(policy: P, repository: R) -> Self {
        Self { policy, repository }
    }

    pub async fn execute(&self, title: TaskTitle) -> Result<Task, CreateTaskError> {
        let allowed = self
            .policy
            .is_allowed(&title)
            .await
            .map_err(|error| match error {
                TaskPolicyError::Unavailable => CreateTaskError::PolicyUnavailable,
                TaskPolicyError::BadResponse => CreateTaskError::PolicyBadResponse,
            })?;
        if !allowed {
            return Err(CreateTaskError::PolicyRejected);
        }

        let task = Task::new(title);
        self.repository
            .insert(&task)
            .await
            .map_err(|_| CreateTaskError::Persistence)?;
        Ok(task)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TaskRepositoryError;
    use std::{
        future::{Future, ready},
        sync::{Arc, Mutex},
        task::{Context, Poll, Waker},
    };

    fn block_on<F: Future>(future: F) -> F::Output {
        let mut future = std::pin::pin!(future);
        let waker = Waker::noop();
        let mut context = Context::from_waker(waker);
        match future.as_mut().poll(&mut context) {
            Poll::Ready(output) => output,
            Poll::Pending => panic!("fake ports must complete immediately"),
        }
    }

    #[derive(Clone)]
    struct FakePolicy {
        result: Result<bool, TaskPolicyError>,
        calls: Arc<Mutex<Vec<String>>>,
    }

    impl TaskPolicy for FakePolicy {
        fn is_allowed(
            &self,
            title: &TaskTitle,
        ) -> impl Future<Output = Result<bool, TaskPolicyError>> + Send {
            self.calls
                .lock()
                .unwrap()
                .push(format!("policy:{}", title.as_str()));
            ready(self.result)
        }
    }

    #[derive(Clone)]
    struct FakeRepository {
        result: Result<(), TaskRepositoryError>,
        calls: Arc<Mutex<Vec<String>>>,
    }

    impl TaskRepository for FakeRepository {
        fn insert(
            &self,
            task: &Task,
        ) -> impl Future<Output = Result<(), TaskRepositoryError>> + Send {
            self.calls.lock().unwrap().push(format!(
                "insert:{}:{}",
                task.id(),
                task.title().as_str()
            ));
            ready(self.result)
        }
    }

    fn use_case(
        policy_result: Result<bool, TaskPolicyError>,
        repository_result: Result<(), TaskRepositoryError>,
    ) -> (
        CreateTask<FakePolicy, FakeRepository>,
        Arc<Mutex<Vec<String>>>,
    ) {
        let calls = Arc::new(Mutex::new(Vec::new()));
        let policy = FakePolicy {
            result: policy_result,
            calls: calls.clone(),
        };
        let repository = FakeRepository {
            result: repository_result,
            calls: calls.clone(),
        };
        (CreateTask::new(policy, repository), calls)
    }

    fn title(raw: &str) -> TaskTitle {
        TaskTitle::parse(raw).expect("valid Task Title")
    }

    #[test]
    fn checks_policy_then_persists_the_same_typed_title() {
        let (use_case, calls) = use_case(Ok(true), Ok(()));

        let task = block_on(use_case.execute(title("  Build 模板  "))).unwrap();

        let calls = calls.lock().unwrap();
        assert_eq!(calls[0], "policy:Build 模板");
        assert_eq!(calls[1], format!("insert:{}:Build 模板", task.id()));
    }

    #[test]
    fn rejected_titles_never_reach_persistence() {
        let (rejected, rejected_calls) = use_case(Ok(false), Ok(()));
        assert_eq!(
            block_on(rejected.execute(title("No"))),
            Err(CreateTaskError::PolicyRejected)
        );
        assert_eq!(&*rejected_calls.lock().unwrap(), &["policy:No"]);
    }

    #[test]
    fn maps_stable_port_failure_categories() {
        let (unavailable, _) = use_case(Err(TaskPolicyError::Unavailable), Ok(()));
        assert_eq!(
            block_on(unavailable.execute(title("Task"))),
            Err(CreateTaskError::PolicyUnavailable)
        );

        let (bad_response, _) = use_case(Err(TaskPolicyError::BadResponse), Ok(()));
        assert_eq!(
            block_on(bad_response.execute(title("Task"))),
            Err(CreateTaskError::PolicyBadResponse)
        );

        let (persistence, _) = use_case(Ok(true), Err(TaskRepositoryError));
        assert_eq!(
            block_on(persistence.execute(title("Task"))),
            Err(CreateTaskError::Persistence)
        );
    }
}
