use std::collections::HashMap;

use crate::{models::theme::ThemeSetting, DbPool};
use crate::schema::theme::dsl::*;
use actix_web::web::Data;
use actix_web::{Error, HttpResponse};
use diesel::{PgConnection, RunQueryDsl};

pub async fn get_theme() -> Result<HttpResponse, Error> {
// TODO: wire up to do_get_theme
    Ok(HttpResponse::Ok().into())
}

async fn do_get_theme(conn: &mut PgConnection) -> Result<HttpResponse, Error> {
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
    use crate::models::theme::ThemeSetting;
    use crate::schema::theme::dsl::theme as schema_theme;
    use crate::schema::theme::dsl::*;
    use actix_web::body::MessageBody;
    use diesel::ExpressionMethods;
    use serde_json::*;

    #[actix_web::test]
    async fn integration_do_get_theme() {
        use diesel::{Connection, RunQueryDsl};

        use crate::{establish_connection, handlers::theme::do_get_theme, schema::theme};

        let conn = &mut establish_connection();
        let _ = conn.begin_test_transaction();

        let _ = diesel::update(schema_theme)
            .filter(event_key.eq("event_name"))
            .set(event_value.eq("test title"))
            .execute(conn);

        let response = do_get_theme(conn).await.unwrap();

        let expected_response: Value =
            serde_json::from_str("{\"event_name\":\"test title\"}").unwrap();

        let bytes = response.into_body().try_into_bytes().unwrap();
        let actual_response: Value = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(actual_response, expected_response);
    }
}
