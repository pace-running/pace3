use crate::models::info::{Info, RunnerInfo, ShippingInfo};

pub struct InfoBuilder {
    info: Info,
}

impl InfoBuilder {
    fn default_runner_info() -> RunnerInfo {
        RunnerInfo {
            firstname: "Hans".to_string(),
            lastname: "Meyer".to_string(),
            team: "FC St. Pauli".to_string(),
            email: "test@example.com".to_string(),
            repeat: "test@example.com".to_string(),
            starting_point: "other".to_string(),
            running_level: "sometimes".to_string(),
            donation: "5".to_string(),
            confirm: "on".to_string(),
        }
    }

    fn default_shipping_info() -> ShippingInfo {
        ShippingInfo {
            tshirt_toggle: "on".to_string(),
            tshirt_model: "unisex".to_string(),
            tshirt_size: "l".to_string(),
            country: "Deutschland".to_string(),
            address_firstname: "Hans".to_string(),
            address_lastname: "Meyer".to_string(),
            street_name: "Street".to_string(),
            house_number: "1".to_string(),
            address_extra: "".to_string(),
            postal_code: "23455".to_string(),
            city: "Hamburg".to_string(),
        }
    }

    pub fn default() -> InfoBuilder {
        Self {
            info: Info {
                runner_info: Self::default_runner_info(),
                shipping_info: Self::default_shipping_info(),
            },
        }
    }

    pub fn minimal_default() -> InfoBuilder {
        Self {
            info: Info {
                runner_info: Self::default_runner_info(),
                shipping_info: Default::default(),
            },
        }
    }

    pub fn with_firstname(&mut self, firstname: &str) -> &mut Self {
        self.info.runner_info.firstname = firstname.to_string();
        self
    }

    pub fn with_lastname(&mut self, lastname: &str) -> &mut Self {
        self.info.runner_info.lastname = lastname.to_string();
        self
    }

    pub fn with_team(&mut self, team: &str) -> &mut Self {
        self.info.runner_info.team = team.to_string();
        self
    }

    pub fn with_email(&mut self, email: &str) -> &mut Self {
        self.info.runner_info.email = email.to_string();
        self
    }

    pub fn with_repeat(&mut self, repeat: &str) -> &mut Self {
        self.info.runner_info.repeat = repeat.to_string();
        self
    }
    pub fn with_starting_point(&mut self, starting_point: &str) -> &mut Self {
        self.info.runner_info.starting_point = starting_point.to_string();
        self
    }
    pub fn with_running_level(&mut self, running_level: &str) -> &mut Self {
        self.info.runner_info.running_level = running_level.to_string();
        self
    }
    pub fn with_donation(&mut self, donation: &str) -> &mut Self {
        self.info.runner_info.donation = donation.to_string();
        self
    }
    pub fn with_tshirt_toggle(&mut self, tshirt_toggle: &str) -> &mut Self {
        self.info.shipping_info.tshirt_toggle = tshirt_toggle.to_string();
        self
    }
    pub fn with_tshirt_model(&mut self, tshirt_model: &str) -> &mut Self {
        self.info.shipping_info.tshirt_model = tshirt_model.to_string();
        self
    }
    pub fn with_tshirt_size(&mut self, tshirt_size: &str) -> &mut Self {
        self.info.shipping_info.tshirt_size = tshirt_size.to_string();
        self
    }
    pub fn with_country(&mut self, country: &str) -> &mut Self {
        self.info.shipping_info.country = country.to_string();
        self
    }
    pub fn with_address_firstname(&mut self, address_firstname: &str) -> &mut Self {
        self.info.shipping_info.address_firstname = address_firstname.to_string();
        self
    }
    pub fn with_address_lastname(&mut self, address_lastname: &str) -> &mut Self {
        self.info.shipping_info.address_lastname = address_lastname.to_string();
        self
    }
    pub fn with_street_name(&mut self, street_name: &str) -> &mut Self {
        self.info.shipping_info.street_name = street_name.to_string();
        self
    }
    pub fn with_house_number(&mut self, house_number: &str) -> &mut Self {
        self.info.shipping_info.house_number = house_number.to_string();
        self
    }
    pub fn with_address_extra(&mut self, address_extra: &str) -> &mut Self {
        self.info.shipping_info.address_extra = address_extra.to_string();
        self
    }
    pub fn with_postal_code(&mut self, postal_code: &str) -> &mut Self {
        self.info.shipping_info.postal_code = postal_code.to_string();
        self
    }
    pub fn with_city(&mut self, city: &str) -> &mut Self {
        self.info.shipping_info.city = city.to_string();
        self
    }
    pub fn with_confirm(&mut self, confirm: &str) -> &mut Self {
        self.info.runner_info.confirm = confirm.to_string();
        self
    }

    pub fn build(&self) -> Info {
        self.info.clone()
    }
}
