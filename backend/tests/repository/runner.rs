use diesel::{prelude::*, QueryDsl, RunQueryDsl};
use pace::core::repository::RunnerRepository;
use pace::models::runner::{
    NewRunner, PaymentReference, Runner, RunnerRegistrationData, ShippingData,
};
use pace::models::shipping::Shipping;
use pace::models::start_number::StartNumber;
use pace::repository::PostgresRunnerRepository;
use pace::schema::{runners, shippings};
use std::collections::HashSet;

pub use crate::helpers::TestDatabase;

#[test]
fn insert_new_runner_adds_runner_to_table() {
    let cli = testcontainers::clients::Cli::default();
    let database = TestDatabase::with_migrations(&cli);
    let pool = database.get_connection_pool();
    let runner_repository = PostgresRunnerRepository::new(pool.clone());

    let new_runner = NewRunner::new(
        RunnerRegistrationData {
            firstname: Option::from("Testi".to_string()),
            lastname: Option::from("McTest".to_string()),
            team: None,
            bsv_participant: false,
            email: None,
            starting_point: "hamburg".to_string(),
            running_level: "9000".to_string(),
            donation: "1,000,000 dollar".to_string(),
            shipping_data: None,
        },
        StartNumber::new(73).unwrap(),
        PaymentReference::random(),
        "foo".to_string(),
        "0".to_string(),
    )
    .unwrap();
    let result = runner_repository.insert_new_runner(new_runner).unwrap();

    let runner_in_db: Runner = runners::dsl::runners
        .find(result.id)
        .get_result::<Runner>(&mut pool.get().expect("Unable to get connection."))
        .unwrap();

    assert_eq!(runner_in_db, result)
}

#[test]
fn insert_new_runner_adds_shipping_data_to_table() {
    let cli = testcontainers::clients::Cli::default();
    let database = TestDatabase::with_migrations(&cli);
    let pool = database.get_connection_pool();
    let runner_repository = PostgresRunnerRepository::new(pool.clone());

    let new_runner = NewRunner::new(
        RunnerRegistrationData {
            firstname: Option::from("Testi".to_string()),
            lastname: Option::from("McTest".to_string()),
            team: None,
            bsv_participant: false,
            email: None,
            starting_point: "hamburg".to_string(),
            running_level: "9000".to_string(),
            donation: "1,000,000 dollar".to_string(),
            shipping_data: Some(ShippingData {
                t_shirt_model: "".to_string(),
                t_shirt_size: "".to_string(),
                country: "".to_string(),
                firstname: "foo".to_string(),
                lastname: "".to_string(),
                street_name: "".to_string(),
                house_number: "".to_string(),
                address_extra: None,
                postal_code: "".to_string(),
                city: "".to_string(),
            }),
        },
        StartNumber::new(73).unwrap(),
        PaymentReference::random(),
        "foo".to_string(),
        "0".to_string(),
    )
    .unwrap();
    let result = runner_repository.insert_new_runner(new_runner).unwrap();

    let shipping_data_in_db: Shipping = shippings::dsl::shippings
        .filter(shippings::runner_id.eq(&result.id))
        .get_result::<Shipping>(&mut pool.get().expect("Unable to get connection."))
        .unwrap();

    assert_eq!(shipping_data_in_db.firstname, "foo")
}

#[test]
fn get_next_start_number_does_not_return_deny_listed_values() {
    let cli = testcontainers::clients::Cli::default();
    let database = TestDatabase::with_migrations(&cli);
    let pool = database.get_connection_pool();
    let runner_repository = PostgresRunnerRepository::new(pool);

    let start_numbers: HashSet<i64> = (0..*StartNumber::DENYLIST.last().unwrap())
        .map(|_| runner_repository.get_next_start_number())
        .map(|v| v.into())
        .collect();

    assert!(start_numbers.is_disjoint(&HashSet::from(StartNumber::DENYLIST)));
}

#[actix_web::test]
async fn get_next_start_number_does_not_produce_duplicates() {
    let cli = testcontainers::clients::Cli::default();
    let database = TestDatabase::with_migrations(&cli);
    let pool = database.get_connection_pool();
    let runner_repository = PostgresRunnerRepository::new(pool);

    let start_numbers: HashSet<i64> = (0..*StartNumber::DENYLIST.last().unwrap())
        .map(|_| runner_repository.get_next_start_number())
        .map(|v| v.into())
        .collect();

    let dedup_start_numbers: HashSet<&i64> = HashSet::from_iter(start_numbers.iter());
    assert_eq!(dedup_start_numbers.len(), start_numbers.len())
}

#[test]
fn find_runner_by_id_should_return_none_if_given_id_is_not_present() {
    let cli = testcontainers::clients::Cli::default();
    let database = TestDatabase::with_migrations(&cli);
    let pool = database.get_connection_pool();
    let runner_repository = PostgresRunnerRepository::new(pool);

    let result = runner_repository.find_runner_by_id(9000);
    assert_eq!(result, None)
}

#[test]
fn find_runner_by_id_should_return_runner_with_given_id_if_present_in_db() {
    let cli = testcontainers::clients::Cli::default();
    let database = TestDatabase::with_migrations(&cli);
    let pool = database.get_connection_pool();
    let runner_repository = PostgresRunnerRepository::new(pool.clone());
    let new_runner = NewRunner::new(
        RunnerRegistrationData {
            firstname: Option::from("Testi".to_string()),
            lastname: Option::from("McTest".to_string()),
            team: None,
            bsv_participant: false,
            email: None,
            starting_point: "hamburg".to_string(),
            running_level: "9000".to_string(),
            donation: "1,000,000 dollar".to_string(),
            shipping_data: Some(ShippingData {
                t_shirt_model: "".to_string(),
                t_shirt_size: "".to_string(),
                country: "".to_string(),
                firstname: "foo".to_string(),
                lastname: "".to_string(),
                street_name: "".to_string(),
                house_number: "".to_string(),
                address_extra: None,
                postal_code: "".to_string(),
                city: "".to_string(),
            }),
        },
        StartNumber::new(73).unwrap(),
        PaymentReference::random(),
        "foo".to_string(),
        "0".to_string(),
    )
    .unwrap();
    let runner_in_db: Runner = diesel::insert_into(runners::table)
        .values(&new_runner)
        .get_result(&mut pool.get().expect("Unable to get connection."))
        .expect("Error saving runner");

    let result = runner_repository.find_runner_by_id(runner_in_db.id);
    assert_eq!(result, Some(runner_in_db))
}
