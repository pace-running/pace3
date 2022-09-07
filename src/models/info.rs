use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Info {
    pub firstname: String,
    pub lastname: String,
    pub team: String,
    pub email: String,
    pub repeat: String,
    pub starting_point: String,
    pub running_level: String,
    pub donation: String,
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
    pub confirm: String,
}
