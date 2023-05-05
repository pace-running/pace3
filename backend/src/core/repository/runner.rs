use crate::models::runner::{NewRunner, PaymentReference, Runner};
use crate::models::shipping::Shipping;
use crate::models::start_number::StartNumber;
#[cfg(test)]
use mockall::automock;

pub type RunnerId = i32;

#[cfg_attr(test, automock)]
pub trait RunnerRepository {
    fn insert_new_runner(&self, new_runner: NewRunner) -> anyhow::Result<Runner>;
    fn get_next_start_number(&self) -> StartNumber;
    fn generate_unique_payment_reference(&self) -> PaymentReference;
    fn find_runner_by_id(&self, id: RunnerId) -> Option<Runner>;
    fn find_shipping_by_runner_id(&self, id: RunnerId) -> Option<Shipping>;
}
