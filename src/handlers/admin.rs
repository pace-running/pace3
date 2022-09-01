use actix_web::{Error,HttpResponse,web};

pub async fn login(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse,Error> {
    let rendered = tmpl.render("admin/login.html", &Default::default()).unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))

}

#[cfg(test)]
mod tests {
    use tera::Tera;
    use actix_web::http::StatusCode;
    use super::*;
    #[actix_web::test]
    async fn login_form() {
        let tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(_e) => { std::process::exit(1) }
        };
        let data = actix_web::web::Data::new(tera);
        let resp = login(data).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}