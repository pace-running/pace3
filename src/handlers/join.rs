use actix_web::{Error, HttpResponse, Result, web};
use tera::Context;
use rusqlite::{Connection,params};
use serde::Deserialize;
use serde::Serialize;
use crate::establish_connection;
use crate::models::{event,runner};

pub async fn form_request(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    ctx.insert("event", &event::current_event());
    let rendered = tmpl.render("join.html", &ctx).unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

#[derive(Deserialize)]
#[derive(Serialize)]
#[derive(Debug)]
pub struct Info {
    firstname: String,
    lastname: String,
    team: String,
    email: String,
    repeat: String,
    starting_point: String,
    running_level: String,
    donation: String,
    tshirt_toggle: String,
    tshirt_model: String,
    tshirt_size: String,
    country: String,
    address_firstname: String,
    address_lastname: String,
    street_name: String,
    house_number: String,
    address_extra: String,
    postal_code: String,
    city: String,
    confirm: String,
}

pub fn has_bad_data(form: &web::Form<Info>) -> bool {
    let donation: u16 = form.donation.trim().parse::<u16>().expect("Unable to parse donation value to number");
    if form.tshirt_toggle == "on" {
        if form.country == "" ||
        form.address_firstname == "" ||
        form.address_lastname == "" ||
        form.street_name == "" ||
        form.house_number == "" ||
        form.postal_code == "" ||
        form.city == "" ||
        form.tshirt_model == "null" ||
        form.tshirt_size == "null"
        {
            println!("Not all required fields  for shipping are there");
            return true
        };
        // let postal_code: i32 = form.postal_code.trim().parse::<i32>().expect("Unable to parse postal code value to number");
    }
    (form.email != form.repeat) ||
    (form.confirm != "on") ||
    (form.starting_point == "null") ||
    (form.running_level == "null") ||
    (donation < 5)
}

pub async fn newform (form: web::Form<Info>) -> Result<HttpResponse, Error> {
    use crate::schema::{shipping, runners};
    
    let conn = establish_connection();
    // Write data into data base
    Ok(HttpResponse::Ok().body("Data received"))
}

pub async fn form( form: web::Form<Info>) -> Result<HttpResponse, Error> {
    let conn = Connection::open("runners.db").unwrap();

    conn.execute(
        "create table if not exists runners (
             id integer primary key,
             firstname text not null,
             lastname text not null,
             team text not null,
             email text not null,
             starting_point text not null,
             running_level text not null,
             donation text not null
         )",
        [],
    ).unwrap();
    if has_bad_data(&form) {
        panic!("data not good");
    }
    conn.execute(
        "INSERT INTO runners (firstname, lastname, team, email, starting_point, running_level, donation) values (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        &[&form.firstname, &form.lastname, &form.team, &form.email, &form.starting_point, &form.running_level, &form.donation],
    ).unwrap();

    if form.tshirt_toggle == "on" {
        let runner_row_id = conn.last_insert_rowid();
        println!("{}",runner_row_id);
        conn.execute(
            "create table if not exists shipping (
                id integer primary key,
                tshirt_model text not null,
                tshirt_size text not null,
                country text not null,
                firstname text not null,
                lastname text not null,
                street_name text not null,
                house_number text not null,
                address_extra text not null,
                postal_code text not null,
                city text not null,
                runner_id integer not null
            )",
            [],
        ).unwrap();
        conn.execute(
            "INSERT INTO shipping (tshirt_model, tshirt_size, country, firstname, lastname, street_name, house_number, address_extra, postal_code, city, runner_id) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![&form.tshirt_model, &form.tshirt_size, &form.country, &form.address_firstname, &form.address_lastname, &form.street_name, &form.house_number, &form.address_extra, &form.postal_code, &form.city, &runner_row_id],
        ).unwrap();
    }
    //print_db(conn);
    Ok(HttpResponse::Ok().body("Data received"))
}

#[cfg(test)]
mod tests {
    use tera::{Tera};
    use crate::handlers::join::{form_request,form, Info};
    use actix_web::{http::{StatusCode}, http, web};
    use actix_web::body::to_bytes;
    use actix_web::web::Bytes;

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
            Err(_e) => { std::process::exit(1) }
        };
        let data = actix_web::web::Data::new(tera);
        let resp = form_request(data).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let body = to_bytes(resp.into_body()).await.unwrap();
        assert!(body.as_str().contains("<h1>Anmeldung</h1>"))
    }

    #[actix_web::test]
    async fn minimal_submit() {
        let participant = Info{
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
            confirm: "on".to_string()
        };
        let input_data = web::Form(participant);
        let resp = form(input_data).await.unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}
