use actix_web::http::StatusCode;
use actix_web::web::Json;
use actix_web::{web, Error, HttpResponse, Result};
use serde::Deserialize;
use serde::Serialize;

use crate::insert_shipping;
use crate::models::info::Info;
use crate::models::runner;
use crate::models::runner::NewRunner;
use crate::models::shipping::NewShipping;
use crate::services::email::send_registration_email;
use crate::{establish_connection, retrieve_runner_by_id, retrieve_shipping_by_runner_id};
use crate::{insert_runner, is_eu_country};

#[derive(Deserialize)]
pub struct TokenRequestData {
    pub verification_code: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct RunnerResponse {
    #[serde(flatten)]
    pub runner_details: Option<RunnerDetails>,
    pub is_tshirt_booked: bool,
    #[serde(flatten)]
    pub shipping_details: Option<ShippingDetails>,
    #[serde(flatten)]
    pub inner_response: Response,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct RunnerDetails {
    pub runner_id: String,
    pub start_number: String,
    pub donation: String,
    pub payment: String,
    pub is_paid: bool,
    pub tshirt_cost: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct ShippingDetails {
    pub tshirt_model: String,
    pub tshirt_size: String,
    pub country: String,
    pub address_firstname: String,
    pub address_lastname: String,
    pub street_name: String,
    pub house_number: String,
    pub address_extra: Option<String>,
    pub postal_code: String,
    pub city: String,
    pub delivery_status: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct Response {
    pub success_message: Option<String>,
    pub error_message: Option<String>,
    pub status_code: u16,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct ResponseBody<T> {
    pub runner_id: Option<String>,
    pub start_number: Option<i64>,
    pub donation: Option<String>,
    pub tshirt_cost: Option<String>,
    pub reason_for_payment: Option<String>,
    pub verification_code: Option<String>,
    pub email_provided: Option<bool>,
    #[serde(flatten)]
    pub inner_response: T,
}

pub type ResponseWithBody = ResponseBody<Response>;

pub fn has_bad_data(form: &Info) -> bool {
    let donation: u16 = form
        .runner_info
        .donation
        .trim()
        .parse::<u16>()
        .expect("Unable to parse donation value to number");
    if form.shipping_info.tshirt_toggle == "on"
        && (form.shipping_info.country.is_empty()
            || form.shipping_info.address_firstname.is_empty()
            || form.shipping_info.address_lastname.is_empty()
            || form.shipping_info.street_name.is_empty()
            || form.shipping_info.house_number.is_empty()
            || form.shipping_info.postal_code.is_empty()
            || form.shipping_info.city.is_empty()
            || form.shipping_info.tshirt_model == "null"
            || form.shipping_info.tshirt_size == "null")
    {
        println!("Not all required fields  for shipping are there");
        return true;
        // let postal_code: i32 = form.postal_code.trim().parse::<i32>().expect("Unable to parse postal code value to number");
    }
    (form.runner_info.email != form.runner_info.repeat)
        || (form.runner_info.confirm != "on")
        || (form.runner_info.starting_point == "null")
        || (form.runner_info.running_level == "null")
        || (donation < 5)
}

pub async fn create_runner(form: Json<Info>) -> Result<HttpResponse, Error> {
    let info = form.into_inner();
    if has_bad_data(&info) {
        return Ok(HttpResponse::BadRequest().json(ResponseWithBody {
            runner_id: None,
            start_number: None,
            donation: None,
            tshirt_cost: None,
            reason_for_payment: None,
            verification_code: None,
            email_provided: None,
            inner_response: Response {
                success_message: None,
                error_message: Some("Bad data".to_string()),
                status_code: StatusCode::BAD_REQUEST.as_u16(),
            },
        }));
    }
    let conn = &mut establish_connection();
    let runner_start_number = runner::next_start_number(conn);
    let reason_for_payment = runner::create_random_payment();
    let verification_code = runner::create_verification_code();
    let tshirt_cost;
    if info.shipping_info.tshirt_toggle == "on" {
        if info.shipping_info.country == "Deutschland" {
            tshirt_cost = "15";
        } else if is_eu_country(&info.shipping_info.country) {
            tshirt_cost = "17";
        } else {
            tshirt_cost = "20";
        }
    } else {
        tshirt_cost = "0";
    }
    // Write data into data base
    let new_runner = NewRunner::from((
        &info,
        runner_start_number,
        reason_for_payment.as_str(),
        verification_code.as_str(),
        tshirt_cost,
    ));
    let returned_runner = insert_runner(conn, new_runner);
    if info.shipping_info.tshirt_toggle == "on" {
        let new_shipping = NewShipping::from((&info, returned_runner.id));
        insert_shipping(conn, new_shipping);
    }
    let email_value = returned_runner.email.unwrap();
    let email_provided = Some(email_value.ne(""));
    if let Some(true) = email_provided {
        send_registration_email(
            returned_runner.id.to_string(),
            returned_runner.start_number.to_string(),
            email_value,
            returned_runner.donation.clone(),
            returned_runner.reason_for_payment.clone(),
            verification_code.clone(),
            returned_runner.tshirt_cost.clone(),
        );
    }

    Ok(HttpResponse::Ok().json(ResponseWithBody {
        runner_id: Some(returned_runner.id.to_string()),
        start_number: Some(returned_runner.start_number),
        donation: Some(returned_runner.donation),
        tshirt_cost: Some(returned_runner.tshirt_cost),
        reason_for_payment: Some(returned_runner.reason_for_payment),
        verification_code: Some(returned_runner.verification_code),
        email_provided,
        inner_response: Response {
            success_message: Some("Data received".to_string()),
            error_message: None,
            status_code: StatusCode::OK.as_u16(),
        },
    }))
}

pub async fn get_runner(
    request_data: web::Path<i32>,
    token: web::Query<TokenRequestData>,
) -> Result<HttpResponse, Error> {
    let runner_id = request_data.into_inner();
    let connection = &mut establish_connection();
    let retrieved_runner = retrieve_runner_by_id(connection, runner_id);

    if retrieved_runner
        .verification_code
        .ne(&token.verification_code)
    {
        return Ok(HttpResponse::Forbidden().json(Response {
            success_message: None,
            error_message: Some("Code could not be verified".to_string()),
            status_code: StatusCode::FORBIDDEN.as_u16(),
        }));
    }

    let retrieved_shipping_result = retrieve_shipping_by_runner_id(connection, runner_id);

    let runner_details = Option::from(RunnerDetails {
        runner_id: retrieved_runner.id.to_string(),
        start_number: retrieved_runner.start_number.to_string(),
        donation: retrieved_runner.donation,
        payment: retrieved_runner.reason_for_payment,
        is_paid: retrieved_runner.payment_status,
        tshirt_cost: retrieved_runner.tshirt_cost,
    });

    let inner_response = Response {
        success_message: Some("Data received".to_string()),
        error_message: None,
        status_code: StatusCode::OK.as_u16(),
    };

    match retrieved_shipping_result {
        Ok(shipping) => Ok(HttpResponse::Ok().json(RunnerResponse {
            runner_details,
            is_tshirt_booked: true,
            shipping_details: Option::from(ShippingDetails {
                tshirt_model: shipping.tshirt_model,
                tshirt_size: shipping.tshirt_size,
                country: shipping.country,
                address_firstname: shipping.firstname,
                address_lastname: shipping.lastname,
                street_name: shipping.street_name,
                house_number: shipping.house_number,
                address_extra: shipping.address_extra,
                postal_code: shipping.postal_code,
                city: shipping.city,
                delivery_status: shipping.delivery_status,
            }),
            inner_response,
        })),
        Err(_) => Ok(HttpResponse::Ok().json(RunnerResponse {
            runner_details,
            is_tshirt_booked: false,
            shipping_details: None,
            inner_response,
        })),
    }
}
