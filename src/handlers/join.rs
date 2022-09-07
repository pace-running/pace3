use crate::converters::create_new_runner;
use crate::converters::create_new_shipping;
use crate::establish_connection;
use crate::insert_runner;
use crate::insert_shipping;
use crate::models::event;
use crate::models::info::Info;
use actix_web::{web, Error, HttpResponse, Result};
use tera::Context;

pub async fn form_request(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    ctx.insert("event", &event::current_event());
    let rendered = tmpl.render("join.html", &ctx).unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

pub fn has_bad_data(form: &web::Form<Info>) -> bool {
    let donation: u16 = form
        .donation
        .trim()
        .parse::<u16>()
        .expect("Unable to parse donation value to number");
    if form.tshirt_toggle == "on"
        && (form.country.is_empty()
            || form.address_firstname.is_empty()
            || form.address_lastname.is_empty()
            || form.street_name.is_empty()
            || form.house_number.is_empty()
            || form.postal_code.is_empty()
            || form.city.is_empty()
            || form.tshirt_model == "null"
            || form.tshirt_size == "null")
    {
        println!("Not all required fields  for shipping are there");
        return true;
        // let postal_code: i32 = form.postal_code.trim().parse::<i32>().expect("Unable to parse postal code value to number");
    }
    (form.email != form.repeat)
        || (form.confirm != "on")
        || (form.starting_point == "null")
        || (form.running_level == "null")
        || (donation < 5)
}

pub async fn register(form: web::Form<Info>) -> Result<HttpResponse, Error> {
    if has_bad_data(&form) {
        return Ok(HttpResponse::BadRequest().body("Bad data"));
    }
    let conn = &mut establish_connection();
    // Write data into data base
    let new_runner = create_new_runner(&form);
    let returned_runner = insert_runner(conn, new_runner);
    if form.tshirt_toggle == "on" {
        let new_shipping = create_new_shipping(&form, returned_runner.id);
        insert_shipping(conn, new_shipping);
    }
    Ok(HttpResponse::Ok().body("Data received"))
}

#[cfg(test)]
mod tests {
    use crate::handlers::join::{form_request, register, Info};
    use actix_web::body::to_bytes;
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
    async fn form_page() {
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
    async fn minimal_submit_() {
        let participant = Info {
            firstname: "Hans".to_string(),
            lastname: "Meyer".to_string(),
            team: "FC St. Pauli".to_string(),
            email: "test@example.com".to_string(),
            repeat: "test@example.com".to_string(),
            starting_point: "somewhere".to_string(),
            running_level: "mediocre".to_string(),
            donation: "5".to_string(),
            tshirt_toggle: "".to_string(),
            tshirt_model: "".to_string(),
            tshirt_size: "".to_string(),
            country: "".to_string(),
            address_firstname: "".to_string(),
            address_lastname: "".to_string(),
            street_name: "".to_string(),
            house_number: "".to_string(),
            address_extra: "".to_string(),
            postal_code: "".to_string(),
            city: "".to_string(),
            confirm: "on".to_string(),
        };
        let input_data = web::Form(participant);
        let resp = register(input_data).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn submit_form_with_shipping() {
        let participant = Info {
            firstname: "Hans".to_string(),
            lastname: "Meyer".to_string(),
            team: "FC St. Pauli".to_string(),
            email: "test@example.com".to_string(),
            repeat: "test@example.com".to_string(),
            starting_point: "somewhere".to_string(),
            running_level: "mediocre".to_string(),
            donation: "5".to_string(),
            tshirt_toggle: "on".to_string(),
            tshirt_model: "unisex".to_string(),
            tshirt_size: "l".to_string(),
            country: "DE".to_string(),
            address_firstname: "Hans".to_string(),
            address_lastname: "Meyer".to_string(),
            street_name: "Street".to_string(),
            house_number: "1".to_string(),
            address_extra: "".to_string(),
            postal_code: "23455".to_string(),
            city: "Hamburg".to_string(),
            confirm: "on".to_string(),
        };
        let input_data = web::Form(participant);
        let resp = register(input_data).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn submit_wrong_form() {
        let participant = Info {
            firstname: "Hans".to_string(),
            lastname: "Meyer".to_string(),
            team: "FC St. Pauli".to_string(),
            email: "test@example.com".to_string(),
            repeat: "test@example.com".to_string(),
            starting_point: "somewhere".to_string(),
            running_level: "mediocre".to_string(),
            donation: "5".to_string(),
            tshirt_toggle: "on".to_string(),
            tshirt_model: "unisex".to_string(),
            tshirt_size: "l".to_string(),
            country: "DE".to_string(),
            address_firstname: "Hans".to_string(),
            address_lastname: "Meyer".to_string(),
            street_name: "Street".to_string(),
            house_number: "".to_string(),
            address_extra: "".to_string(),
            postal_code: "23455".to_string(),
            city: "Hamburg".to_string(),
            confirm: "on".to_string(),
        };
        let input_data = web::Form(participant);
        let resp = register(input_data).await.unwrap();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }
}
