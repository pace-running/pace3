use crate::core::repository::{RunnerId, RunnerRepository};
use crate::models::payment::PaymentReference;
use crate::models::runner::{NewRunner, Runner, ShippingData};
use crate::models::start_number::StartNumber;
use crate::schema;
use diesel::r2d2::ConnectionManager;
use diesel::sql_types::BigInt;
use diesel::{OptionalExtension, PgConnection, QueryDsl, RunQueryDsl};
use r2d2::Pool;

use crate::models::shipping::{DeliveryStatus, Shipping};
use diesel::prelude::*;

pub struct PostgresRunnerRepository {
    connection_pool: Pool<ConnectionManager<PgConnection>>,
}

impl PostgresRunnerRepository {
    pub fn new(connection_pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { connection_pool }
    }

    fn get_escape_char_for_search_text(search_text: &str) -> anyhow::Result<char> {
        if !search_text.contains('\\') {
            Ok('\\')
        } else if !search_text.contains('!') {
            Ok('!')
        } else if !search_text.contains('@') {
            Ok('@')
        } else if !search_text.contains('#') {
            Ok('#')
        } else if !search_text.contains('$') {
            Ok('$')
        } else if !search_text.contains('^') {
            Ok('^')
        } else {
            Err(anyhow::Error::msg(
                "Unable to get escape character for search text.",
            ))
        }
    }
}

#[derive(diesel::QueryableByName)]
struct StartNumberCandidate {
    #[diesel(sql_type = BigInt)]
    #[diesel(column_name = start_number_candidate)]
    value: i64,
}

impl<'insert> diesel::Insertable<schema::runners::table> for &'insert NewRunner {
    type Values = <(
        diesel::dsl::Eq<schema::runners::start_number, i64>,
        Option<diesel::dsl::Eq<schema::runners::firstname, &'insert str>>,
        Option<diesel::dsl::Eq<schema::runners::lastname, &'insert str>>,
        Option<diesel::dsl::Eq<schema::runners::team, &'insert str>>,
        diesel::dsl::Eq<schema::runners::bsv_participant, &'insert bool>,
        Option<diesel::dsl::Eq<schema::runners::email, &'insert str>>,
        diesel::dsl::Eq<schema::runners::starting_point, &'insert str>,
        diesel::dsl::Eq<schema::runners::running_level, &'insert str>,
        diesel::dsl::Eq<schema::runners::donation, &'insert str>,
        diesel::dsl::Eq<schema::runners::reason_for_payment, &'insert str>,
        diesel::dsl::Eq<schema::runners::verification_code, &'insert str>,
        diesel::dsl::Eq<schema::runners::tshirt_cost, &'insert str>,
    ) as diesel::Insertable<schema::runners::table>>::Values;

    fn values(self) -> Self::Values {
        (
            schema::runners::start_number.eq::<i64>((*self.start_number()).into()),
            self.firstname().map(|x| schema::runners::firstname.eq(x)),
            self.lastname().map(|x| schema::runners::lastname.eq(x)),
            self.team().map(|x| schema::runners::team.eq(x)),
            schema::runners::bsv_participant.eq(self.bsv_participant()),
            self.email().map(|x| schema::runners::email.eq(x)),
            schema::runners::starting_point.eq(self.starting_point()),
            schema::runners::running_level.eq(self.running_level()),
            schema::runners::donation.eq(self.donation()),
            schema::runners::reason_for_payment.eq::<&str>(self.payment_reference().as_str()),
            schema::runners::verification_code.eq(self.verification_code()),
            schema::runners::tshirt_cost.eq(self.t_shirt_cost()),
        )
            .values()
    }
}

impl<'insert> diesel::Insertable<schema::shippings::table> for &'insert (&ShippingData, &RunnerId) {
    type Values = <(
        diesel::dsl::Eq<schema::shippings::tshirt_model, &'insert str>,
        diesel::dsl::Eq<schema::shippings::tshirt_size, &'insert str>,
        diesel::dsl::Eq<schema::shippings::country, &'insert str>,
        diesel::dsl::Eq<schema::shippings::firstname, &'insert str>,
        diesel::dsl::Eq<schema::shippings::lastname, &'insert str>,
        diesel::dsl::Eq<schema::shippings::street_name, &'insert str>,
        diesel::dsl::Eq<schema::shippings::house_number, &'insert str>,
        Option<diesel::dsl::Eq<schema::shippings::address_extra, &'insert str>>,
        diesel::dsl::Eq<schema::shippings::postal_code, &'insert str>,
        diesel::dsl::Eq<schema::shippings::city, &'insert str>,
        diesel::dsl::Eq<schema::shippings::runner_id, &'insert i32>,
        diesel::dsl::Eq<schema::shippings::delivery_status, &'insert str>,
    ) as diesel::Insertable<schema::shippings::table>>::Values;

    fn values(self) -> Self::Values {
        (
            schema::shippings::tshirt_model.eq(self.0.t_shirt_model()),
            schema::shippings::tshirt_size.eq(self.0.t_shirt_size()),
            schema::shippings::country.eq(self.0.country()),
            schema::shippings::firstname.eq(self.0.firstname()),
            schema::shippings::lastname.eq(self.0.lastname()),
            schema::shippings::street_name.eq(self.0.street_name()),
            schema::shippings::house_number.eq(self.0.house_number()),
            self.0
                .address_extra()
                .map(|x| schema::shippings::address_extra.eq(x)),
            schema::shippings::postal_code.eq(self.0.postal_code()),
            schema::shippings::city.eq(self.0.city()),
            schema::shippings::runner_id.eq(self.1),
            schema::shippings::delivery_status.eq(DeliveryStatus::PROCESSED.as_ref()),
        )
            .values()
    }
}

impl RunnerRepository for PostgresRunnerRepository {
    fn insert_new_runner(&self, new_runner: NewRunner) -> anyhow::Result<Runner> {
        let mut connection = self
            .connection_pool
            .get()
            .expect("Unable to get connection.");

        diesel::insert_into(schema::runners::table)
            .values(&new_runner)
            .get_result(&mut connection)
            .map_err(anyhow::Error::from)
            .and_then(|runner: Runner| {
                let runner_id = runner.id;
                new_runner.shipping_data().map_or_else(
                    || Ok(runner.clone()),
                    |sd| {
                        diesel::insert_into(schema::shippings::table)
                            .values(&(sd, &runner_id))
                            .execute(&mut connection)
                            .map_err(anyhow::Error::from)
                            .map(|_| runner.clone())
                    },
                )
            })
    }

    fn get_next_start_number(&self) -> StartNumber {
        let mut connection = self
            .connection_pool
            .get()
            .expect("Unable to get connection.");

        loop {
            let candidate = diesel::sql_query(
                "SELECT nextval('runner_start_number_seq') AS start_number_candidate",
            )
            .get_result::<StartNumberCandidate>(&mut connection)
            .expect("Error getting start number");

            let start_number = StartNumber::try_from(candidate.value);

            if let Ok(result) = start_number {
                return result;
            }
        }
    }

    fn generate_unique_payment_reference(&self) -> PaymentReference {
        PaymentReference::random()
    }

    fn find_runner_by_id(&self, id: RunnerId) -> Option<Runner> {
        let mut connection = self
            .connection_pool
            .get()
            .expect("Unable to get connection");

        schema::runners::dsl::runners
            .find(id)
            .get_result::<Runner>(&mut connection)
            .optional()
            .expect("Failed to execute `find_runner_by_id` query")
    }

    fn find_runner_by_start_number(
        &self,
        start_number: i64,
        bsv_participant: Option<bool>,
    ) -> Option<Runner> {
        let mut connection = self
            .connection_pool
            .get()
            .expect("Unable to get connection");

        if let Some(is_bsv_participant) = bsv_participant {
            schema::runners::dsl::runners
                .filter(schema::runners::dsl::start_number.eq(start_number))
                .filter(schema::runners::dsl::bsv_participant.eq(is_bsv_participant))
                .get_result::<Runner>(&mut connection)
                .optional()
                .expect("Failed to execute `find_runner_by_start_number` query")
        } else {
            schema::runners::dsl::runners
                .filter(schema::runners::dsl::start_number.eq(start_number))
                .get_result::<Runner>(&mut connection)
                .optional()
                .expect("Failed to execute `find_runner_by_start_number` query")
        }
    }

    fn find_runners_by_name_containing(
        &self,
        search_text: &str,
        bsv_participant: Option<bool>,
    ) -> Vec<Runner> {
        let mut connection = self
            .connection_pool
            .get()
            .expect("Unable to get connection");

        let escape_char = Self::get_escape_char_for_search_text(search_text).unwrap();
        let escaped_percentage_char = format!("{escape_char}%");
        let escaped_underscore_char = format!("{escape_char}_");
        let escaped_search_text = search_text
            .replace('%', &escaped_percentage_char)
            .replace('_', &escaped_underscore_char);

        if let Some(is_bsv_participant) = bsv_participant {
            diesel::sql_query(
                "SELECT * FROM runners \
                WHERE bsv_participant = $1 \
                AND concat(firstname, ' ', lastname) ILIKE '%' || $2 || '%' ESCAPE $3;",
            )
            .bind::<diesel::sql_types::Bool, _>(is_bsv_participant)
            .bind::<diesel::sql_types::Text, _>(escaped_search_text)
            .bind::<diesel::sql_types::Text, _>(escape_char.to_string())
            .get_results(&mut connection)
            .expect("Failed to execute `find_runners_by_name_containing` query")
        } else {
            diesel::sql_query(
                "SELECT * FROM runners WHERE concat(firstname, ' ', lastname) ILIKE '%' || $1 || '%' ESCAPE $2;",
            )
                .bind::<diesel::sql_types::Text, _>(escaped_search_text)
                .bind::<diesel::sql_types::Text, _>(escape_char.to_string())
                .get_results(&mut connection)
                .expect("Failed to execute `find_runners_by_name_containing` query")
        }
    }

    fn find_runners_by_email_containing(
        &self,
        search_text: &str,
        bsv_participant: Option<bool>,
    ) -> Vec<Runner> {
        let mut connection = self
            .connection_pool
            .get()
            .expect("Unable to get connection");

        let escape_char = Self::get_escape_char_for_search_text(search_text).unwrap();
        let escaped_percentage_char = format!("{escape_char}%");
        let escaped_underscore_char = format!("{escape_char}_");
        let escaped_search_text = search_text
            .replace('%', &escaped_percentage_char)
            .replace('_', &escaped_underscore_char);

        if let Some(is_bsv_participant) = bsv_participant {
            diesel::sql_query(
                "SELECT * FROM runners \
                WHERE bsv_participant = $1 \
                AND email ILIKE '%' || $2 || '%' ESCAPE $3;",
            )
            .bind::<diesel::sql_types::Bool, _>(is_bsv_participant)
            .bind::<diesel::sql_types::Text, _>(escaped_search_text)
            .bind::<diesel::sql_types::Text, _>(escape_char.to_string())
            .get_results(&mut connection)
            .expect("Failed to execute `find_runners_by_name_containing` query")
        } else {
            diesel::sql_query("SELECT * FROM runners WHERE email ILIKE '%' || $1 || '%' ESCAPE $2;")
                .bind::<diesel::sql_types::Text, _>(escaped_search_text)
                .bind::<diesel::sql_types::Text, _>(escape_char.to_string())
                .get_results(&mut connection)
                .expect("Failed to execute `find_runners_by_email_containing` query")
        }
    }

    fn find_runners_by_payment_reference_containing(
        &self,
        search_text: &str,
        bsv_participant: Option<bool>,
    ) -> Vec<Runner> {
        let mut connection = self
            .connection_pool
            .get()
            .expect("Unable to get connection");

        let escape_char = Self::get_escape_char_for_search_text(search_text).unwrap();
        let escaped_percentage_char = format!("{escape_char}%");
        let escaped_underscore_char = format!("{escape_char}_");
        let escaped_search_text = search_text
            .replace('%', &escaped_percentage_char)
            .replace('_', &escaped_underscore_char);

        if let Some(is_bsv_participant) = bsv_participant {
            diesel::sql_query(
                "SELECT * FROM runners \
                WHERE bsv_participant = $1 \
                AND reason_for_payment ILIKE '%' || $2 || '%' ESCAPE $3;",
            )
            .bind::<diesel::sql_types::Bool, _>(is_bsv_participant)
            .bind::<diesel::sql_types::Text, _>(escaped_search_text)
            .bind::<diesel::sql_types::Text, _>(escape_char.to_string())
            .get_results(&mut connection)
            .expect("Failed to execute `find_runners_by_name_containing` query")
        } else {
            diesel::sql_query(
                "SELECT * FROM runners WHERE reason_for_payment ILIKE '%' || $1 || '%' ESCAPE $2;",
            )
            .bind::<diesel::sql_types::Text, _>(escaped_search_text)
            .bind::<diesel::sql_types::Text, _>(escape_char.to_string())
            .get_results(&mut connection)
            .expect("Failed to execute `find_runners_by_email_containing` query")
        }
    }

    fn find_shipping_by_runner_id(&self, runner_id: RunnerId) -> Option<Shipping> {
        let mut connection = self
            .connection_pool
            .get()
            .expect("Unable to get connection");

        schema::shippings::dsl::shippings
            .filter(schema::shippings::dsl::runner_id.eq(runner_id))
            .get_result::<Shipping>(&mut connection)
            .optional()
            .expect("Failed to find shipping")
    }
}
