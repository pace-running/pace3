use crate::schema::theme::dsl::theme as theme_table;
use crate::schema::theme::event_value;
use crate::{core::repository::ThemeRepository, models::theme::Theme};
use diesel::r2d2::ConnectionManager;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use r2d2::Pool;

pub struct PostgresThemeRepository {
    connection_pool: Pool<ConnectionManager<PgConnection>>,
}

impl PostgresThemeRepository {
    pub fn new(connection_pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { connection_pool }
    }
}

impl ThemeRepository for PostgresThemeRepository {
    fn update_theme(&self, theme: Theme) -> anyhow::Result<()> {
        let mut connection = self
            .connection_pool
            .get()
            .expect("Unable to get connection");
        connection.build_transaction().read_write().run(|conn| {
            diesel::update(theme_table.find("event_name"))
                .set(event_value.eq(theme.event_title()))
                .execute(conn)?;

            diesel::update(theme_table.find("event_description"))
                .set(event_value.eq(theme.event_description()))
                .execute(conn)?;

            diesel::update(theme_table.find("closed_registration_message"))
                .set(event_value.eq(theme.closed_registration_message()))
                .execute(conn)?;

            diesel::update(theme_table.find("is_registration_open"))
                .set(event_value.eq(theme.is_registration_open().to_string()))
                .execute(conn)?;

            diesel::update(theme_table.find("enable_tshirts"))
                .set(event_value.eq(theme.tshirts_enabled().to_string()))
                .execute(conn)?;

            Ok(())
        })
    }
}
