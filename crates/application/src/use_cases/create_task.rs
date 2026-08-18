use crate::{
    CreateTaskError, TaskPolicy, TaskPolicyDecision, TaskPolicyError, TaskPolicyInput,
    TaskRepository, TaskView,
};
use domain::{
    AssigneeId, NewTask, Task, TaskDescription, TaskEstimateMinutes, TaskPriority, TaskTitle,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateTaskCommand {
    title: TaskTitle,
    description: Option<TaskDescription>,
    priority: TaskPriority,
    assignee_id: Option<AssigneeId>,
    estimate_minutes: Option<TaskEstimateMinutes>,
}

impl CreateTaskCommand {
    pub fn new(
        title: TaskTitle,
        description: Option<TaskDescription>,
        priority: TaskPriority,
        assignee_id: Option<AssigneeId>,
        estimate_minutes: Option<TaskEstimateMinutes>,
    ) -> Self {
        Self {
            title,
            description,
            priority,
            assignee_id,
            estimate_minutes,
        }
    }

    fn policy_input(&self) -> TaskPolicyInput<'_> {
        TaskPolicyInput {
            title: &self.title,
            description: self.description.as_ref(),
            priority: self.priority,
            assignee_id: self.assignee_id,
            estimate_minutes: self.estimate_minutes,
        }
    }

    fn into_new_task(self) -> NewTask {
        NewTask {
            title: self.title,
            description: self.description,
            priority: self.priority,
            assignee_id: self.assignee_id,
            estimate_minutes: self.estimate_minutes,
        }
    }
}

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

    pub async fn execute(&self, command: CreateTaskCommand) -> Result<TaskView, CreateTaskError> {
        let decision = self
            .policy
            .evaluate(command.policy_input())
            .await
            .map_err(|error| match error {
                TaskPolicyError::Unavailable => CreateTaskError::PolicyUnavailable,
                TaskPolicyError::BadResponse => CreateTaskError::PolicyBadResponse,
            })?;

        if decision == TaskPolicyDecision::Rejected {
            return Err(CreateTaskError::PolicyRejected);
        }

        let task = Task::create(command.into_new_task());
        self.repository
            .insert(&task)
            .await
            .map_err(|_| CreateTaskError::Persistence)?;

        Ok(task.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{TaskPolicyError, TaskRepositoryError};
    use domain::TaskId;
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
        result: Result<TaskPolicyDecision, TaskPolicyError>,
        calls: Arc<Mutex<Vec<String>>>,
    }

    impl TaskPolicy for FakePolicy {
        fn evaluate(
            &self,
            input: TaskPolicyInput<'_>,
        ) -> impl Future<Output = Result<TaskPolicyDecision, TaskPolicyError>> + Send {
            self.calls
                .lock()
                .unwrap()
                .push(format!("policy:{}:{}", input.title, input.priority));
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
                "insert:{}:{}:{}",
                task.id(),
                task.title(),
                task.status()
            ));
            ready(self.result)
        }

        fn find(
            &self,
            _task_id: &TaskId,
        ) -> impl Future<Output = Result<Option<Task>, TaskRepositoryError>> + Send {
            ready(Ok(None))
        }
    }

    fn command() -> CreateTaskCommand {
        CreateTaskCommand::new(
            "  Build 模板  ".parse().unwrap(),
            Some("Document conversions".parse().unwrap()),
            TaskPriority::High,
            None,
            Some(90.try_into().unwrap()),
        )
    }

    fn use_case(
        policy_result: Result<TaskPolicyDecision, TaskPolicyError>,
        repository_result: Result<(), TaskRepositoryError>,
    ) -> (
        CreateTask<FakePolicy, FakeRepository>,
        Arc<Mutex<Vec<String>>>,
    ) {
        let calls = Arc::new(Mutex::new(Vec::new()));
        (
            CreateTask::new(
                FakePolicy {
                    result: policy_result,
                    calls: calls.clone(),
                },
                FakeRepository {
                    result: repository_result,
                    calls: calls.clone(),
                },
            ),
            calls,
        )
    }

    #[test]
    fn checks_policy_then_persists_typed_domain_state() {
        let (use_case, calls) = use_case(Ok(TaskPolicyDecision::Allowed), Ok(()));

        let view = block_on(use_case.execute(command())).unwrap();

        assert_eq!(view.title.as_str(), "Build 模板");
        assert_eq!(view.status.to_string(), "pending");
        assert_eq!(
            &*calls.lock().unwrap(),
            &[
                "policy:Build 模板:high".to_owned(),
                format!("insert:{}:Build 模板:pending", view.id),
            ]
        );
    }

    #[test]
    fn rejection_short_circuits_persistence() {
        let (use_case, calls) = use_case(Ok(TaskPolicyDecision::Rejected), Ok(()));

        assert_eq!(
            block_on(use_case.execute(command())),
            Err(CreateTaskError::PolicyRejected)
        );
        assert_eq!(&*calls.lock().unwrap(), &["policy:Build 模板:high"]);
    }

    #[test]
    fn maps_stable_port_failure_categories() {
        let (unavailable, _) = use_case(Err(TaskPolicyError::Unavailable), Ok(()));
        assert_eq!(
            block_on(unavailable.execute(command())),
            Err(CreateTaskError::PolicyUnavailable)
        );

        let (bad_response, _) = use_case(Err(TaskPolicyError::BadResponse), Ok(()));
        assert_eq!(
            block_on(bad_response.execute(command())),
            Err(CreateTaskError::PolicyBadResponse)
        );

        let (persistence, _) = use_case(
            Ok(TaskPolicyDecision::Allowed),
            Err(TaskRepositoryError::Unavailable),
        );
        assert_eq!(
            block_on(persistence.execute(command())),
            Err(CreateTaskError::Persistence)
        );
    }
}
