use crate::models::{runner::Runner, shipping::Shipping};
use actix_web::{http::StatusCode, web, Error, HttpResponse};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::establish_connection;

use super::join::Response;

#[derive(Deserialize)]
pub struct RunnerID {
    runner_id: i32,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct RunnerResponse {
    runner_id: Option<String>,
    start_number: Option<String>,
    donation: Option<String>,
    payment: Option<String>,
    is_paid: Option<bool>,
    is_tshirt_booked: Option<bool>,
    tshirt_model: Option<String>,
    tshirt_size: Option<String>,
    country: Option<String>,
    address_firstname: Option<String>,
    address_lastname: Option<String>,
    street_name: Option<String>,
    house_number: Option<String>,
    address_extra: Option<String>,
    postal_code: Option<String>,
    city: Option<String>,
    delivery_status: Option<String>,

    #[serde(flatten)]
    inner_response: Response,
}

pub async fn get_runner(r_id: web::Path<RunnerID>) -> Result<HttpResponse, Error> {
    use crate::schema::runners::dsl::*;
    use crate::schema::shippings::dsl::*;
    let query_id = r_id.runner_id;
    let connection = &mut establish_connection();
    let queried_runner = runners
        .find(query_id)
        .get_result::<Runner>(connection)
        .expect(&format!("Could not retrieve runner with id {}", query_id));
    let shipping = shippings
        .filter(runner_id.eq(query_id))
        .get_result::<Shipping>(connection);
    let is_tshirt_booked = match shipping {
        Ok(_) => true,
        Err(_) => false,
    };
    // println!("Api call with runner_id: {}",queried_runner.id);
    if is_tshirt_booked {
        let unwr_shipping = shipping.unwrap();
        return Ok(HttpResponse::Ok().json(RunnerResponse {
            runner_id: Some(queried_runner.id.to_string()),
            start_number: Some(queried_runner.start_number.to_string()),
            donation: Some(queried_runner.donation),
            payment: Some(queried_runner.reason_for_payment),
            is_paid: Some(queried_runner.payment_status),
            is_tshirt_booked: Some(is_tshirt_booked),

            tshirt_model: Some(unwr_shipping.tshirt_model),
            tshirt_size: Some(unwr_shipping.tshirt_size),
            country: Some(unwr_shipping.country),
            address_firstname: Some(unwr_shipping.firstname),
            address_lastname: Some(unwr_shipping.lastname),
            street_name: Some(unwr_shipping.street_name),
            house_number: Some(unwr_shipping.house_number),
            address_extra: unwr_shipping.address_extra,
            postal_code: Some(unwr_shipping.postal_code),
            city: Some(unwr_shipping.city),
            delivery_status: Some(unwr_shipping.delivery_status),

            inner_response: Response {
                success_message: Some("Data received".to_string()),
                error_message: None,
                status_code: StatusCode::OK.as_u16(),
            },
        }));
    } else {
        return Ok(HttpResponse::Ok().json(RunnerResponse {
            runner_id: Some(queried_runner.id.to_string()),
            start_number: Some(queried_runner.start_number.to_string()),
            donation: Some(queried_runner.donation),
            payment: Some(queried_runner.reason_for_payment),
            is_paid: Some(queried_runner.payment_status),
            is_tshirt_booked: Some(is_tshirt_booked),

            tshirt_model: Some("".to_string()),
            tshirt_size: Some("".to_string()),
            country: Some("".to_string()),
            address_firstname: Some("".to_string()),
            address_lastname: Some("".to_string()),
            street_name: Some("".to_string()),
            house_number: Some("".to_string()),
            address_extra: Some("".to_string()),
            postal_code: Some("".to_string()),
            city: Some("".to_string()),
            delivery_status: Some("".to_string()),

            inner_response: Response {
                success_message: Some("Data received".to_string()),
                error_message: None,
                status_code: StatusCode::OK.as_u16(),
            },
        }));
    }
}
