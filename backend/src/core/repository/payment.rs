#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait PaymentRepository {
    fn delete_rejected_transactions_by_ids(&self, rejected_transactions_ids: Vec<i32>) -> usize;
}
