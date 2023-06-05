use crate::core::service::RunnerService;
use crate::models::info::Info;
use crate::models::runner::Runner;
use crate::models::shipping::Shipping;
use crate::validation::ValidateInto;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use actix_web::{web, Error, HttpResponse, Result};
use serde::Deserialize;
use serde::Serialize;

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

impl From<Runner> for RunnerDetails {
    fn from(runner: Runner) -> Self {
        Self {
            runner_id: runner.id.to_string(),
            start_number: runner.start_number.to_string(),
            donation: runner.donation,
            payment: runner.reason_for_payment,
            is_paid: runner.payment_status,
            tshirt_cost: runner.tshirt_cost,
        }
    }
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

impl From<Shipping> for ShippingDetails {
    fn from(shipping: Shipping) -> Self {
        Self {
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
        }
    }
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

pub async fn create_runner(
    form: Json<Info>,
    runner_service: web::Data<dyn RunnerService>,
) -> Result<HttpResponse, Error> {
    let runner_registration_data = form
        .validate_into()
        .map_err(crate::handlers::error::ClientError::ValidationError)?;
    let returned_runner_result = runner_service.register_runner(runner_registration_data);

    if returned_runner_result.is_err() {
        let e = returned_runner_result.unwrap_err();
        return if e
            .to_string()
            .contains("T-Shirt ordering is not enabled but shipping data was still provided!")
        {
            Err(crate::handlers::error::ClientError::BadRequestError.into())
        } else {
            Err(crate::handlers::error::InternalError::from(e).into())
        };
    }

    let returned_runner = returned_runner_result.unwrap();

    let has_provided_email_address = returned_runner.email.is_some();

    Ok(HttpResponse::Ok().json(ResponseWithBody {
        runner_id: Some(returned_runner.id.to_string()),
        start_number: Some(returned_runner.start_number),
        donation: Some(returned_runner.donation),
        tshirt_cost: Some(returned_runner.tshirt_cost),
        reason_for_payment: Some(returned_runner.reason_for_payment),
        verification_code: Some(returned_runner.verification_code),
        email_provided: Some(has_provided_email_address),
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
    runner_service: web::Data<dyn RunnerService>,
) -> Result<HttpResponse, Error> {
    let runner_id = request_data.into_inner();

    let runner_details = runner_service
        .find_runner_by_id_and_verification_code(runner_id, &token.verification_code)
        .map(RunnerDetails::from)
        .ok_or(crate::handlers::error::ClientError::AuthorizationError)?;

    let shipping_details = runner_service
        .find_shipping_by_runner_id(runner_id)
        .map(ShippingDetails::from);

    Ok(HttpResponse::Ok().json(RunnerResponse {
        runner_details: Some(runner_details),
        is_tshirt_booked: shipping_details.is_some(),
        shipping_details,
        inner_response: Response {
            success_message: Some("Data received".to_string()),
            error_message: None,
            status_code: StatusCode::OK.as_u16(),
        },
    }))
}
