use std::collections::HashMap;

use crate::models::theme::ThemeSetting;
use crate::schema::theme::dsl::*;
use crate::DbPool;
use actix_web::{web, Error, HttpResponse};
use diesel::RunQueryDsl;

pub async fn get_theme(db_pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let mut connection = db_pool.get().expect("Could not get connection");
    let theme_list = theme.load::<ThemeSetting>(&mut connection).unwrap();

    let mut m: HashMap<String, String> = HashMap::new();
    for setting in theme_list.into_iter() {
        m.insert(setting.event_key, setting.event_value);
    }
    Ok(HttpResponse::Ok()
        .content_type("text/json")
        .body(serde_json::to_string(&m).unwrap()))
}

pub async fn update_theme() -> Result<HttpResponse, Error> {
    // TODO: Add implementation
    Ok(HttpResponse::Ok().into())
}
