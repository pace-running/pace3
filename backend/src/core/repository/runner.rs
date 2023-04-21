use crate::models::runner::{NewNewRunner, PaymentReference, Runner};
use crate::models::start_number::StartNumber;
#[cfg(test)]
use mockall::automock;

pub type RunnerId = i32;

#[cfg_attr(test, automock)]
pub trait RunnerRepository {
    fn insert_new_runner(&self, new_runner: NewNewRunner) -> anyhow::Result<Runner>;
    fn get_next_start_number(&self) -> StartNumber;
    fn generate_unique_payment_reference(&self) -> PaymentReference;
    fn find_runner_by_id(&self, id: RunnerId) -> Option<Runner>;
}
