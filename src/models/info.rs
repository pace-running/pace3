use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct RunnerInfo {
    pub firstname: String,
    pub lastname: String,
    pub team: String,
    pub email: String,
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
