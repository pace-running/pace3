use std::collections::HashMap;

use crate::dao::users::Dao;
use crate::models::theme::ThemeSetting;
use crate::schema::theme::dsl::*;
use crate::DatabaseConnection;
use actix_web::{web, Error, HttpResponse};
use diesel::RunQueryDsl;

pub async fn get_theme(dao: web::Data<Dao>) -> Result<HttpResponse, Error> {
    let mut conn = dao.pool().get().expect("Could not get connection");
    do_get_theme(&mut conn).await
}

async fn do_get_theme(conn: &mut DatabaseConnection) -> Result<HttpResponse, Error> {
    let theme_list = theme.load::<ThemeSetting>(conn).unwrap();

    let mut m: HashMap<String, String> = HashMap::new();
    for setting in theme_list.into_iter() {
        m.insert(setting.event_key, setting.event_value);
    }
    Ok(HttpResponse::Ok()
        .content_type("text/json")
        .body(serde_json::to_string(&m).unwrap()))
}
