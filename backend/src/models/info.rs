use crate::validation::{Validate, ValidationError};
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct RunnerInfo {
    pub firstname: String,
    pub lastname: String,
    pub team: String,
    pub email: String,
    pub bsv_participant: bool,
    pub repeat: String,
    pub starting_point: String,
    pub running_level: String,
    pub donation: String,
    pub confirm: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct ShippingInfo {
    pub tshirt_toggle: String,
    pub tshirt_model: String,
    pub tshirt_size: String,
    pub country: String,
    pub address_firstname: String,
    pub address_lastname: String,
    pub street_name: String,
    pub house_number: String,
    pub address_extra: String,
    pub postal_code: String,
    pub city: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct Info {
    #[serde(flatten)]
    pub runner_info: RunnerInfo,
    #[serde(flatten)]
    pub shipping_info: ShippingInfo,
}

impl Validate for Info {
    fn validate(self) -> anyhow::Result<Info, ValidationError> {
        self.shipping_info.clone().validate()?;
        self.runner_info.clone().validate()?;

        Ok(self)
    }
}

impl Validate for RunnerInfo {
    fn validate(self) -> std::result::Result<Self, ValidationError> {
        let mut field_errors = HashMap::new();
        if self.email != self.repeat {
            field_errors.insert("email", vec!["email addresses did not match"]);
        }
        if self.confirm != "on" {
            field_errors.insert("confirm", vec!["must be selected"]);
        }
        if self.starting_point == "null" || self.starting_point.trim().is_empty() {
            // TODO Enhance validation by restricting possible values to a fixed set, e.g., via an enum.
            field_errors.insert("starting_point", vec!["must not be empty"]);
        }
        if self.running_level == "null" || self.running_level.trim().is_empty() {
            // TODO Enhance validation by restricting possible values to a fixed set, e.g., via an enum.
            field_errors.insert("running_level", vec!["must not be empty"]);
        }

        if field_errors.is_empty() {
            Ok(self)
        } else {
            Err(ValidationError::new("running_info", field_errors))
        }
    }
}

impl Validate for ShippingInfo {
    fn validate(self) -> std::result::Result<Self, ValidationError> {
        if self.tshirt_toggle == "off" || self.tshirt_toggle.is_empty() {
            return Ok(self);
        }

        let mut field_errors = HashMap::new();

        if self.tshirt_toggle == "on" {
            if self.country.trim().is_empty() {
                field_errors.insert("country", vec!["must not be empty"]);
            }
            if self.address_firstname.trim().is_empty() {
                field_errors.insert("address_firstname", vec!["must not be empty"]);
            }
            if self.address_lastname.trim().is_empty() {
                field_errors.insert("address_lastname", vec!["must not be empty"]);
            }
            if self.street_name.trim().is_empty() {
                field_errors.insert("street_name", vec!["must not be empty"]);
            }
            if self.house_number.trim().is_empty() {
                field_errors.insert("house_number", vec!["must not be empty"]);
            }
            if self.postal_code.trim().is_empty() {
                field_errors.insert("postal_code", vec!["must not be empty"]);
            }
            if self.city.trim().is_empty() {
                field_errors.insert("city", vec!["must not be empty"]);
            }
            if self.tshirt_model == "null" || self.tshirt_model.trim().is_empty() {
                // TODO Enhance validation by restricting possible values to a fixed set, e.g., via an enum.
                field_errors.insert("tshirt_model", vec!["must not be empty"]);
            }
            if self.tshirt_size == "null" || self.tshirt_size.trim().is_empty() {
                // TODO Enhance validation by restricting possible values to a fixed set, e.g., via an enum.
                field_errors.insert("tshirt_size", vec!["must not be empty"]);
            }
        } else {
            field_errors.insert("tshirt_toggle", vec!["value must be `on`, `off`, or empty"]);
        };

        if field_errors.is_empty() {
            Ok(self)
        } else {
            Err(ValidationError::new("shipping_info", field_errors))
        }
    }
}
