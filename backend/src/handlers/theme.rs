use std::collections::HashMap;

use crate::dao::users::Dao;
use crate::models::theme::ThemeSetting;
use crate::schema::theme::dsl::*;
use crate::DatabaseConnection;
use actix_web::{web, Error, HttpResponse};
use diesel::RunQueryDsl;

pub async fn get_theme(dao: web::Data<Dao>) -> Result<HttpResponse, Error> {
    let mut conn = dao.pool().get().expect("Could not get connection");
    let response = do_get_theme(&mut conn).await;
    response
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

#[cfg(test)]
mod tests {
    use crate::schema::theme::dsl::theme as schema_theme;
    use crate::schema::theme::dsl::*;
    use crate::{get_connection_pool, DatabaseConnection};
    use actix_web::body::MessageBody;
    use diesel::ExpressionMethods;
    use serde_json::*;

    #[actix_web::test]
    async fn integration_do_get_theme() {
        use diesel::{Connection, RunQueryDsl};

        use crate::{establish_connection, handlers::theme::do_get_theme, schema::theme};

        let mut conn: DatabaseConnection = get_connection_pool().unwrap().get().unwrap();
        let _ = conn.begin_test_transaction();

        let _ = diesel::update(schema_theme)
            .filter(event_key.eq("event_name"))
            .set(event_value.eq("test title"))
            .execute(&mut conn);

        let response = do_get_theme(&mut conn).await.unwrap();

        let expected_response: Value =
            serde_json::from_str("{\"event_name\":\"test title\"}").unwrap();

        let bytes = response.into_body().try_into_bytes().unwrap();
        let actual_response: Value = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(actual_response, expected_response);
    }
}
