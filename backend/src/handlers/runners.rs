use actix_web::http::StatusCode;
use actix_web::web::Json;
use actix_web::{error, web, Error, HttpResponse, Result};
use serde::Deserialize;
use serde::Serialize;

use crate::core::service::RunnerService;
use crate::models::info::{Info, ShippingInfo};
use crate::models::runner::{RunnerRegistrationData, ShippingData};
use crate::services::email::send_registration_email;
use crate::{retrieve_shipping_by_runner_id, DbPool};

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

impl From<Info> for RunnerRegistrationData {
    fn from(value: Info) -> Self {
        let firstname = match value.runner_info.firstname.as_str() {
            "" => None,
            _ => Some(value.runner_info.firstname),
        };
        let lastname = match value.runner_info.lastname.as_str() {
            "" => None,
            _ => Some(value.runner_info.lastname),
        };
        let team = match value.runner_info.team.as_str() {
            "" => None,
            _ => Some(value.runner_info.team),
        };
        let email = match value.runner_info.email.as_str() {
            "" => None,
            _ => Some(value.runner_info.email),
        };

        RunnerRegistrationData {
            firstname,
            lastname,
            team,
            bsv_participant: value.runner_info.bsv_participant,
            email,
            starting_point: value.runner_info.starting_point,
            running_level: value.runner_info.running_level,
            donation: value.runner_info.donation,
            shipping_data: value.shipping_info.into(),
        }
    }
}

impl From<ShippingInfo> for Option<ShippingData> {
    fn from(value: ShippingInfo) -> Self {
        if value.tshirt_toggle != "on" {
            return None;
        }

        /* TODO: use this after verifying that it won't break the frontend
        let address_extra = match value.address_extra.as_str() {
            "" => None,
            _ => Some(value.address_extra),
        };
         */

        Some(ShippingData {
            t_shirt_model: value.tshirt_model,
            t_shirt_size: value.tshirt_size,
            country: value.country,
            firstname: value.address_firstname,
            lastname: value.address_lastname,
            street_name: value.street_name,
            house_number: value.house_number,
            address_extra: Some(value.address_extra),
            postal_code: value.postal_code,
            city: value.city,
        })
    }
}

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

pub async fn create_runner(
    form: Json<Info>,
    runner_service: web::Data<dyn RunnerService>,
) -> Result<HttpResponse, Error> {
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
    let runner_registration_data = RunnerRegistrationData::from(info);
    let returned_runner = runner_service
        .register_runner(runner_registration_data)
        .map_err(error::ErrorInternalServerError)?;

    let email_value = returned_runner.email.unwrap_or_default();
    let email_provided = Some(email_value.ne(""));
    if let Some(true) = email_provided {
        send_registration_email(
            returned_runner.id.to_string(),
            returned_runner.start_number.to_string(),
            email_value,
            returned_runner.donation.clone(),
            returned_runner.reason_for_payment.clone(),
            returned_runner.verification_code.clone(),
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
    db_pool: web::Data<DbPool>,
    runner_service: web::Data<dyn RunnerService>,
) -> Result<HttpResponse, Error> {
    let runner_id = request_data.into_inner();
    let connection = &mut db_pool.get().map_err(error::ErrorInternalServerError)?;
    let retrieved_runner = runner_service
        .find_runner_by_id(runner_id)
        .unwrap_or_else(|| panic!("Unable to find runner with id {runner_id}."));

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
