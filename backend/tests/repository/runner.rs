use diesel::{QueryDsl, RunQueryDsl};
use pace::core::repository::RunnerRepository;
use pace::models::runner::{NewRunner, Runner};
use pace::repository::PostgresRunnerRepository;
use pace::schema::runners;

use crate::helpers::TestDatabase;

#[test]
fn insert_runner_should_store_runner_in_db_if_no_constraints_are_violated() {
    let cli = testcontainers::clients::Cli::default();
    let database = TestDatabase::with_migrations(&cli);
    let pool = database.get_connection_pool();
    let runner_repository = PostgresRunnerRepository::new(pool.clone());

    let new_runner = NewRunner {
        start_number: 30,
        firstname: Option::from("Testi"),
        lastname: Option::from("McTest"),
        team: None,
        email: None,
        starting_point: "hamburg",
        running_level: "9000",
        donation: "1,000,000 dollar",
        reason_for_payment: "",
        payment_status: &false,
        verification_code: "",
        tshirt_cost: "",
    };
    let result = runner_repository.insert_runner(new_runner);

    let runner_in_db: Runner = runners::dsl::runners
        .find(result.id)
        .get_result::<Runner>(&mut pool.get().expect("Unable to get connection."))
        .unwrap();

    assert_eq!(runner_in_db, result)
}

#[test]
fn find_runner_by_id_should_return_none_if_given_id_is_not_present() {
    let cli = testcontainers::clients::Cli::default();
    let database = TestDatabase::with_migrations(&cli);
    let pool = database.get_connection_pool();
    let runner_repository = PostgresRunnerRepository::new(pool.clone());

    let result = runner_repository.find_runner_by_id(9000);
    assert_eq!(result, None)
}

#[test]
fn find_runner_by_id_should_return_runner_with_given_id_if_present_in_db() {
    let cli = testcontainers::clients::Cli::default();
    let database = TestDatabase::with_migrations(&cli);
    let pool = database.get_connection_pool();
    let runner_repository = PostgresRunnerRepository::new(pool.clone());
    let runner_in_db: Runner = diesel::insert_into(runners::table)
        .values(&NewRunner {
            start_number: 73,
            firstname: Option::from("Testi"),
            lastname: Option::from("McTest"),
            team: None,
            email: None,
            starting_point: "hamburg",
            running_level: "9000",
            donation: "1,000,000 dollar",
            reason_for_payment: "",
            payment_status: &false,
            verification_code: "",
            tshirt_cost: "",
        })
        .get_result(&mut pool.get().expect("Unable to get connection."))
        .expect("Error saving runner");

    let result = runner_repository.find_runner_by_id(runner_in_db.id);
    assert_eq!(result, Some(runner_in_db))
}
