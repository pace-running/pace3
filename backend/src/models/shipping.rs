use super::info::Info;
use crate::schema::shippings;
use diesel::prelude::*;
use std::convert::AsRef;
use strum_macros::AsRefStr;

#[derive(AsRefStr)]
#[allow(dead_code, clippy::upper_case_acronyms)]
enum DeliveryStatus {
    #[strum(serialize = "In Bearbeitung")]
    PROCESSED,
    #[strum(serialize = "Versendet")]
    SHIPPED,
    #[strum(serialize = "Zugestellt")]
    DELIVERED,
}

#[derive(Insertable)]
#[diesel(table_name = shippings)]
pub struct NewShipping<'a> {
    pub tshirt_model: &'a str,
    pub tshirt_size: &'a str,
    pub country: &'a str,
    pub firstname: &'a str,
    pub lastname: &'a str,
    pub street_name: &'a str,
    pub house_number: &'a str,
    pub address_extra: Option<&'a str>,
    pub postal_code: &'a str,
    pub city: &'a str,
    pub runner_id: i32,
    pub delivery_status: &'a str,
}

#[derive(Queryable)]
pub struct Shipping {
    pub id: i32,
    pub tshirt_model: String,
    pub tshirt_size: String,
    pub country: String,
    pub firstname: String,
    pub lastname: String,
    pub street_name: String,
    pub house_number: String,
    pub address_extra: Option<String>,
    pub postal_code: String,
    pub city: String,
    pub runner_id: i32,
    pub delivery_status: String,
}

impl<'a> From<(&'a Info, i32)> for NewShipping<'a> {
    fn from(info_with_runner_id: (&'a Info, i32)) -> Self {
        let (info, runner_id) = info_with_runner_id;
        NewShipping {
            tshirt_model: &info.shipping_info.tshirt_model,
            tshirt_size: &info.shipping_info.tshirt_size,
            country: &info.shipping_info.country,
            firstname: &info.shipping_info.address_firstname,
            lastname: &info.shipping_info.address_lastname,
            street_name: &info.shipping_info.street_name,
            house_number: &info.shipping_info.house_number,
            address_extra: Some(&info.shipping_info.address_extra),
            postal_code: &info.shipping_info.postal_code,
            city: &info.shipping_info.city,
            runner_id,
            delivery_status: DeliveryStatus::PROCESSED.as_ref(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builders::InfoBuilder;

    #[actix_web::test]
    async fn unit_create_new_shipping_test() {
        let runner_id = 1;
        let info = InfoBuilder::default().build();
        let shipping = NewShipping::from((&info, runner_id));
        assert_eq!(shipping.firstname, info.shipping_info.address_firstname);
        assert_eq!(shipping.lastname, info.shipping_info.address_lastname);
        assert_eq!(shipping.tshirt_model, info.shipping_info.tshirt_model);
        assert_eq!(shipping.tshirt_size, info.shipping_info.tshirt_size);
        assert_eq!(shipping.country, info.shipping_info.country);
        assert_eq!(shipping.street_name, info.shipping_info.street_name);
        assert_eq!(shipping.house_number, info.shipping_info.house_number);
        assert_eq!(shipping.postal_code, info.shipping_info.postal_code);
        assert_eq!(shipping.city, info.shipping_info.city);
        assert_eq!(shipping.runner_id, runner_id);
        assert_eq!(shipping.delivery_status, "In Bearbeitung");
    }
}
