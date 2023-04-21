use diesel::prelude::*;
use rand::distributions::{Alphanumeric, DistString};
use serde::Deserialize;
use serde::Serialize;

use crate::constants::VERIFICATION_CODE_LENGTH;

pub use crate::models::payment::PaymentReference;
use crate::models::start_number::StartNumber;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ShippingData {
    pub t_shirt_model: String,
    pub t_shirt_size: String,
    pub country: String,
    pub firstname: String,
    pub lastname: String,
    pub street_name: String,
    pub house_number: String,
    pub address_extra: Option<String>,
    pub postal_code: String,
    pub city: String,
}

impl ShippingData {
    pub fn t_shirt_model(&self) -> &str {
        &self.t_shirt_model
    }
    pub fn t_shirt_size(&self) -> &str {
        &self.t_shirt_size
    }
    pub fn country(&self) -> &str {
        &self.country
    }
    pub fn firstname(&self) -> &str {
        &self.firstname
    }
    pub fn lastname(&self) -> &str {
        &self.lastname
    }
    pub fn street_name(&self) -> &str {
        &self.street_name
    }
    pub fn house_number(&self) -> &str {
        &self.house_number
    }
    pub fn address_extra(&self) -> Option<&str> {
        self.address_extra.as_deref()
    }
    pub fn postal_code(&self) -> &str {
        &self.postal_code
    }
    pub fn city(&self) -> &str {
        &self.city
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct RunnerRegistrationData {
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub team: Option<String>,
    pub email: Option<String>,
    pub starting_point: String,
    pub running_level: String,
    pub donation: String,
    pub shipping_data: Option<ShippingData>,
}

pub type PaymentStatus = bool;
pub type VerificationCode = String;
pub type TShirtCost = String;

pub struct NewNewRunner {
    start_number: StartNumber,
    firstname: Option<String>,
    lastname: Option<String>,
    team: Option<String>,
    email: Option<String>,
    starting_point: String,
    running_level: String,
    donation: String,
    payment_reference: PaymentReference,
    verification_code: String,
    t_shirt_cost: String,
    shipping_data: Option<ShippingData>,
}

impl NewNewRunner {
    pub fn new(
        runner_registration_data: RunnerRegistrationData,
        start_number: StartNumber,
        payment_reference: PaymentReference,
        verification_code: VerificationCode,
        t_shirt_cost: TShirtCost,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            start_number,
            firstname: runner_registration_data.firstname,
            lastname: runner_registration_data.lastname,
            team: runner_registration_data.team,
            email: runner_registration_data.email,
            starting_point: runner_registration_data.starting_point,
            running_level: runner_registration_data.running_level,
            donation: runner_registration_data.donation,
            payment_reference,
            verification_code,
            t_shirt_cost,
            shipping_data: runner_registration_data.shipping_data,
        })
    }

    pub fn start_number(&self) -> &StartNumber {
        &self.start_number
    }
    pub fn firstname(&self) -> Option<&str> {
        self.firstname.as_deref()
    }
    pub fn lastname(&self) -> Option<&str> {
        self.lastname.as_deref()
    }
    pub fn team(&self) -> Option<&str> {
        self.team.as_deref()
    }
    pub fn email(&self) -> Option<&str> {
        self.email.as_deref()
    }
    pub fn starting_point(&self) -> &str {
        &self.starting_point
    }
    pub fn running_level(&self) -> &str {
        &self.running_level
    }
    pub fn donation(&self) -> &str {
        &self.donation
    }
    pub fn payment_reference(&self) -> &PaymentReference {
        &self.payment_reference
    }
    pub fn verification_code(&self) -> &str {
        &self.verification_code
    }
    pub fn t_shirt_cost(&self) -> &str {
        &self.t_shirt_cost
    }
    pub fn shipping_data(&self) -> &Option<ShippingData> {
        &self.shipping_data
    }
}

#[derive(Queryable, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Runner {
    pub id: i32,
    pub start_number: i64,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub team: Option<String>,
    pub email: Option<String>,
    pub starting_point: String,
    pub running_level: String,
    pub donation: String,
    pub reason_for_payment: String,
    pub payment_status: bool,
    pub verification_code: String,
    pub payment_confirmation_mail_sent: bool,
    pub tshirt_cost: String,
}

pub fn create_verification_code() -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), VERIFICATION_CODE_LENGTH)
}

#[cfg(test)]
mod tests {
    use crate::constants::VERIFICATION_CODE_LENGTH;

    use super::*;

    #[actix_web::test]
    async fn unit_create_verification_code() {
        let verification_code_1 = create_verification_code();
        let verification_code_2 = create_verification_code();

        assert_eq!(verification_code_1.len(), VERIFICATION_CODE_LENGTH);
        assert_eq!(verification_code_2.len(), VERIFICATION_CODE_LENGTH);
        assert_ne!(verification_code_1, verification_code_2)
    }
}
