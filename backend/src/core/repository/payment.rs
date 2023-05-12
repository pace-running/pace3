use crate::core::repository::RunnerId;
use crate::core::service::PaymentStatus;
use crate::models::runner::Runner;
#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait PaymentRepository {
    fn set_payment_status(
        &self,
        runner_id: RunnerId,
        payment_status: PaymentStatus,
    ) -> Option<Runner>;
    fn delete_rejected_transactions_by_ids(&self, rejected_transactions_ids: Vec<i32>) -> usize;
}
