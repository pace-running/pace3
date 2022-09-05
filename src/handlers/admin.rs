use actix_web::{web, Error, HttpResponse};

pub async fn login(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let rendered = tmpl
        .render("admin/login.html", &Default::default())
        .unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use tera::Tera;
    #[actix_web::test]
    async fn login_form() {
        let tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(_e) => std::process::exit(1),
        };
        let data = actix_web::web::Data::new(tera);
        let resp = login(data).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
