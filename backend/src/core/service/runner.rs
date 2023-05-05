use crate::core::repository::{RunnerId, RunnerRepository};
use crate::core::service::EmailService;
use crate::models::runner::{
    NewRunner, Runner, RunnerRegistrationData, ShippingData, VerificationCode,
};
use std::sync::Arc;

pub trait RunnerService {
    fn register_runner(
        &self,
        runner_registration_data: RunnerRegistrationData,
    ) -> anyhow::Result<Runner>;

    fn find_runner_by_id(&self, id: RunnerId) -> Option<Runner>;

    fn find_runner_by_id_and_verification_code(
        &self,
        id: RunnerId,
        verification_code: &str,
    ) -> Option<Runner>;
}

pub struct DefaultRunnerService<RR: RunnerRepository, ES: EmailService + ?Sized> {
    runner_repository: RR,
    email_service: Arc<ES>,
}

impl<RR: RunnerRepository, ES: EmailService + ?Sized> DefaultRunnerService<RR, ES> {
    const VERIFICATION_CODE_LENGTH: usize = 64;

    pub fn new(runner_repository: RR, email_service: Arc<ES>) -> Self {
        DefaultRunnerService {
            runner_repository,
            email_service,
        }
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

impl<RR: RunnerRepository, ES: EmailService + ?Sized> RunnerService
    for DefaultRunnerService<RR, ES>
{
    fn register_runner(
        &self,
        runner_registration_data: RunnerRegistrationData,
    ) -> anyhow::Result<Runner> {
        let start_number = self.runner_repository.get_next_start_number();
        let payment_reference = self.runner_repository.generate_unique_payment_reference();
        let verification_code = self.generate_verification_code();

        let t_shirt_cost = runner_registration_data
            .shipping_data()
            .as_ref()
            .map_or("0", |sd| self.get_t_shirt_cost_for_shipping_data(sd));

        let new_runner = NewRunner::new(
            runner_registration_data,
            start_number,
            payment_reference,
            verification_code,
            t_shirt_cost.to_string(),
        )
        .unwrap();

        self.runner_repository
            .insert_new_runner(new_runner)
            .and_then(|r| {
                if r.email.is_some() {
                    self.email_service
                        .send_registration_confirmation(r.clone())?;
                }
                Ok(r)
            })
    }

    fn find_runner_by_id(&self, id: RunnerId) -> Option<Runner> {
        self.runner_repository.find_runner_by_id(id)
    }

    fn find_runner_by_id_and_verification_code(
        &self,
        id: RunnerId,
        verification_code: &str,
    ) -> Option<Runner> {
        self.find_runner_by_id(id)
            .filter(|r| r.verification_code == verification_code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::repository::MockRunnerRepository;
    use crate::core::service::MockEmailService;
    use crate::models::donation::Donation;
    use crate::models::runner::PaymentReference;
    use mockall::predicate::*;

    const EXAMPLE_RUNNER: Runner = Runner {
        id: 42,
        start_number: 9000,
        firstname: None,
        lastname: None,
        team: None,
        bsv_participant: false,
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
        let unregistered_runner = RunnerRegistrationData::new(
            None,
            None,
            None,
            false,
            None,
            "".to_string(),
            "".to_string(),
            Donation::try_from(5).unwrap(),
            None,
        );

        let mut email_service = MockEmailService::new();
        email_service
            .expect_send_registration_confirmation()
            .times(0);

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
                    bsv_participant: false,
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

        let runner_service = DefaultRunnerService::new(runner_repository, Arc::new(email_service));
        let result = runner_service
            .register_runner(unregistered_runner)
            .expect("Unable to register runner");

        assert_eq!(result.start_number, 73i64)
    }

    #[test]
    fn register_runner_should_send_registration_confirmation_if_email_address_is_provided() {
        let unregistered_runner = RunnerRegistrationData::new(
            None,
            None,
            None,
            false,
            Some("runner@whatever.com".to_string()),
            "".to_string(),
            "".to_string(),
            Donation::try_from(5).unwrap(),
            None,
        );

        let start_number = 73i64;

        let runner = Runner {
            id: 0,
            start_number,
            firstname: None,
            lastname: None,
            team: None,
            bsv_participant: false,
            email: Some("runner@whatever.com".to_string()),
            starting_point: "".to_string(),
            running_level: "".to_string(),
            donation: "5".to_string(),
            reason_for_payment: "".to_string(),
            payment_status: false,
            verification_code: "".to_string(),
            payment_confirmation_mail_sent: false,
            tshirt_cost: "".to_string(),
        };

        let mut email_service = MockEmailService::new();

        email_service
            .expect_send_registration_confirmation()
            .with(eq(runner.clone()))
            .times(1)
            .returning(|_r| Ok(()));

        let mut runner_repository = MockRunnerRepository::new();

        runner_repository
            .expect_get_next_start_number()
            .times(1)
            .returning(move || start_number.try_into().unwrap());

        runner_repository
            .expect_generate_unique_payment_reference()
            .times(1)
            .returning(|| PaymentReference::random());

        runner_repository
            .expect_insert_new_runner()
            .times(1)
            .returning(move |_nr| Ok(runner.clone()));

        let runner_service = DefaultRunnerService::new(runner_repository, Arc::new(email_service));
        let result = runner_service
            .register_runner(unregistered_runner)
            .expect("Unable to register runner");

        assert_eq!(result.start_number, start_number)
    }

    #[test]
    fn find_runner_by_id_must_call_repository() {
        let mut runner_repository = MockRunnerRepository::new();
        runner_repository
            .expect_find_runner_by_id()
            .with(eq(EXAMPLE_RUNNER.id))
            .times(1)
            .returning(|_r| Some(EXAMPLE_RUNNER));

        let mock_email_service = MockEmailService::new();

        let runner_service =
            DefaultRunnerService::new(runner_repository, Arc::new(mock_email_service));
        let result = runner_service.find_runner_by_id(EXAMPLE_RUNNER.id);

        assert_eq!(result, Some(EXAMPLE_RUNNER))
    }

    #[test]
    fn find_runner_by_id_and_verification_code_must_not_return_runner_if_verification_code_does_not_match(
    ) {
        let mut runner_repository = MockRunnerRepository::new();
        runner_repository
            .expect_find_runner_by_id()
            .with(eq(EXAMPLE_RUNNER.id))
            .times(1)
            .returning(|_r| Some(EXAMPLE_RUNNER));

        let mock_email_service = MockEmailService::new();

        let runner_service =
            DefaultRunnerService::new(runner_repository, Arc::new(mock_email_service));

        let result = runner_service
            .find_runner_by_id_and_verification_code(EXAMPLE_RUNNER.id, "not-the-right-value");

        assert_eq!(result, None);
    }

    #[test]
    fn find_runner_by_id_and_verification_code_must_return_runner_if_verification_code_matches() {
        let mut runner_repository = MockRunnerRepository::new();
        runner_repository
            .expect_find_runner_by_id()
            .with(eq(EXAMPLE_RUNNER.id))
            .times(1)
            .returning(|_r| Some(EXAMPLE_RUNNER));

        let mock_email_service = MockEmailService::new();

        let runner_service =
            DefaultRunnerService::new(runner_repository, Arc::new(mock_email_service));

        let result = runner_service.find_runner_by_id_and_verification_code(
            EXAMPLE_RUNNER.id,
            &EXAMPLE_RUNNER.verification_code,
        );

        assert_eq!(result, Some(EXAMPLE_RUNNER));
    }
}
