use actix_identity::Identity;
use std::collections::HashMap;

use crate::core::service::ThemeService;
use crate::models::theme::{Theme, ThemeSetting};
use crate::schema::theme::dsl::*;
use crate::DbPool;
use actix_web::{web, Error, HttpResponse};
use diesel::RunQueryDsl;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ThemeData {
    pub event_title: String,
    pub event_description: String,
    pub closed_registration_message: String,
    pub is_registration_open: bool,
    pub tshirts_enabled: bool,
}
impl TryFrom<ThemeData> for Theme {
    type Error = anyhow::Error;
    fn try_from(value: ThemeData) -> Result<Self, Self::Error> {
        Theme::new(
            value.event_title,
            value.event_description,
            value.closed_registration_message,
            value.is_registration_open,
            value.tshirts_enabled,
        )
    }
}

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

pub async fn update_theme(
    _: Identity,
    data: web::Json<ThemeData>,
    theme_service: web::Data<dyn ThemeService>,
) -> anyhow::Result<HttpResponse, Error> {
    let theme_data = data.into_inner();
    theme_data
        .try_into()
        .and_then(|th| theme_service.update_theme(th)).map(|_| HttpResponse::Ok().into())
        .or_else(|_| Ok(HttpResponse::BadRequest().into()))
}
