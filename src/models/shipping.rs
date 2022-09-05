use crate::schema::shippings;
use diesel::prelude::*;

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
}
