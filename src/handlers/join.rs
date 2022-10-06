use crate::establish_connection;
use crate::insert_runner;
use crate::insert_shipping;
use crate::models::event;
use crate::models::info::Info;
use crate::models::runner;
use crate::models::runner::NewRunner;
use crate::models::shipping::NewShipping;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use actix_web::{web, Error, HttpResponse, Result};
use serde::Deserialize;
use serde::Serialize;
use tera::Context;

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct Response {
    success_message: Option<String>,
    error_message: Option<String>,
    status_code: u16,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct WithID<T> {
    runner_id: Option<String>,
    #[serde(flatten)]
    inner_response: T,
}

type ResponseWithId = WithID<Response>;

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

pub async fn register(form: Json<Info>) -> Result<HttpResponse, Error> {
    let info = form.into_inner();
    if has_bad_data(&info) {
        return Ok(HttpResponse::BadRequest().json(ResponseWithId {
            runner_id: None,
            inner_response: Response {
                success_message: None,
                error_message: Some("Bad data".to_string()),
                status_code: StatusCode::BAD_REQUEST.as_u16(),
            },
        }));
    }
    let conn = &mut establish_connection();
    let runner_start_number = runner::next_start_number(conn);
    // Write data into data base
    let new_runner = NewRunner::from((&info, runner_start_number));
    let returned_runner = insert_runner(conn, new_runner);
    if info.shipping_info.tshirt_toggle == "on" {
        let new_shipping = NewShipping::from((&info, returned_runner.id));
        insert_shipping(conn, new_shipping);
    }
    Ok(HttpResponse::Ok().json(ResponseWithId {
        runner_id: Some(returned_runner.id.to_string()),
        inner_response: Response {
            success_message: Some("Data received".to_string()),
            error_message: None,
            status_code: StatusCode::OK.as_u16(),
        },
    }))
}

#[cfg(test)]
mod tests {
    use crate::builders::InfoBuilder;
    use crate::handlers::join::{form_request, register, Response};
    use actix_web::body::{to_bytes, MessageBody};
    use actix_web::web::Bytes;
    use actix_web::{http::StatusCode, web};
    use tera::Tera;

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
        let response = register(input_data).await.unwrap();
        println!("{:?}", &response.body());
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
        let response = register(input_data).await.unwrap();
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
        let response = register(input_data).await.unwrap();
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
}
