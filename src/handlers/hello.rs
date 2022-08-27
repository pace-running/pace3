use actix_web::{Error,  HttpRequest, HttpResponse, Result, web};
use std::path::PathBuf;
use actix_files::NamedFile;
use tera::Context;


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

