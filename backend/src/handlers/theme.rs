use std::collections::HashMap;

use crate::models::theme::ThemeSetting;
use crate::schema::theme::{self, dsl::*};
use actix_web::{Error, HttpResponse};
use diesel::{PgConnection, RunQueryDsl};

pub async fn get_theme() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().into())
}

async fn do_get_theme(conn: &mut PgConnection) -> Result<HttpResponse, Error> {

    let theme_list = theme.load::<ThemeSetting>(conn).unwrap();
    
    let m = HashMap::from(theme_list);
    Ok(HttpResponse::Ok().content_type("text/json")
    .body(serde_json::to_string(&m).unwrap()))
}

#[cfg(test)]
mod tests {
    use crate::schema::theme::dsl::theme as schema_theme;
    use crate::schema::theme::dsl::*;
    use diesel::ExpressionMethods;

    #[actix_web::test]
    async fn integration_do_get_theme() {
        use diesel::{Connection, RunQueryDsl};
        println!("1 here!!!!!");

        use crate::{establish_connection, handlers::theme::do_get_theme, schema::theme};

        let conn = &mut establish_connection();
        conn.begin_test_transaction();

        diesel::update(schema_theme)
            .filter(event_key.eq("event_name"))
            .set(event_value.eq("test title"))
            .execute(conn);

        let response = do_get_theme(conn).await;



        println!("response: {:?}", response.unwrap().body());
    }
}
