use actix_web::{Error, error, HttpRequest, HttpResponse, Result, web};
use std::path::PathBuf;
use actix_files::NamedFile;
use tera::Context;
use rusqlite::Connection;
use serde::Deserialize;


pub async fn index(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let path: String = "index".to_string();
    result_template(tmpl, path).await
}

pub async fn template(tmpl: web::Data<tera::Tera>, page: web::Path<String>) -> Result<HttpResponse, Error> {
    let path = page.into_inner();
    result_template(tmpl, path).await
}

pub async fn result_template(tmpl: web::Data<tera::Tera>, name: String) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    ctx.insert("var", "Hello World");
    let rendered = tmpl.render(&(name + ".html"), &ctx).unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

pub async fn file(file: web::Path<String>, _req: HttpRequest) -> Result<NamedFile, Error> {
    let path: PathBuf = (String::from("./static/") + &file.into_inner()).parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[derive(Deserialize)]
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
    // country: String,
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
    println!("{:?}",form);
    if form.tshirt_toggle.len() >= 1 {
        
    }
    (form.email != form.repeat) || 
    (form.confirm.len() < 1) || 
    (form.starting_point == "null") || 
    (form.running_level == "null") ||
    (donation < 5)
}

pub async fn form(tmpl: web::Data<tera::Tera>, form: web::Form<Info>) -> Result<HttpResponse, Error> {
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
        panic!("data not good")
    }
    conn.execute(
        "INSERT INTO runners (firstname, lastname, team, email, starting_point, running_level, donation) values (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        &[&form.firstname, &form.lastname, &form.team, &form.email, &form.starting_point, &form.running_level, &form.donation],
    ).unwrap();

    result_template(tmpl, "submit".to_string()).await
}


pub async fn hey() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("Hey There!"))
}
pub async fn echo(req_body: String ) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body(req_body))
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;
    use crate::app_config::routes;
    use actix_web::dev::Service;
    use actix_web::{
        http::{StatusCode},
        test, App,
    };
    use actix_web::body::to_bytes;
    use httpmock::prelude::*;
    use isahc::prelude::*;

    #[actix_web::test]
    async fn index() {
        let app = test::init_service(App::new().configure(routes)).await;

        let req = test::TestRequest::get()
            .uri("/")
            .to_request();

        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);

        let mut index_as_string = String::new();
        let mut f = File::open("../../templates/index.html").expect("Unable to read file");
        f.read_to_string(&mut index_as_string).expect("Unable to read string");

        let body = to_bytes(resp.into_body()).await.unwrap();
        assert_eq!(body ,index_as_string)
    }
    #[actix_web::test]
    async fn echo() {
        let app = test::init_service(App::new().configure(routes)).await;
        let test_string = "I EXPECT THIS STRING TO BE RETURNED IMMEDIATELY";

        let req = test::TestRequest::post()
            .uri("/echo")
            .set_payload(test_string)
            .to_request();

        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);

        let body = to_bytes(resp.into_body()).await.unwrap();
        assert_eq!(body, test_string)
    }
 
    // #[actix_web::test]
    // async fn test_sending_registration_information() {
    //     let app = test::init_service(App::new().configure(routes)).await;

    //     let payload = r#"{"firstname=fname&lastname=lname&team=tname&email=mail&repeat=mail&starting_point=hamburg&running_level=rarely&confirm=on"}"#.as_bytes();

    //     let req = test::TestRequest::get()
    //         .uri("/")
    //         .set_payload(payload)
    //         .to_request();

    //     let resp = app.call(req).await.unwrap();

    //     assert_eq!(resp.status(), StatusCode::OK);
    // }
}
