use crate::core::repository::{PaymentRepository, RunnerId};
use crate::core::service::EmailService;
use crate::models::runner::Runner;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::sync::Arc;

pub enum PaymentStatus {
    Pending,
    Paid,
    // Partial(i32),
}

impl From<PaymentStatus> for bool {
    fn from(value: PaymentStatus) -> Self {
        match value {
            PaymentStatus::Pending => false,
            PaymentStatus::Paid => true,
        }
    }
}

#[derive(Debug)]
pub enum PaymentUpdateError {
    UserNotFound,
    UnableToSendEmail,
}

impl Display for PaymentUpdateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PaymentUpdateError::UserNotFound => write!(f, "PaymentUpdateError::UserNotFound"),
            PaymentUpdateError::UnableToSendEmail => {
                write!(f, "PaymentUpdateError::UnableToSendEmail")
            }
        }
    }
}

impl Error for PaymentUpdateError {}

pub trait PaymentService {
    fn set_payment_status(
        &self,
        runner_id: RunnerId,
        payment_status: PaymentStatus,
    ) -> anyhow::Result<Runner, PaymentUpdateError>;
    fn delete_rejected_transactions(&self, rejected_transaction_ids: Vec<i32>) -> usize;
}

pub struct DefaultPaymentService<RR: PaymentRepository, ES: EmailService + ?Sized> {
    payment_repository: RR,
    email_service: Arc<ES>,
}

impl<RR: PaymentRepository, ES: EmailService + ?Sized> DefaultPaymentService<RR, ES> {
    pub fn new(payment_repository: RR, email_service: Arc<ES>) -> Self {
        DefaultPaymentService {
            payment_repository,
            email_service,
        }
    }
}

impl<RR: PaymentRepository, ES: EmailService + ?Sized> PaymentService
    for DefaultPaymentService<RR, ES>
{
    fn set_payment_status(
        &self,
        runner_id: RunnerId,
        payment_status: PaymentStatus,
    ) -> anyhow::Result<Runner, PaymentUpdateError> {
        let runner: Runner = self
            .payment_repository
            .set_payment_status(runner_id, payment_status)
            .ok_or(PaymentUpdateError::UserNotFound)?;

        if runner.payment_status && runner.email.is_some() {
            self.email_service
                .send_payment_confirmation(runner.clone())
                .map(|_| runner)
                .map_err(|_| PaymentUpdateError::UnableToSendEmail)
        } else {
            Ok(runner)
        }
    }

    fn delete_rejected_transactions(&self, rejected_transaction_ids: Vec<i32>) -> usize {
        self.payment_repository
            .delete_rejected_transactions_by_ids(rejected_transaction_ids)
    }
}
