use crate::core::repository::{RunnerId, RunnerRepository, ThemeRepository};
use crate::core::service::EmailService;
use crate::models::runner::{
    NewRunner, Runner, RunnerRegistrationData, RunnerUpdateData, ShippingData, VerificationCode,
};
use crate::models::shipping::Shipping;
use std::cmp::min;
use std::num::TryFromIntError;
use std::sync::Arc;

pub enum RunnerSearchFilter {
    StartNumberEquals(i64),
    FullNameContaining(String),
    EmailContaining(String),
    PaymentReferenceContaining(String),
}

impl RunnerSearchFilter {
    pub fn from_category_and_keyword(
        category: &str,
        keyword: &str, // talisman-ignore-line
    ) -> anyhow::Result<Self> {
        match category {
            "start_number" => Ok(Self::StartNumberEquals(keyword.parse()?)),
            "name" => Ok(Self::FullNameContaining(keyword.to_string())),
            "email" => Ok(Self::EmailContaining(keyword.to_string())),
            "reason_for_payment" => Ok(Self::PaymentReferenceContaining(keyword.to_string())),
            unknown_category => Err(anyhow::Error::msg(format!(
                "Unknown category: {}",
                unknown_category
            ))),
        }
    }
}

pub struct PageParameters {
    page_number: usize,
    page_size: usize,
}

impl PageParameters {
    const DEFAULT_PAGE_SIZE: usize = 15;

    pub fn new(page_number: usize) -> Self {
        Self {
            page_number,
            page_size: Self::DEFAULT_PAGE_SIZE,
        }
    }
}

impl TryFrom<i32> for PageParameters {
    type Error = TryFromIntError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        usize::try_from(value).map(PageParameters::new)
    }
}

pub struct RunnerSearchParameters {
    search_filter: RunnerSearchFilter,
    bsv_participant_filter: Option<bool>,
    page_parameters: PageParameters,
}

impl RunnerSearchParameters {
    pub fn new(
        search_filter: RunnerSearchFilter,
        bsv_participant_filter: Option<bool>,
        page_parameters: PageParameters,
    ) -> Self {
        Self {
            search_filter,
            bsv_participant_filter,
            page_parameters,
        }
    }
}

pub struct RunnerSearchResultStats {
    pub count_total_results: usize,
    pub count_starting_point: usize,
    pub count_donations: i32,
}

pub struct PageInfo {
    pub page_size: usize,
    pub current_page: usize,
    pub last_page: usize,
}

pub struct RunnerSearchResult {
    pub runners: Vec<Runner>,
    pub stats: RunnerSearchResultStats,
    pub page_info: PageInfo,
}

pub trait RunnerService {
    fn register_runner(
        &self,
        runner_registration_data: RunnerRegistrationData,
    ) -> anyhow::Result<Runner>;

    fn update_runner(
        &self,
        runner_id: RunnerId,
        runner_update_data: RunnerUpdateData,
    ) -> anyhow::Result<Runner>;

    fn find_runner_by_id(&self, id: RunnerId) -> Option<Runner>;

    fn find_runner_by_id_and_verification_code(
        &self,
        id: RunnerId,
        verification_code: &str,
    ) -> Option<Runner>;

    fn find_runners_by_search_parameters(
        &self,
        search_parameters: RunnerSearchParameters,
    ) -> RunnerSearchResult;

    fn find_shipping_by_runner_id(&self, runner_id: RunnerId) -> Option<Shipping>;
}

pub struct DefaultRunnerService<
    RR: RunnerRepository,
    TR: ThemeRepository + ?Sized,
    ES: EmailService + ?Sized,
> {
    runner_repository: RR,
    theme_repository: Arc<TR>,
    email_service: Arc<ES>,
}

impl<RR: RunnerRepository, TR: ThemeRepository + ?Sized, ES: EmailService + ?Sized>
    DefaultRunnerService<RR, TR, ES>
{
    const VERIFICATION_CODE_LENGTH: usize = 64;

    pub fn new(runner_repository: RR, theme_repository: Arc<TR>, email_service: Arc<ES>) -> Self {
        DefaultRunnerService {
            runner_repository,
            theme_repository,
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

impl<RR: RunnerRepository, TR: ThemeRepository + ?Sized, ES: EmailService + ?Sized> RunnerService
    for DefaultRunnerService<RR, TR, ES>
{
    fn register_runner(
        &self,
        runner_registration_data: RunnerRegistrationData,
    ) -> anyhow::Result<Runner> {
        let is_registration_open = self
            .theme_repository
            .get_theme_value("is_registration_open")?
            .unwrap_or_else(|| {
                log::warn!(
                    "No value set for theme setting `is_registration_open`. Defaulting to `false`."
                );
                "false".to_string()
            })
            .parse::<bool>()?;

        if !is_registration_open {
            return Err(anyhow::Error::msg("Registration is not enabled!"));
        }

        if runner_registration_data.shipping_data().is_some() {
            let is_t_shirt_order_enabled = self
                .theme_repository
                .get_theme_value("enable_tshirts")?
                .unwrap_or_else(|| {
                    log::warn!(
                        "No value set for theme setting `enable_tshirts`. Defaulting to `false`."
                    );
                    "false".to_string()
                })
                .parse::<bool>()?;

            if !is_t_shirt_order_enabled {
                return Err(anyhow::Error::msg(
                    "T-Shirt ordering is not enabled but shipping data was still provided!",
                ));
            }
        }

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

    fn update_runner(
        &self,
        runner_id: RunnerId,
        runner_update_data: RunnerUpdateData,
    ) -> anyhow::Result<Runner> {
        self.runner_repository
            .update_runner(runner_id, runner_update_data)

        // TODO send email if changed?
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

    fn find_runners_by_search_parameters(
        &self,
        search_parameters: RunnerSearchParameters,
    ) -> RunnerSearchResult {
        let mut runners: Vec<Runner> = match search_parameters.search_filter {
            RunnerSearchFilter::StartNumberEquals(start_number) => self
                .runner_repository
                .find_runner_by_start_number(start_number, search_parameters.bsv_participant_filter)
                .map_or_else(Vec::new, |r| vec![r]),
            RunnerSearchFilter::FullNameContaining(search_text) => {
                self.runner_repository.find_runners_by_name_containing(
                    &search_text,
                    search_parameters.bsv_participant_filter,
                )
            }
            RunnerSearchFilter::EmailContaining(search_text) => {
                self.runner_repository.find_runners_by_email_containing(
                    &search_text,
                    search_parameters.bsv_participant_filter,
                )
            }
            RunnerSearchFilter::PaymentReferenceContaining(search_text) => self
                .runner_repository
                .find_runners_by_payment_reference_containing(
                    &search_text,
                    search_parameters.bsv_participant_filter,
                ),
        };

        if search_parameters.bsv_participant_filter.is_some() {
            runners.sort_by(|a, b| a.team.cmp(&b.team).then(a.id.cmp(&b.id)));
        } else {
            runners.sort_by(|a, b| a.id.cmp(&b.id));
        }

        let stats = runners.iter().fold((0, 0, 0), |acc, r| {
            let count_total = acc.0 + 1;
            let count_hamburg = if &r.starting_point == "hamburg" {
                acc.1 + 1
            } else {
                acc.1
            };
            let count_donation = acc.2 + r.donation.parse::<i32>().unwrap();
            (count_total, count_hamburg, count_donation)
        });

        let page_number = search_parameters.page_parameters.page_number;
        let page_size = search_parameters.page_parameters.page_size;

        let from = page_number * page_size;
        let to = min((page_number + 1) * page_size, stats.0);

        let last_page = if stats.0 == 0 {
            0
        } else {
            (stats.0 - 1) / page_size
        };

        RunnerSearchResult {
            runners: runners[from..to].to_vec(),
            stats: RunnerSearchResultStats {
                count_total_results: stats.0,
                count_starting_point: stats.1,
                count_donations: stats.2,
            },
            page_info: PageInfo {
                page_size,
                current_page: page_number,
                last_page,
            },
        }
    }

    fn find_shipping_by_runner_id(&self, runner_id: RunnerId) -> Option<Shipping> {
        self.runner_repository.find_shipping_by_runner_id(runner_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::repository::{MockRunnerRepository, MockThemeRepository};
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

        let theme_repository = MockThemeRepository::new();

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

        let runner_service = DefaultRunnerService::new(
            runner_repository,
            Arc::new(theme_repository),
            Arc::new(email_service),
        );
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

        let theme_repository = MockThemeRepository::new();

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

        let runner_service = DefaultRunnerService::new(
            runner_repository,
            Arc::new(theme_repository),
            Arc::new(email_service),
        );
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

        let theme_repository = MockThemeRepository::new();
        let email_service = MockEmailService::new();

        let runner_service = DefaultRunnerService::new(
            runner_repository,
            Arc::new(theme_repository),
            Arc::new(email_service),
        );
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

        let theme_repository = MockThemeRepository::new();
        let email_service = MockEmailService::new();

        let runner_service = DefaultRunnerService::new(
            runner_repository,
            Arc::new(theme_repository),
            Arc::new(email_service),
        );

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

        let theme_repository = MockThemeRepository::new();
        let email_service = MockEmailService::new();

        let runner_service = DefaultRunnerService::new(
            runner_repository,
            Arc::new(theme_repository),
            Arc::new(email_service),
        );

        let result = runner_service.find_runner_by_id_and_verification_code(
            EXAMPLE_RUNNER.id,
            &EXAMPLE_RUNNER.verification_code,
        );

        assert_eq!(result, Some(EXAMPLE_RUNNER));
    }
}
