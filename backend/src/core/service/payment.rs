use crate::core::repository::PaymentRepository;

pub trait PaymentService {
    fn delete_rejected_transactions(&self, rejected_transaction_ids: Vec<i32>) -> usize;
}

pub struct DefaultPaymentService<RR: PaymentRepository> {
    payment_repository: RR,
}

impl<RR: PaymentRepository> DefaultPaymentService<RR> {
    pub fn new(payment_repository: RR) -> Self {
        DefaultPaymentService { payment_repository }
    }
}

impl<RR: PaymentRepository> PaymentService for DefaultPaymentService<RR> {
    fn delete_rejected_transactions(&self, rejected_transaction_ids: Vec<i32>) -> usize {
        self.payment_repository
            .delete_rejected_transactions_by_ids(rejected_transaction_ids)
    }
}
