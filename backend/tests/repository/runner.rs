use diesel::{prelude::*, QueryDsl, RunQueryDsl};
use pace::core::repository::RunnerRepository;
use pace::core::service::PaymentStatus;
use pace::models::donation::Donation;
use pace::models::runner::{
    NewRunner, PaymentReference, Runner, RunnerRegistrationData, RunnerUpdateData, ShippingData,
    ShippingUpdateData,
};
use pace::models::shipping::{DeliveryStatus, Shipping};
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
        RunnerRegistrationData::new(
            Option::from("Testi".to_string()),
            Option::from("McTest".to_string()),
            None,
            false,
            None,
            "hamburg".to_string(),
            "9000".to_string(),
            Donation::try_from(5).unwrap(),
            None,
        ),
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
        RunnerRegistrationData::new(
            Option::from("Testi".to_string()),
            Option::from("McTest".to_string()),
            None,
            false,
            None,
            "hamburg".to_string(),
            "9000".to_string(),
            Donation::try_from(5).unwrap(),
            Some(ShippingData {
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
        ),
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
    let shipping_data = ShippingData {
        t_shirt_model: "unisex".to_string(),
        t_shirt_size: "l".to_string(),
        country: "Deutschland".to_string(),
        firstname: "Testy".to_string(),
        lastname: "McTest".to_string(),
        street_name: "Teststraße".to_string(),
        house_number: "73".to_string(),
        address_extra: None,
        postal_code: "12345".to_string(),
        city: "Teststadt".to_string(),
    };
    let new_runner = NewRunner::new(
        RunnerRegistrationData::new(
            Option::from("Testy".to_string()),
            Option::from("McTest".to_string()),
            None,
            false,
            None,
            "hamburg".to_string(),
            "9000".to_string(),
            Donation::try_from(5).unwrap(),
            Some(shipping_data.clone()),
        ),
        StartNumber::new(73).unwrap(),
        PaymentReference::random(),
        "foo".to_string(),
        "0".to_string(),
    )
    .unwrap();
    let runner_in_db: Runner = diesel::insert_into(runners::table)
        .values(&new_runner)
        .get_result(&mut pool.get().expect("Unable to get connection"))
        .expect("Unable to insert runner data");

    let _shipping_in_db: Shipping = diesel::insert_into(shippings::table)
        .values(&(&shipping_data, &runner_in_db.id))
        .get_result(&mut pool.get().expect("Unable to get connection"))
        .expect("Unable to insert shipping data");

    let result = runner_repository.find_runner_by_id(runner_in_db.id);
    assert_eq!(result, Some(runner_in_db))
}

#[test]
fn find_runner_by_start_number_should_return_none_if_given_id_is_not_present() {
    let cli = testcontainers::clients::Cli::default();
    let database = TestDatabase::with_migrations(&cli);
    let pool = database.get_connection_pool();
    let runner_repository = PostgresRunnerRepository::new(pool);

    let result = runner_repository.find_runner_by_start_number(9000, None);
    assert_eq!(result, None)
}

#[test]
fn find_runner_by_start_number_should_return_runner_with_given_id_if_present_in_db() {
    let cli = testcontainers::clients::Cli::default();
    let database = TestDatabase::with_migrations(&cli);
    let pool = database.get_connection_pool();
    let runner_repository = PostgresRunnerRepository::new(pool.clone());
    let shipping_data = ShippingData {
        t_shirt_model: "unisex".to_string(),
        t_shirt_size: "l".to_string(),
        country: "Deutschland".to_string(),
        firstname: "Testy".to_string(),
        lastname: "McTest".to_string(),
        street_name: "Teststraße".to_string(),
        house_number: "73".to_string(),
        address_extra: None,
        postal_code: "12345".to_string(),
        city: "Teststadt".to_string(),
    };
    let new_runner = NewRunner::new(
        RunnerRegistrationData::new(
            Option::from("Testy".to_string()),
            Option::from("McTest".to_string()),
            None,
            false,
            None,
            "hamburg".to_string(),
            "9000".to_string(),
            Donation::try_from(5).unwrap(),
            Some(shipping_data.clone()),
        ),
        StartNumber::new(73).unwrap(),
        PaymentReference::random(),
        "foo".to_string(),
        "0".to_string(),
    )
    .unwrap();
    let runner_in_db: Runner = diesel::insert_into(runners::table)
        .values(&new_runner)
        .get_result(&mut pool.get().expect("Unable to get connection"))
        .expect("Unable to insert runner data");

    let _shipping_in_db: Shipping = diesel::insert_into(shippings::table)
        .values(&(&shipping_data, &runner_in_db.id))
        .get_result(&mut pool.get().expect("Unable to get connection"))
        .expect("Unable to insert shipping data");

    let result = runner_repository.find_runner_by_start_number(runner_in_db.start_number, None);
    assert_eq!(result, Some(runner_in_db))
}

#[test]
fn find_runner_by_start_number_should_return_none_if_given_id_is_presen_but_bsv_filter_does_not_match(
) {
    let cli = testcontainers::clients::Cli::default();
    let database = TestDatabase::with_migrations(&cli);
    let pool = database.get_connection_pool();
    let runner_repository = PostgresRunnerRepository::new(pool.clone());
    let shipping_data = ShippingData {
        t_shirt_model: "unisex".to_string(),
        t_shirt_size: "l".to_string(),
        country: "Deutschland".to_string(),
        firstname: "Testy".to_string(),
        lastname: "McTest".to_string(),
        street_name: "Teststraße".to_string(),
        house_number: "73".to_string(),
        address_extra: None,
        postal_code: "12345".to_string(),
        city: "Teststadt".to_string(),
    };
    let new_runner = NewRunner::new(
        RunnerRegistrationData::new(
            Option::from("Testy".to_string()),
            Option::from("McTest".to_string()),
            None,
            false,
            None,
            "hamburg".to_string(),
            "9000".to_string(),
            Donation::try_from(5).unwrap(),
            Some(shipping_data.clone()),
        ),
        StartNumber::new(73).unwrap(),
        PaymentReference::random(),
        "foo".to_string(),
        "0".to_string(),
    )
    .unwrap();
    let runner_in_db: Runner = diesel::insert_into(runners::table)
        .values(&new_runner)
        .get_result(&mut pool.get().expect("Unable to get connection"))
        .expect("Unable to insert runner data");

    let result = runner_repository.find_runner_by_start_number(
        runner_in_db.start_number,
        Some(!runner_in_db.bsv_participant),
    );
    assert_eq!(result, None)
}

#[test]
fn find_runners_by_name_containing_search_text_returns_expected_results() {
    let cli = testcontainers::clients::Cli::default();
    let database = TestDatabase::with_migrations(&cli);
    let pool = database.get_connection_pool();

    let mut connection = pool.get().unwrap();
    diesel::sql_query(
        "\
INSERT INTO runners (start_number, firstname, lastname, team, email,
                     starting_point, running_level, donation,
                     reason_for_payment, payment_status, verification_code,
                     payment_confirmation_mail_sent, tshirt_cost)
VALUES (42, 'Testy', 'McTest', 'Go Team!', NULL,
        'somewhere', 'super-duper', '10',
        'LGR-TESTY', false, 'befcf8a1-5acf-4590-ba96-9e95a3f82251',
        false, '10'),
       (73, NULL, 'McTest', 'Go Team!', NULL,
        'somewhere', 'super-duper', '10',
        'LGR-HANSI', false, '20e47480-3165-45fe-bdbf-64dee3c347bd',
        false, '10'),
       (111, 'Testy', NULL, 'Go Team!', NULL,
        'somewhere', 'super-duper', '10',
        'LGR-FLORA', false, 'c73b558b-3c1b-4602-a1e3-e98ae5d7b571',
        false, '10');",
    )
    .execute(&mut connection)
    .unwrap();

    let runner_repository = PostgresRunnerRepository::new(pool);
    let result = runner_repository.find_runners_by_name_containing("Testy", None);

    assert_eq!(2, result.len());
    assert_eq!("McTest", result.get(0).unwrap().lastname.as_ref().unwrap());
    assert!(result.get(1).unwrap().lastname.as_ref().is_none());
}

#[test]
fn find_runners_by_name_containing_search_text_returns_expected_results_filtered_by_bsv() {
    let cli = testcontainers::clients::Cli::default();
    let database = TestDatabase::with_migrations(&cli);
    let pool = database.get_connection_pool();

    let mut connection = pool.get().unwrap();
    diesel::sql_query(
        "\
INSERT INTO runners (start_number, firstname, lastname, team, email,
                     starting_point, running_level, donation,
                     reason_for_payment, payment_status, verification_code,
                     payment_confirmation_mail_sent, tshirt_cost, bsv_participant)
VALUES (42, 'Testy', 'McTest', 'Go Team!', NULL,
        'somewhere', 'super-duper', '10',
        'LGR-TESTY', false, 'befcf8a1-5acf-4590-ba96-9e95a3f82251',
        false, '10', false),
       (73, NULL, 'McTest', 'Go Team!', NULL,
        'somewhere', 'super-duper', '10',
        'LGR-HANSI', false, '20e47480-3165-45fe-bdbf-64dee3c347bd',
        false, '10', false),
       (111, 'Testy', NULL, 'Go Team!', NULL,
        'somewhere', 'super-duper', '10',
        'LGR-FLORA', false, 'c73b558b-3c1b-4602-a1e3-e98ae5d7b571',
        false, '10', true);",
    )
    .execute(&mut connection)
    .unwrap();

    let runner_repository = PostgresRunnerRepository::new(pool);
    let result = runner_repository.find_runners_by_name_containing("Testy", Some(false));

    assert_eq!(1, result.len());
    assert_eq!("McTest", result.get(0).unwrap().lastname.as_ref().unwrap());
}

#[test]
fn find_runners_by_name_containing_search_text_escapes_percentage_and_underscore() {
    let cli = testcontainers::clients::Cli::default();
    let database = TestDatabase::with_migrations(&cli);
    let pool = database.get_connection_pool();

    let mut connection = pool.get().unwrap();
    diesel::sql_query(
        "\
INSERT INTO runners (start_number, firstname, lastname, team, email,
                     starting_point, running_level, donation,
                     reason_for_payment, payment_status, verification_code,
                     payment_confirmation_mail_sent, tshirt_cost)
VALUES (42, '%Testy', 'McTest', 'Go Team!', NULL,
        'somewhere', 'super-duper', '10',
        'LGR-TESTY', false, 'befcf8a1-5acf-4590-ba96-9e95a3f82251',
        false, '10'),
       (73, 'Testy_', 'McTest', 'Go Team!', NULL,
        'somewhere', 'super-duper', '10',
        'LGR-HANSI', false, '20e47480-3165-45fe-bdbf-64dee3c347bd',
        false, '10'),
       (111, '%Testy_', 'McTest', 'Go Team!', NULL,
        'somewhere', 'super-duper', '10',
        'LGR-FLORA', false, 'c73b558b-3c1b-4602-a1e3-e98ae5d7b571',
        false, '10');",
    )
    .execute(&mut connection)
    .unwrap();

    let runner_repository = PostgresRunnerRepository::new(pool);
    let result = runner_repository.find_runners_by_name_containing("%Testy_", None);

    assert_eq!(1, result.len());
    assert_eq!(
        "%Testy_",
        result.get(0).unwrap().firstname.as_ref().unwrap()
    );
}

#[test]
fn find_runners_by_email_containing_search_text_returns_expected_results() {
    let cli = testcontainers::clients::Cli::default();
    let database = TestDatabase::with_migrations(&cli);
    let pool = database.get_connection_pool();

    let mut connection = pool.get().unwrap();
    diesel::sql_query(
        "\
INSERT INTO runners (start_number, firstname, lastname, team, email,
                     starting_point, running_level, donation,
                     reason_for_payment, payment_status, verification_code,
                     payment_confirmation_mail_sent, tshirt_cost)
VALUES (42, 'Testy', 'McTest', 'Go Team!', 'fancy@email.com',
        'somewhere', 'super-duper', '10',
        'LGR-TESTY', false, 'befcf8a1-5acf-4590-ba96-9e95a3f82251',
        false, '10'),
       (73, NULL, 'McTest', 'Go Team!', NULL,
        'somewhere', 'super-duper', '10',
        'LGR-HANSI', false, '20e47480-3165-45fe-bdbf-64dee3c347bd',
        false, '10'),
       (111, 'Testy', NULL, 'Go Team!', NULL,
        'somewhere', 'super-duper', '10',
        'LGR-FLORA', false, 'c73b558b-3c1b-4602-a1e3-e98ae5d7b571',
        false, '10');",
    )
    .execute(&mut connection)
    .unwrap();

    let runner_repository = PostgresRunnerRepository::new(pool);
    let result = runner_repository.find_runners_by_email_containing("ncy@em", None);

    assert_eq!(1, result.len());
    assert_eq!(
        "fancy@email.com",
        result.get(0).unwrap().email.as_ref().unwrap()
    );
}

#[test]
fn find_runners_by_email_containing_search_text_returns_expected_results_filtered_by_bsv() {
    let cli = testcontainers::clients::Cli::default();
    let database = TestDatabase::with_migrations(&cli);
    let pool = database.get_connection_pool();

    let mut connection = pool.get().unwrap();
    diesel::sql_query(
        "\
INSERT INTO runners (start_number, firstname, lastname, team, email,
                     starting_point, running_level, donation,
                     reason_for_payment, payment_status, verification_code,
                     payment_confirmation_mail_sent, tshirt_cost, bsv_participant)
VALUES (42, 'Testy', 'McTest', 'Go Team!', 'fancy@email.com',
        'somewhere', 'super-duper', '10',
        'LGR-TESTY', false, 'befcf8a1-5acf-4590-ba96-9e95a3f82251',
        false, '10', false),
       (73, NULL, 'McTest', 'Go Team!', 'fancy@example.com',
        'somewhere', 'super-duper', '10',
        'LGR-HANSI', false, '20e47480-3165-45fe-bdbf-64dee3c347bd',
        false, '10', true),
       (111, 'Testy', NULL, 'Go Team!', NULL,
        'somewhere', 'super-duper', '10',
        'LGR-FLORA', false, 'c73b558b-3c1b-4602-a1e3-e98ae5d7b571',
        false, '10', false);",
    )
    .execute(&mut connection)
    .unwrap();

    let runner_repository = PostgresRunnerRepository::new(pool);
    let result = runner_repository.find_runners_by_email_containing("ncy@e", Some(false));

    assert_eq!(1, result.len());
    assert_eq!(
        "fancy@email.com",
        result.get(0).unwrap().email.as_ref().unwrap()
    );
}

#[test]
fn find_runners_by_payment_reference_containing_search_text_returns_expected_results() {
    let cli = testcontainers::clients::Cli::default();
    let database = TestDatabase::with_migrations(&cli);
    let pool = database.get_connection_pool();

    let mut connection = pool.get().unwrap();
    diesel::sql_query(
        "\
INSERT INTO runners (start_number, firstname, lastname, team, email,
                     starting_point, running_level, donation,
                     reason_for_payment, payment_status, verification_code,
                     payment_confirmation_mail_sent, tshirt_cost)
VALUES (42, 'Testy', 'McTest', 'Go Team!', NULL,
        'somewhere', 'super-duper', '10',
        'LGR-TESTY', false, 'befcf8a1-5acf-4590-ba96-9e95a3f82251',
        false, '10'),
       (73, NULL, 'McTest', 'Go Team!', NULL,
        'somewhere', 'super-duper', '10',
        'LGR-HANSI', false, '20e47480-3165-45fe-bdbf-64dee3c347bd',
        false, '10'),
       (111, 'Testy', NULL, 'Go Team!', NULL,
        'somewhere', 'super-duper', '10',
        'LGR-FLORA', false, 'c73b558b-3c1b-4602-a1e3-e98ae5d7b571',
        false, '10');",
    )
    .execute(&mut connection)
    .unwrap();

    let runner_repository = PostgresRunnerRepository::new(pool);
    let result = runner_repository.find_runners_by_payment_reference_containing("hans", None);

    assert_eq!(1, result.len());
    assert_eq!("LGR-HANSI", result.get(0).unwrap().reason_for_payment);
}

#[test]
fn find_runners_by_payment_reference_containing_search_text_returns_expected_results_filtered_by_bsv(
) {
    let cli = testcontainers::clients::Cli::default();
    let database = TestDatabase::with_migrations(&cli);
    let pool = database.get_connection_pool();

    let mut connection = pool.get().unwrap();
    diesel::sql_query(
        "\
INSERT INTO runners (start_number, firstname, lastname, team, email,
                     starting_point, running_level, donation,
                     reason_for_payment, payment_status, verification_code,
                     payment_confirmation_mail_sent, tshirt_cost, bsv_participant)
VALUES (42, 'Testy', 'McTest', 'Go Team!', NULL,
        'somewhere', 'super-duper', '10',
        'LGR-TESTY', false, 'befcf8a1-5acf-4590-ba96-9e95a3f82251',
        false, '10', false),
       (73, NULL, 'McTest', 'Go Team!', NULL,
        'somewhere', 'super-duper', '10',
        'LGR-HANSI', false, '20e47480-3165-45fe-bdbf-64dee3c347bd',
        false, '10', true),
       (111, 'Testy', NULL, 'Go Team!', NULL,
        'somewhere', 'super-duper', '10',
        'LGR-FLORA', false, 'c73b558b-3c1b-4602-a1e3-e98ae5d7b571',
        false, '10', false);",
    )
    .execute(&mut connection)
    .unwrap();

    let runner_repository = PostgresRunnerRepository::new(pool);
    let result = runner_repository.find_runners_by_payment_reference_containing("lgr-", Some(true));

    assert_eq!(1, result.len());
    assert_eq!("LGR-HANSI", result.get(0).unwrap().reason_for_payment);
}

#[test]
fn find_shipping_by_runner_id_should_return_none_if_given_id_is_not_present() {
    let cli = testcontainers::clients::Cli::default();
    let database = TestDatabase::with_migrations(&cli);
    let pool = database.get_connection_pool();
    let runner_repository = PostgresRunnerRepository::new(pool);

    let result = runner_repository.find_shipping_by_runner_id(9000);
    assert_eq!(result, None)
}

#[test]
fn find_shipping_by_runner_id_should_return_shipping_with_given_runner_id_if_present_in_db() {
    let cli = testcontainers::clients::Cli::default();
    let database = TestDatabase::with_migrations(&cli);
    let pool = database.get_connection_pool();
    let runner_repository = PostgresRunnerRepository::new(pool.clone());
    let shipping_data = ShippingData {
        t_shirt_model: "unisex".to_string(),
        t_shirt_size: "l".to_string(),
        country: "Deutschland".to_string(),
        firstname: "Testy".to_string(),
        lastname: "McTest".to_string(),
        street_name: "Teststraße".to_string(),
        house_number: "73".to_string(),
        address_extra: None,
        postal_code: "12345".to_string(),
        city: "Teststadt".to_string(),
    };
    let new_runner = NewRunner::new(
        RunnerRegistrationData::new(
            Option::from("Testy".to_string()),
            Option::from("McTest".to_string()),
            None,
            false,
            None,
            "hamburg".to_string(),
            "9000".to_string(),
            Donation::try_from(5).unwrap(),
            Some(shipping_data.clone()),
        ),
        StartNumber::new(73).unwrap(),
        PaymentReference::random(),
        "foo".to_string(),
        "0".to_string(),
    )
    .unwrap();

    let runner_in_db: Runner = diesel::insert_into(runners::table)
        .values(&new_runner)
        .get_result(&mut pool.get().expect("Unable to get connection"))
        .expect("Error saving runner");

    let shipping_in_db: Shipping = diesel::insert_into(shippings::table)
        .values(&(&shipping_data, &runner_in_db.id))
        .get_result(&mut pool.get().expect("Unable to get connection"))
        .expect("Unable to insert shipping data");

    let result = runner_repository.find_shipping_by_runner_id(runner_in_db.id);
    assert_eq!(result, Some(shipping_in_db))
}

fn get_example_runner_update_data(shipping_data: Option<()>) -> RunnerUpdateData {
    RunnerUpdateData {
        start_number: StartNumber::new(42).unwrap(),
        firstname: Some("Testy".to_string()),
        lastname: Some("McTest".to_string()),
        team: Some("Go Team!".to_string()),
        bsv_participant: false,
        email: Some("Testy.McTest@example.com".to_string()),
        starting_point: "hamburg".to_string(),
        running_level: "9k".to_string(),
        donation: "10".to_string(),
        payment_reference: "LGR-TESTY".parse().unwrap(),
        payment_status: PaymentStatus::Pending,
        verification_code: "a3fea912-9c2a-40ee-bce4-c6fb3b3235c8".to_string(),
        shipping_data: shipping_data.map(|_| ShippingUpdateData {
            t_shirt_model: "unisex".to_string(),
            t_shirt_size: "l".to_string(),
            country: "Deutschland".to_string(),
            firstname: "Testy".to_string(),
            lastname: "McTest".to_string(),
            street_name: "Testy-McTest-Str.".to_string(),
            house_number: "42".to_string(),
            address_extra: None,
            postal_code: "54321".to_string(),
            city: "Metropolis".to_string(),
            delivery_status: DeliveryStatus::PROCESSED,
        }),
    }
}

#[test]
fn update_runner_should_fail_if_given_runner_id_is_not_present() {
    let cli = testcontainers::clients::Cli::default();
    let database = TestDatabase::with_migrations(&cli);
    let pool = database.get_connection_pool();

    let runner_repository = PostgresRunnerRepository::new(pool);
    let result: anyhow::Result<Runner> =
        runner_repository.update_runner(73i32, get_example_runner_update_data(None));

    assert!(result.is_err())
}

#[test]
fn update_runner_should_update_runner_data() {
    let cli = testcontainers::clients::Cli::default();
    let database = TestDatabase::with_migrations(&cli);
    let pool = database.get_connection_pool();
    let runner_update_data = get_example_runner_update_data(None);

    let runner_id = diesel::sql_query(
        "\
INSERT INTO runners (start_number, firstname, lastname, team, email,
                     starting_point, running_level, donation,
                     reason_for_payment, payment_status, verification_code,
                     payment_confirmation_mail_sent, tshirt_cost, bsv_participant)
VALUES (42, 'Testy', 'McTest', 'Go Team!', 'Testy.McTest@example.com',
        'other', '9k', '10',
        'LGR-TESTY', false, 'a3fea912-9c2a-40ee-bce4-c6fb3b3235c8',
        false, '0', false)
RETURNING *;",
    )
    .get_result::<Runner>(&mut (&pool).get().expect("Unable to get connection"))
    .unwrap()
    .id;

    let runner_repository = PostgresRunnerRepository::new(pool.clone());
    let result: anyhow::Result<Runner> =
        runner_repository.update_runner(runner_id, runner_update_data);

    assert!(result.is_ok());

    let runner = runners::dsl::runners
        .filter(runners::dsl::id.eq(&runner_id))
        .get_result::<Runner>(&mut (&pool).get().expect("Unable to get connection"))
        .unwrap();

    assert_eq!(runner.starting_point, "hamburg".to_string());
}

#[test]
fn update_runner_should_update_shipping_data() {
    let cli = testcontainers::clients::Cli::default();
    let database = TestDatabase::with_migrations(&cli);
    let pool = database.get_connection_pool();
    let runner_update_data = get_example_runner_update_data(Some(()));

    let runner_id = diesel::sql_query(
        "\
INSERT INTO runners (start_number, firstname, lastname, team, email,
                     starting_point, running_level, donation,
                     reason_for_payment, payment_status, verification_code,
                     payment_confirmation_mail_sent, tshirt_cost, bsv_participant)
VALUES (42, 'Testy', 'McTest', 'Go Team!', 'Testy.McTest@example.com',
        'hamburg', '9k', '10',
        'LGR-TESTY', false, 'a3fea912-9c2a-40ee-bce4-c6fb3b3235c8',
        false, '0', false)
RETURNING *;",
    )
    .get_result::<Runner>(&mut (&pool).get().expect("Unable to get connection"))
    .unwrap()
    .id;

    diesel::sql_query(
        "\
INSERT INTO shippings (tshirt_model, tshirt_size, country, firstname, lastname,
                       street_name, house_number, address_extra, postal_code,
                       city, runner_id)
VALUES ('unisex', 's', 'Deutschland', 'Testy', 'McTest',
        'Testy-McTest-Str.', '42', NULL, '54321',
        'Metropolis', $1);",
    )
    .bind::<diesel::sql_types::Integer, i32>(runner_id)
    .execute(&mut (&pool).get().expect("Unable to get connection"))
    .unwrap();

    let runner_repository = PostgresRunnerRepository::new(pool.clone());
    runner_repository
        .update_runner(runner_id, runner_update_data)
        .unwrap();

    let shipping = shippings::dsl::shippings
        .filter(shippings::runner_id.eq(&runner_id))
        .get_result::<Shipping>(&mut (&pool).get().expect("Unable to get connection"))
        .unwrap();

    assert_eq!(shipping.tshirt_size, "l");
}

#[test]
fn update_runner_should_add_shipping_data_if_prior_not_present() {
    let cli = testcontainers::clients::Cli::default();
    let database = TestDatabase::with_migrations(&cli);
    let pool = database.get_connection_pool();
    let runner_update_data = get_example_runner_update_data(Some(()));

    let runner_id = diesel::sql_query(
        "\
INSERT INTO runners (start_number, firstname, lastname, team, email,
                     starting_point, running_level, donation,
                     reason_for_payment, payment_status, verification_code,
                     payment_confirmation_mail_sent, tshirt_cost, bsv_participant)
VALUES (42, 'Testy', 'McTest', 'Go Team!', 'Testy.McTest@example.com',
        'hamburg', '9k', '10',
        'LGR-TESTY', false, 'a3fea912-9c2a-40ee-bce4-c6fb3b3235c8',
        false, '0', false)
RETURNING *;",
    )
    .get_result::<Runner>(&mut (&pool).get().expect("Unable to get connection"))
    .unwrap()
    .id;

    let runner_repository = PostgresRunnerRepository::new(pool.clone());
    let result: anyhow::Result<Runner> =
        runner_repository.update_runner(runner_id, runner_update_data);

    assert!(result.is_ok());

    let shipping_rows = diesel::sql_query("SELECT runner_id FROM shippings;")
        .execute(&mut (&pool).get().expect("Unable to get connection"))
        .unwrap();

    assert_eq!(shipping_rows, 1_usize);
}

#[test]
fn update_runner_should_remove_shipping_data_if_no_longer_present() {
    let cli = testcontainers::clients::Cli::default();
    let database = TestDatabase::with_migrations(&cli);
    let pool = database.get_connection_pool();
    let runner_update_data = get_example_runner_update_data(None);

    let runner_id = diesel::sql_query(
        "\
INSERT INTO runners (start_number, firstname, lastname, team, email,
                     starting_point, running_level, donation,
                     reason_for_payment, payment_status, verification_code,
                     payment_confirmation_mail_sent, tshirt_cost, bsv_participant)
VALUES (42, 'Testy', 'McTest', 'Go Team!', 'Testy.McTest@example.com',
        'hamburg', '9k', '10',
        'LGR-TESTY', false, 'a3fea912-9c2a-40ee-bce4-c6fb3b3235c8',
        false, '0', false)
RETURNING *;",
    )
    .get_result::<Runner>(&mut (&pool).get().expect("Unable to get connection"))
    .unwrap()
    .id;

    diesel::sql_query(
        "\
INSERT INTO shippings (tshirt_model, tshirt_size, country, firstname, lastname,
                       street_name, house_number, address_extra, postal_code,
                       city, runner_id)
VALUES ('unisex', 'l', 'Deutschland', 'Testy', 'McTest',
        'Testy-McTest-Str.', '42', NULL, '54321',
        'Metropolis', $1);",
    )
    .bind::<diesel::sql_types::Integer, i32>(runner_id)
    .execute(&mut (&pool).get().expect("Unable to get connection"))
    .unwrap();

    let runner_repository = PostgresRunnerRepository::new(pool.clone());
    let result: anyhow::Result<Runner> =
        runner_repository.update_runner(runner_id, runner_update_data);

    assert!(result.is_ok());

    let shipping_rows = diesel::sql_query("SELECT runner_id FROM shippings;")
        .execute(&mut (&pool).get().expect("Unable to get connection"))
        .unwrap();

    assert_eq!(shipping_rows, 0_usize);
}
