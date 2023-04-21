use crate::core::repository::{RunnerId, RunnerRepository};
use crate::models::runner::{
    NewNewRunner, Runner, RunnerRegistrationData, ShippingData, VerificationCode,
};

pub trait RunnerService {
    fn register_runner(
        &self,
        runner_registration_data: RunnerRegistrationData,
    ) -> anyhow::Result<Runner>;
    fn find_runner_by_id(&self, id: RunnerId) -> Option<Runner>;
}

pub struct DefaultRunnerService<RR: RunnerRepository> {
    runner_repository: RR,
}

impl<RR: RunnerRepository> DefaultRunnerService<RR> {
    const VERIFICATION_CODE_LENGTH: usize = 64;

    pub fn new(runner_repository: RR) -> Self {
        DefaultRunnerService { runner_repository }
    }

    fn get_t_shirt_cost_for_shipping_data(&self, shipping_data: &ShippingData) -> &'static str {
        use crate::EU_COUNTRIES;

        if shipping_data.country == "Deutschland" {
            "15"
        } else if EU_COUNTRIES.contains(&shipping_data.country.as_str()) {
            "17"
        } else {
            "20"
        }
    }

    fn generate_verification_code(&self) -> VerificationCode {
        use rand::distributions::DistString;

        rand::distributions::Alphanumeric
            .sample_string(&mut rand::thread_rng(), Self::VERIFICATION_CODE_LENGTH)
    }
}

impl<RR: RunnerRepository> RunnerService for DefaultRunnerService<RR> {
    fn register_runner(
        &self,
        runner_registration_data: RunnerRegistrationData,
    ) -> anyhow::Result<Runner> {
        let start_number = self.runner_repository.get_next_start_number();
        let payment_reference = self.runner_repository.generate_unique_payment_reference();
        let verification_code = self.generate_verification_code();

        let t_shirt_cost = runner_registration_data
            .shipping_data
            .as_ref()
            .map_or("0", |sd| self.get_t_shirt_cost_for_shipping_data(sd));

        let new_runner = NewNewRunner::new(
            runner_registration_data,
            start_number,
            payment_reference,
            verification_code,
            t_shirt_cost.to_string(),
        )
        .unwrap();

        self.runner_repository.insert_new_runner(new_runner)
    }

    fn find_runner_by_id(&self, id: RunnerId) -> Option<Runner> {
        self.runner_repository.find_runner_by_id(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::repository::MockRunnerRepository;
    use crate::models::runner::PaymentReference;
    use mockall::predicate::*;

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
    fn register_runner_should_assign_a_starting_number_to_the_runner() {
        let unregistered_runner = RunnerRegistrationData {
            firstname: None,
            lastname: None,
            team: None,
            email: None,
            starting_point: "".to_string(),
            running_level: "".to_string(),
            donation: "".to_string(),
            shipping_data: None,
        };

        let mut runner_repository = MockRunnerRepository::new();

        runner_repository
            .expect_get_next_start_number()
            .times(1)
            .returning(|| 73i64.try_into().unwrap());

        runner_repository
            .expect_generate_unique_payment_reference()
            .times(1)
            .returning(|| PaymentReference::random());

        runner_repository
            .expect_insert_new_runner()
            .times(1)
            .returning(|nr| {
                Ok(Runner {
                    id: 0,
                    start_number: nr.start_number().into(),
                    firstname: None,
                    lastname: None,
                    team: None,
                    email: None,
                    starting_point: "".to_string(),
                    running_level: "".to_string(),
                    donation: "".to_string(),
                    reason_for_payment: "".to_string(),
                    payment_status: false,
                    verification_code: "".to_string(),
                    payment_confirmation_mail_sent: false,
                    tshirt_cost: "".to_string(),
                })
            });

        let runner_service = DefaultRunnerService::new(runner_repository);
        let result = runner_service
            .register_runner(unregistered_runner)
            .expect("Unable to register runner");

        assert_eq!(result.start_number, 73i64)
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
