use crate::core::repository::{RunnerId, RunnerRepository};
use crate::models::runner::{NewRunner, Runner};

pub trait RunnerService {
    fn add_runner(&self, new_runner: NewRunner) -> Runner;
    fn find_runner_by_id(&self, id: RunnerId) -> Option<Runner>;
}

pub struct DefaultRunnerService<RR: RunnerRepository> {
    runner_repository: RR,
}

impl<RR: RunnerRepository> DefaultRunnerService<RR> {
    pub fn new(runner_repository: RR) -> Self {
        DefaultRunnerService { runner_repository }
    }
}

impl<RR: RunnerRepository> RunnerService for DefaultRunnerService<RR> {
    fn add_runner(&self, new_runner: NewRunner) -> Runner {
        self.runner_repository.insert_runner(new_runner)
    }

    fn find_runner_by_id(&self, id: RunnerId) -> Option<Runner> {
        self.runner_repository.find_runner_by_id(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::repository::MockRunnerRepository;
    use mockall::predicate::*;

    const EXAMPLE_NEW_RUNNER: NewRunner = NewRunner {
        start_number: 9000,
        firstname: None,
        lastname: None,
        team: None,
        email: None,
        starting_point: "",
        running_level: "",
        donation: "",
        reason_for_payment: "",
        payment_status: &false,
        verification_code: "",
        tshirt_cost: "",
    };

    const EXAMPLE_RUNNER: Runner = Runner {
        id: 42,
        start_number: 9000,
        firstname: None,
        lastname: None,
        team: None,
        email: None,
        starting_point: String::new(),
        running_level: String::new(),
        donation: String::new(),
        reason_for_payment: String::new(),
        payment_status: false,
        verification_code: String::new(),
        payment_confirmation_mail_sent: false,
        tshirt_cost: String::new(),
    };

    #[test]
    fn add_runner_must_call_repository() {
        let mut runner_repository = MockRunnerRepository::new();

        runner_repository
            .expect_insert_runner()
            .times(1)
            .returning(|_r| EXAMPLE_RUNNER);

        let runner_service = DefaultRunnerService::new(runner_repository);

        let result = runner_service.add_runner(EXAMPLE_NEW_RUNNER);

        assert_eq!(result, EXAMPLE_RUNNER)
    }

    #[test]
    fn find_runner_by_id_must_call_repository() {
        let mut runner_repository = MockRunnerRepository::new();
        runner_repository
            .expect_find_runner_by_id()
            .with(eq(EXAMPLE_RUNNER.id))
            .times(1)
            .returning(|_r| Some(EXAMPLE_RUNNER));

        let runner_service = DefaultRunnerService::new(runner_repository);
        let result = runner_service.find_runner_by_id(EXAMPLE_RUNNER.id);

        assert_eq!(result, Some(EXAMPLE_RUNNER))
    }
}
