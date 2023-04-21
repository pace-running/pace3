use crate::core::repository::{RunnerId, RunnerRepository};
use crate::models::payment::PaymentReference;
use crate::models::runner::{NewNewRunner, Runner};
use crate::models::start_number::StartNumber;
use crate::schema;
use crate::schema::runners::dsl::runners;
use diesel::r2d2::ConnectionManager;
use diesel::sql_types::BigInt;
use diesel::{OptionalExtension, PgConnection, QueryDsl, RunQueryDsl};
use r2d2::Pool;

use crate::models::shipping::DeliveryStatus;
use diesel::prelude::*;

pub struct PostgresRunnerRepository {
    connection_pool: Pool<ConnectionManager<PgConnection>>,
}

impl PostgresRunnerRepository {
    pub fn new(connection_pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { connection_pool }
    }
}

#[derive(diesel::QueryableByName)]
struct StartNumberCandidate {
    #[diesel(sql_type = BigInt)]
    #[diesel(column_name = start_number_candidate)]
    value: i64,
}

impl<'insert> diesel::Insertable<schema::runners::table> for &'insert NewNewRunner {
    type Values = <(
        diesel::dsl::Eq<schema::runners::start_number, i64>,
        Option<diesel::dsl::Eq<schema::runners::firstname, &'insert str>>,
        Option<diesel::dsl::Eq<schema::runners::lastname, &'insert str>>,
        Option<diesel::dsl::Eq<schema::runners::team, &'insert str>>,
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

impl RunnerRepository for PostgresRunnerRepository {
    fn insert_new_runner(&self, new_runner: NewNewRunner) -> anyhow::Result<Runner> {
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
                new_runner.shipping_data().as_ref().map_or_else(
                    || Ok(runner.clone()),
                    |sd| {
                        diesel::insert_into(schema::shippings::table)
                            .values((
                                schema::shippings::tshirt_model.eq(sd.t_shirt_model()),
                                schema::shippings::tshirt_size.eq(sd.t_shirt_size()),
                                schema::shippings::country.eq(sd.country()),
                                schema::shippings::firstname.eq(sd.firstname()),
                                schema::shippings::lastname.eq(sd.lastname()),
                                schema::shippings::street_name.eq(sd.street_name()),
                                schema::shippings::house_number.eq(sd.house_number()),
                                schema::shippings::address_extra.eq(sd.address_extra()),
                                schema::shippings::postal_code.eq(sd.postal_code()),
                                schema::shippings::city.eq(sd.city()),
                                schema::shippings::runner_id.eq(runner_id),
                                schema::shippings::delivery_status
                                    .eq(DeliveryStatus::PROCESSED.as_ref()),
                            ))
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
            .expect("Unable to get connection.");

        runners
            .find(id)
            .get_result::<Runner>(&mut connection)
            .optional()
            .expect("Failed to find runner")
    }
}
