use actix_web::http::StatusCode;
use actix_web::web::Json;
use actix_web::{web, Error, HttpResponse, Result};
use serde::Deserialize;
use serde::Serialize;
use tera::Context;

use crate::insert_runner;
use crate::insert_shipping;
use crate::models::event;
use crate::models::info::Info;
use crate::models::runner;
use crate::models::runner::NewRunner;
use crate::models::shipping::NewShipping;
use crate::{establish_connection, retrieve_runner_by_id, retrieve_shipping_by_runner_id};

use super::email::send_registration_email;

#[derive(Deserialize)]
pub struct TokenRequestData {
    verification_code: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct RunnerResponse {
    #[serde(flatten)]
    runner_details: Option<RunnerDetails>,
    is_tshirt_booked: bool,
    #[serde(flatten)]
    shipping_details: Option<ShippingDetails>,
    #[serde(flatten)]
    inner_response: Response,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct RunnerDetails {
    runner_id: String,
    start_number: String,
    donation: String,
    payment: String,
    is_paid: bool,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct ShippingDetails {
    tshirt_model: String,
    tshirt_size: String,
    country: String,
    address_firstname: String,
    address_lastname: String,
    street_name: String,
    house_number: String,
    address_extra: Option<String>,
    postal_code: String,
    city: String,
    delivery_status: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct Response {
    success_message: Option<String>,
    error_message: Option<String>,
    status_code: u16,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct ResponseBody<T> {
    runner_id: Option<String>,
    start_number: Option<i64>,
    donation: Option<String>,
    reason_for_payment: Option<String>,
    verification_code: Option<String>,
    email_provided: Option<bool>,
    #[serde(flatten)]
    inner_response: T,
}

type ResponseWithBody = ResponseBody<Response>;

pub async fn form_request(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    ctx.insert("event", &event::current_event());
    let rendered = tmpl.render("join.html", &ctx).unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
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

pub async fn create_runner(form: Json<Info>) -> Result<HttpResponse, Error> {
    let info = form.into_inner();
    if has_bad_data(&info) {
        return Ok(HttpResponse::BadRequest().json(ResponseWithBody {
            runner_id: None,
            start_number: None,
            donation: None,
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
    // Write data into data base
    let new_runner = NewRunner::from((
        &info,
        runner_start_number,
        reason_for_payment.as_str(),
        verification_code.as_str(),
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
        );
    }

    Ok(HttpResponse::Ok().json(ResponseWithBody {
        runner_id: Some(returned_runner.id.to_string()),
        start_number: Some(returned_runner.start_number),
        donation: Some(returned_runner.donation),
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

#[cfg(test)]
mod tests {
    use actix_web::body::{to_bytes, MessageBody};
    use actix_web::web::{Bytes, Path, Query};
    use actix_web::{http::StatusCode, web};
    use tera::Tera;

    use crate::builders::InfoBuilder;
    use crate::handlers::runners::*;

    trait BodyTest {
        fn as_str(&self) -> &str;
    }

    impl BodyTest for Bytes {
        fn as_str(&self) -> &str {
            std::str::from_utf8(self).unwrap()
        }
    }

    #[actix_web::test]
    async fn unit_form_page() {
        let tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(_e) => std::process::exit(1),
        };
        let data = web::Data::new(tera);
        let resp = form_request(data).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let body = to_bytes(resp.into_body()).await.unwrap();
        assert!(body.as_str().contains("<h1>Anmeldung</h1>"))
    }

    #[actix_web::test]
    async fn integration_minimal_submit() {
        let participant = InfoBuilder::minimal_default().build();
        let input_data = web::Json(participant);
        let response = create_runner(input_data).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = response.into_body().try_into_bytes().unwrap();
        let actual_response: Response = serde_json::from_slice(&bytes).unwrap();
        let expected_response = Response {
            success_message: Some("Data received".to_string()),
            error_message: None,
            status_code: 200,
        };
        assert_eq!(actual_response, expected_response);
    }

    #[actix_web::test]
    async fn integration_submit_form_with_shipping() {
        let participant = InfoBuilder::default().build();
        let input_data = web::Json(participant);
        let response = create_runner(input_data).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = response.into_body().try_into_bytes().unwrap();
        let actual_response: Response = serde_json::from_slice(&bytes).unwrap();
        let expected_response = Response {
            success_message: Some("Data received".to_string()),
            error_message: None,
            status_code: 200,
        };
        assert_eq!(actual_response, expected_response);
    }

    #[actix_web::test]
    async fn integration_submit_wrong_form() {
        let participant = InfoBuilder::default().with_house_number("").build();
        let input_data = web::Json(participant);
        let response = create_runner(input_data).await.unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let bytes = response.into_body().try_into_bytes().unwrap();
        let actual_response: Response = serde_json::from_slice(&bytes).unwrap();
        let expected_response = Response {
            success_message: None,
            error_message: Some("Bad data".to_string()),
            status_code: 400,
        };
        assert_eq!(actual_response, expected_response);
    }

    #[actix_web::test]
    async fn integration_get_runner_by_id() {
        let participant = InfoBuilder::default().build();
        let input_data = web::Json(participant.clone());
        let post_response = create_runner(input_data).await.unwrap();
        assert_eq!(post_response.status(), StatusCode::OK);
        let bytes = post_response.into_body().try_into_bytes().unwrap();
        let created_runner: ResponseWithBody = serde_json::from_slice(&bytes).unwrap();
        let runner_id: i32 = created_runner.runner_id.clone().unwrap().parse().unwrap();
        let verification_code = TokenRequestData {
            verification_code: created_runner.verification_code.unwrap(),
        };
        let get_response = get_runner(Path::from(runner_id), Query(verification_code))
            .await
            .unwrap();
        assert_eq!(get_response.status(), StatusCode::OK);
        let bytes = get_response.into_body().try_into_bytes().unwrap();
        let returned_runner: RunnerResponse = serde_json::from_slice(&bytes).unwrap();
        let expected_runner = RunnerResponse {
            runner_details: Option::from(RunnerDetails {
                runner_id: created_runner.runner_id.unwrap(),
                start_number: created_runner.start_number.unwrap().to_string(),
                donation: created_runner.donation.unwrap(),
                payment: created_runner.reason_for_payment.unwrap(),
                is_paid: false,
            }),
            is_tshirt_booked: true,
            shipping_details: Option::from(ShippingDetails {
                tshirt_model: participant.shipping_info.tshirt_model.to_string(),
                tshirt_size: participant.shipping_info.tshirt_size.to_string(),
                country: participant.shipping_info.country.to_string(),
                address_firstname: participant.shipping_info.address_firstname.to_string(),
                address_lastname: participant.shipping_info.address_lastname.to_string(),
                street_name: participant.shipping_info.street_name.to_string(),
                house_number: participant.shipping_info.house_number.to_string(),
                address_extra: Some(participant.shipping_info.address_extra.to_string()),
                postal_code: participant.shipping_info.postal_code.to_string(),
                city: participant.shipping_info.city.to_string(),
                delivery_status: "In Bearbeitung".to_string(),
            }),
            inner_response: Response {
                success_message: Some("Data received".to_string()),
                error_message: None,
                status_code: StatusCode::OK.as_u16(),
            },
        };
        assert_eq!(returned_runner, expected_runner);
    }
}
