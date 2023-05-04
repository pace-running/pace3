use diesel::prelude::*;
use rand::distributions::{Alphanumeric, DistString};
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

use crate::constants::VERIFICATION_CODE_LENGTH;

pub use crate::models::donation::Donation;
use crate::models::info::Info;
pub use crate::models::payment::PaymentReference;
use crate::models::start_number::StartNumber;
use crate::validation::{Validate, ValidateFrom, ValidationError};

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
    firstname: Option<String>,
    lastname: Option<String>,
    team: Option<String>,
    bsv_participant: bool,
    email: Option<String>,
    starting_point: String,
    running_level: String,
    donation: Donation,
    shipping_data: Option<ShippingData>,
}

impl RunnerRegistrationData {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        firstname: Option<String>,
        lastname: Option<String>,
        team: Option<String>,
        bsv_participant: bool,
        email: Option<String>,
        starting_point: String,
        running_level: String,
        donation: Donation,
        shipping_data: Option<ShippingData>,
    ) -> Self {
        Self {
            firstname,
            lastname,
            team,
            bsv_participant,
            email,
            starting_point,
            running_level,
            donation,
            shipping_data,
        }
    }

    pub fn firstname(&self) -> Option<&String> {
        self.firstname.as_ref()
    }
    pub fn lastname(&self) -> Option<&String> {
        self.lastname.as_ref()
    }
    pub fn team(&self) -> Option<&String> {
        self.team.as_ref()
    }
    pub fn bsv_participant(&self) -> bool {
        self.bsv_participant
    }
    pub fn email(&self) -> Option<&String> {
        self.email.as_ref()
    }
    pub fn starting_point(&self) -> &str {
        &self.starting_point
    }
    pub fn running_level(&self) -> &str {
        &self.running_level
    }
    pub fn donation(&self) -> &str {
        self.donation.as_ref()
    }
    pub fn shipping_data(&self) -> Option<&ShippingData> {
        self.shipping_data.as_ref()
    }
}

pub type PaymentStatus = bool;
pub type VerificationCode = String;
pub type TShirtCost = String;

pub struct NewRunner {
    start_number: StartNumber,
    firstname: Option<String>,
    lastname: Option<String>,
    team: Option<String>,
    bsv_participant: bool,
    email: Option<String>,
    starting_point: String,
    running_level: String,
    donation: Donation,
    payment_reference: PaymentReference,
    verification_code: String,
    t_shirt_cost: String,
    shipping_data: Option<ShippingData>,
}

impl NewRunner {
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
            bsv_participant: runner_registration_data.bsv_participant,
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
    pub fn bsv_participant(&self) -> &bool {
        &self.bsv_participant
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
        self.donation.as_ref()
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
    pub fn shipping_data(&self) -> Option<&ShippingData> {
        self.shipping_data.as_ref()
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
    pub bsv_participant: bool,
}

pub fn create_verification_code() -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), VERIFICATION_CODE_LENGTH)
}

#[cfg(test)]
mod tests {
    use crate::constants::VERIFICATION_CODE_LENGTH;

    use super::*;

    #[test]
    fn unit_create_verification_code() {
        let verification_code_1 = create_verification_code();
        let verification_code_2 = create_verification_code();

        assert_eq!(verification_code_1.len(), VERIFICATION_CODE_LENGTH);
        assert_eq!(verification_code_2.len(), VERIFICATION_CODE_LENGTH);
        assert_ne!(verification_code_1, verification_code_2)
    }
}

impl ValidateFrom<Info> for RunnerRegistrationData {
    fn validate_from(value: Info) -> std::result::Result<Self, ValidationError> {
        let info = value.validate()?;

        let donation = Donation::try_from(info.runner_info.donation).map_err(|e| {
            ValidationError::new(
                "runner_info",
                HashMap::from([("donation", vec![e.to_string()])]),
            )
        })?;

        let firstname = match info.runner_info.firstname.as_str() {
            "" => None,
            _ => Some(info.runner_info.firstname),
        };
        let lastname = match info.runner_info.lastname.as_str() {
            "" => None,
            _ => Some(info.runner_info.lastname),
        };
        let team = match info.runner_info.team.as_str() {
            "" => None,
            _ => Some(info.runner_info.team),
        };
        let email = match info.runner_info.email.as_str() {
            "" => None,
            _ => Some(info.runner_info.email),
        };

        Ok(RunnerRegistrationData::new(
            firstname,
            lastname,
            team,
            info.runner_info.bsv_participant,
            email,
            info.runner_info.starting_point,
            info.runner_info.running_level,
            donation,
            info.shipping_info.into(),
        ))
    }
}
