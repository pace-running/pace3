use diesel::RunQueryDsl;
use pace::models::users::{LoginData, PasswordChangeData};
use pace::{insert_rejected_transaction, models::rejected_transaction::NewRejectedTransaction};
use std::collections::HashMap;

pub mod helpers;
pub use helpers::{TestApp, TestDatabase};
use pace::models::runner::Runner;
use pace::models::shipping::Shipping;

#[test]
fn put_rejected_transaction_into_database() {
    let cli = testcontainers::clients::Cli::default();
    let database = TestDatabase::with_migrations(&cli);
    let conn = &mut database.get_connection();
    let new_transaction = NewRejectedTransaction {
        runner_ids: "2, 5",
        date_of_payment: "03.02.2023",
        reasons_for_payment: "LGR-POIUY, LGR-QWERT",
        payment_amount: "44",
        expected_amount: Some("45"),
        currency: "EUR",
        payer_name: "Testy McTest",
        iban: "DE87876876876",
    };
    let inserted_transaction = insert_rejected_transaction(conn, new_transaction);
    assert_eq!(inserted_transaction.iban, "DE87876876876");
}

#[actix_web::test]
async fn login_should_fail_if_login_data_is_empty() {
    let docker = testcontainers::clients::Cli::default();
    let test_app = TestApp::new(&docker).await;
    let address = test_app.get_address();
    let client = test_app.get_client();

    let login_data = LoginData {
        username: "".to_string(),
        password: "".to_string(),
    };

    let actual_response = client
        .post(format!("{address}/api/admin/login"))
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&login_data).unwrap())
        .send()
        .await
        .expect("Unable to send request.");

    assert_eq!(
        actual_response.status(),
        actix_web::http::StatusCode::FORBIDDEN
    );
}

#[actix_web::test]
async fn login_should_fail_if_password_is_wrong() {
    let docker = testcontainers::clients::Cli::default();
    let test_app = TestApp::new(&docker).await;
    let address = test_app.get_address();
    let client = test_app.get_client();

    let login_data = LoginData {
        username: "admin".to_string(),
        password: "not the correct password!".to_string(),
    };

    let actual_response = client
        .post(format!("{address}/api/admin/login"))
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&login_data).unwrap())
        .send()
        .await
        .expect("Unable to send request.");

    assert_eq!(
        actual_response.status(),
        actix_web::http::StatusCode::FORBIDDEN
    );
}

#[actix_web::test]
async fn change_password_should_fail_if_new_password_is_empty() {
    let docker = testcontainers::clients::Cli::default();
    let test_app = TestApp::new(&docker).await;
    let address = test_app.get_address();
    let client = test_app.get_client();

    let login_response = client
        .post(format!("{address}/api/admin/login"))
        .header("Content-Type", "application/json")
        .body(
            serde_json::to_string(&LoginData {
                username: "admin".to_string(),
                password: "xoh7Ongui4oo".to_string(), // talisman-ignore-line
            })
            .unwrap(),
        )
        .send()
        .await
        .expect("Unable to send request.");

    let cookie = login_response
        .cookies()
        .next()
        .expect("Unable to get cookie");

    let password_change_data = PasswordChangeData {
        old_password: "xoh7Ongui4oo".to_string(), // talisman-ignore-line
        new_password: "".to_string(),
    };

    let actual_response = client
        .put(format!("{address}/api/admin/change_password"))
        .header("Content-Type", "application/json")
        .header("Cookie", format!("{}={}", cookie.name(), cookie.value()))
        .body(serde_json::to_string(&password_change_data).unwrap())
        .send()
        .await
        .expect("Unable to send request.");

    assert_eq!(
        actual_response.status(),
        actix_web::http::StatusCode::FORBIDDEN
    );
}

#[actix_web::test]
async fn change_password_should_fail_if_old_password_is_empty() {
    let docker = testcontainers::clients::Cli::default();
    let test_app = TestApp::new(&docker).await;
    let address = test_app.get_address();
    let client = test_app.get_client();

    let login_response = client
        .post(format!("{address}/api/admin/login"))
        .header("Content-Type", "application/json")
        .body(
            serde_json::to_string(&LoginData {
                username: "admin".to_string(),
                password: "xoh7Ongui4oo".to_string(), // talisman-ignore-line
            })
            .unwrap(),
        )
        .send()
        .await
        .expect("Unable to send request.");

    let cookie = login_response
        .cookies()
        .next()
        .expect("Unable to get cookie");

    let password_change_data = PasswordChangeData {
        old_password: "".to_string(),
        new_password: "new_password".to_string(), // talisman-ignore-line
    };

    let actual_response = client
        .put(format!("{address}/api/admin/change_password"))
        .header("Content-Type", "application/json")
        .header("Cookie", format!("{}={}", cookie.name(), cookie.value()))
        .body(serde_json::to_string(&password_change_data).unwrap())
        .send()
        .await
        .expect("Unable to send request.");

    assert_eq!(
        actual_response.status(),
        actix_web::http::StatusCode::FORBIDDEN
    );
}

#[actix_web::test]
async fn change_password_should_fail_if_old_password_is_invalid() {
    let docker = testcontainers::clients::Cli::default();
    let test_app = TestApp::new(&docker).await;
    let address = test_app.get_address();
    let client = test_app.get_client();

    let login_response = client
        .post(format!("{address}/api/admin/login"))
        .header("Content-Type", "application/json")
        .body(
            serde_json::to_string(&LoginData {
                username: "admin".to_string(),
                password: "xoh7Ongui4oo".to_string(), // talisman-ignore-line
            })
            .unwrap(),
        )
        .send()
        .await
        .expect("Unable to send request.");

    let cookie = login_response
        .cookies()
        .next()
        .expect("Unable to get cookie");

    let password_change_data = PasswordChangeData {
        old_password: "not the correct password!".to_string(),
        new_password: "new_password".to_string(), // talisman-ignore-line
    };

    let actual_response = client
        .put(format!("{address}/api/admin/change_password"))
        .header("Content-Type", "application/json")
        .header("Cookie", format!("{}={}", cookie.name(), cookie.value()))
        .body(serde_json::to_string(&password_change_data).unwrap())
        .send()
        .await
        .expect("Unable to send request.");

    assert_eq!(
        actual_response.status(),
        actix_web::http::StatusCode::FORBIDDEN
    );
}

#[actix_web::test]
async fn change_password_should_be_successful_if_new_and_old_passwords_are_valid() {
    let docker = testcontainers::clients::Cli::default();
    let test_app = TestApp::new(&docker).await;
    let address = test_app.get_address();
    let client = test_app.get_client();

    let login_response = client
        .post(format!("{address}/api/admin/login"))
        .header("Content-Type", "application/json")
        .body(
            serde_json::to_string(&LoginData {
                username: "admin".to_string(),
                password: "xoh7Ongui4oo".to_string(), // talisman-ignore-line
            })
            .unwrap(),
        )
        .send()
        .await
        .expect("Unable to send request");

    let cookie = login_response
        .cookies()
        .next()
        .expect("Unable to get cookie");

    let password_change_data = PasswordChangeData {
        old_password: "xoh7Ongui4oo".to_string(), // talisman-ignore-line
        new_password: "new_password".to_string(), // talisman-ignore-line
    };

    let change_password_response = client
        .put(format!("{address}/api/admin/change_password"))
        .header("Content-Type", "application/json")
        .header("Cookie", format!("{}={}", cookie.name(), cookie.value()))
        .body(serde_json::to_string(&password_change_data).unwrap())
        .send()
        .await
        .expect("Unable to send request");

    assert_eq!(
        change_password_response.status(),
        actix_web::http::StatusCode::OK
    );

    let logout_response = client
        .post(format!("{address}/api/admin/logout"))
        .header("Content-Type", "application/json")
        .header("Cookie", format!("{}={}", cookie.name(), cookie.value()))
        .send()
        .await
        .expect("Unable to send request");

    assert_eq!(
        logout_response.status(),
        actix_web::http::StatusCode::NO_CONTENT
    );

    let failed_login_response = client
        .post(format!("{address}/api/admin/login"))
        .header("Content-Type", "application/json")
        .body(
            serde_json::to_string(&LoginData {
                username: "admin".to_string(),
                password: "xoh7Ongui4oo".to_string(), // talisman-ignore-line
            })
            .unwrap(),
        )
        .send()
        .await
        .expect("Unable to send request");

    assert_eq!(
        failed_login_response.status(),
        actix_web::http::StatusCode::FORBIDDEN
    );

    let new_login_response = client
        .post(format!("{address}/api/admin/login"))
        .header("Content-Type", "application/json")
        .body(
            serde_json::to_string(&LoginData {
                username: "admin".to_string(),
                password: "new_password".to_string(), // talisman-ignore-line
            })
            .unwrap(),
        )
        .send()
        .await
        .expect("Unable to send request");

    assert_eq!(new_login_response.status(), actix_web::http::StatusCode::OK);
}

#[actix_web::test]
async fn delete_rejected_transactions_should_fail_if_user_is_unauthorized() {
    let docker = testcontainers::clients::Cli::default();
    let test_app = TestApp::new(&docker).await;

    let result = test_app
        .delete_rejected_transactions("[1, 2]".to_string(), None)
        .await;

    assert_eq!(result.status(), actix_web::http::StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn delete_rejected_transactions_should_be_successful_if_authenticated() {
    let docker = testcontainers::clients::Cli::default();
    let test_app = TestApp::new(&docker).await;

    let mut connection = test_app.get_connection();

    let _ = diesel::sql_query(
        r#"
    INSERT INTO rejected_transactions (runner_ids, date_of_payment,
                                       reasons_for_payment, payment_amount,
                                       expected_amount, currency, payer_name, iban)
    VALUES ('73', '2023-01-01', 'LGR-RANDO', '1', '20', 'EUR', 'Peter',
            'DE20 1342 6474 521 45'),
           ('42', '2023-01-01', 'LGR-EMPTY', '2', '10', 'EUR', 'Petra',
            'DE20 2342 5474 523 11');"#,
    )
    .execute(&mut connection)
    .unwrap();

    let cookie = test_app.get_admin_cookie().await;

    let result = test_app
        .delete_rejected_transactions("[1, 2]".to_string(), Some(cookie))
        .await;

    assert_eq!(result.status(), actix_web::http::StatusCode::OK);

    let values = result.json::<HashMap<String, usize>>().await.unwrap();
    assert_eq!(&2, values.get("deletedRejectedTransactions").unwrap())
}

#[actix_web::test]
async fn get_runners_should_fail_if_user_is_unauthorized() {
    let docker = testcontainers::clients::Cli::default();
    let test_app = TestApp::new(&docker).await;

    let result = test_app.get_runners(1, "name", "foo", false, None).await;

    assert_eq!(result.status(), actix_web::http::StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn get_runners_should_return_runners_with_matching_start_number() {
    let docker = testcontainers::clients::Cli::default();
    let test_app = TestApp::new(&docker).await;

    let mut connection = test_app.get_connection();
    diesel::sql_query(
        "\
INSERT INTO runners (start_number, firstname, lastname, team, email,
                     starting_point, running_level, donation,
                     reason_for_payment, payment_status, verification_code,
                     payment_confirmation_mail_sent, tshirt_cost, bsv_participant)
VALUES (42, 'Testy', 'McTest', 'Team Testy', 'some.email@example.com',
        'somewhere', 'super-duper', '10',
        'LGR-TESTY', false, 'befcf8a1-5acf-4590-ba96-9e95a3f82251',
        false, '10', false),
       (73, 'Hansi', 'McTest', 'Go Team!', 'hello@example.com',
        'somewhere', 'super-duper', '10',
        'LGR-HANSI', false, '20e47480-3165-45fe-bdbf-64dee3c347bd',
        false, '10', true),
       (111, 'Flora', 'McTest', 'Go Team!', NULL,
        'somewhere', 'super-duper', '10',
        'LGR-FLORA', false, 'c73b558b-3c1b-4602-a1e3-e98ae5d7b571',
        false, '10', true);",
    )
    .execute(&mut connection)
    .unwrap();

    let result = test_app
        .get_runners(
            1,
            "start_number",
            "42",
            false,
            Some(test_app.get_admin_cookie().await),
        )
        .await;

    assert_eq!(result.status(), actix_web::http::StatusCode::OK);
    assert_eq!(
        result
            .json::<serde_json::Value>()
            .await
            .unwrap()
            .get("runner_list")
            .unwrap()
            .get(0)
            .unwrap()
            .get("email")
            .unwrap()
            .as_str()
            .unwrap(),
        "some.email@example.com"
    );
}

#[actix_web::test]
async fn get_runners_should_return_runners_with_matching_start_number_and_bsv_filter() {
    let docker = testcontainers::clients::Cli::default();
    let test_app = TestApp::new(&docker).await;

    let mut connection = test_app.get_connection();
    diesel::sql_query(
        "\
INSERT INTO runners (start_number, firstname, lastname, team, email,
                     starting_point, running_level, donation,
                     reason_for_payment, payment_status, verification_code,
                     payment_confirmation_mail_sent, tshirt_cost, bsv_participant)
VALUES (42, 'Testy', 'McTest', 'Team Testy', 'some.email@example.com',
        'somewhere', 'super-duper', '10',
        'LGR-TESTY', false, 'befcf8a1-5acf-4590-ba96-9e95a3f82251',
        false, '10', false),
       (73, 'Hansi', 'McTest', 'Go Team!', 'hello@example.com',
        'somewhere', 'super-duper', '10',
        'LGR-HANSI', false, '20e47480-3165-45fe-bdbf-64dee3c347bd',
        false, '10', true),
       (111, 'Flora', 'McTest', 'Go Team!', NULL,
        'somewhere', 'super-duper', '10',
        'LGR-FLORA', false, 'c73b558b-3c1b-4602-a1e3-e98ae5d7b571',
        false, '10', true);",
    )
    .execute(&mut connection)
    .unwrap();

    let result = test_app
        .get_runners(
            1,
            "start_number",
            "42",
            true,
            Some(test_app.get_admin_cookie().await),
        )
        .await;

    assert_eq!(result.status(), actix_web::http::StatusCode::OK);
    assert_eq!(
        result
            .json::<serde_json::Value>()
            .await
            .unwrap()
            .get("runner_list")
            .unwrap()
            .as_array()
            .unwrap(),
        &Vec::<serde_json::Value>::new()
    );
}

#[actix_web::test]
async fn get_runners_should_return_runners_with_matching_names() {
    let docker = testcontainers::clients::Cli::default();
    let test_app = TestApp::new(&docker).await;

    let mut connection = test_app.get_connection();
    diesel::sql_query(
        "\
INSERT INTO runners (start_number, firstname, lastname, team, email,
                     starting_point, running_level, donation,
                     reason_for_payment, payment_status, verification_code,
                     payment_confirmation_mail_sent, tshirt_cost, bsv_participant)
VALUES (42, 'Testy', 'McTest', 'Team Testy', 'some.email@example.com',
        'somewhere', 'super-duper', '10',
        'LGR-TESTY', false, 'befcf8a1-5acf-4590-ba96-9e95a3f82251',
        false, '10', false),
       (73, 'Hansi', 'McTest', 'Go Team!', 'hello@example.com',
        'somewhere', 'super-duper', '10',
        'LGR-HANSI', false, '20e47480-3165-45fe-bdbf-64dee3c347bd',
        false, '10', true),
       (111, 'Flora', 'McTest', 'Go Team!', NULL,
        'somewhere', 'super-duper', '10',
        'LGR-FLORA', false, 'c73b558b-3c1b-4602-a1e3-e98ae5d7b571',
        false, '10', true);",
    )
    .execute(&mut connection)
    .unwrap();

    let result = test_app
        .get_runners(
            1,
            "name",
            "Testy",
            false,
            Some(test_app.get_admin_cookie().await),
        )
        .await;

    assert_eq!(result.status(), actix_web::http::StatusCode::OK);
    assert_eq!(
        result
            .json::<serde_json::Value>()
            .await
            .unwrap()
            .get("runner_list")
            .unwrap()
            .get(0)
            .unwrap()
            .get("start_number")
            .unwrap()
            .as_i64()
            .unwrap(),
        42i64
    );
}

#[actix_web::test]
async fn get_runners_should_return_runners_with_matching_names_and_bsv_filter() {
    let docker = testcontainers::clients::Cli::default();
    let test_app = TestApp::new(&docker).await;

    let mut connection = test_app.get_connection();
    diesel::sql_query(
        "\
INSERT INTO runners (start_number, firstname, lastname, team, email,
                     starting_point, running_level, donation,
                     reason_for_payment, payment_status, verification_code,
                     payment_confirmation_mail_sent, tshirt_cost, bsv_participant)
VALUES (42, 'Testy', 'McTest', 'Team Testy', 'some.email@example.com',
        'somewhere', 'super-duper', '10',
        'LGR-TESTY', false, 'befcf8a1-5acf-4590-ba96-9e95a3f82251',
        false, '10', false),
       (73, 'Hansi', 'McTest', 'Go Team!', 'hello@example.com',
        'somewhere', 'super-duper', '10',
        'LGR-HANSI', false, '20e47480-3165-45fe-bdbf-64dee3c347bd',
        false, '10', true),
       (111, 'Flora', 'McTest', 'Go Team!', NULL,
        'somewhere', 'super-duper', '10',
        'LGR-FLORA', false, 'c73b558b-3c1b-4602-a1e3-e98ae5d7b571',
        false, '10', true);",
    )
    .execute(&mut connection)
    .unwrap();

    let result = test_app
        .get_runners(
            1,
            "name",
            "McTest",
            true,
            Some(test_app.get_admin_cookie().await),
        )
        .await;

    assert_eq!(result.status(), actix_web::http::StatusCode::OK);
    let value = result.json::<serde_json::Value>().await.unwrap();
    let runner_list = value.get("runner_list").unwrap();
    assert_eq!(
        runner_list
            .get(0)
            .unwrap()
            .get("start_number")
            .unwrap()
            .as_i64()
            .unwrap(),
        73i64
    );
    assert_eq!(
        runner_list
            .get(1)
            .unwrap()
            .get("start_number")
            .unwrap()
            .as_i64()
            .unwrap(),
        111i64
    );
}

#[actix_web::test]
async fn get_runners_should_return_runners_with_matching_email_addresses() {
    let docker = testcontainers::clients::Cli::default();
    let test_app = TestApp::new(&docker).await;

    let mut connection = test_app.get_connection();
    diesel::sql_query(
        "\
INSERT INTO runners (start_number, firstname, lastname, team, email,
                     starting_point, running_level, donation,
                     reason_for_payment, payment_status, verification_code,
                     payment_confirmation_mail_sent, tshirt_cost, bsv_participant)
VALUES (42, 'Testy', 'McTest', 'Team Testy', 'some.email@example.com',
        'somewhere', 'super-duper', '10',
        'LGR-TESTY', false, 'befcf8a1-5acf-4590-ba96-9e95a3f82251',
        false, '10', false),
       (73, 'Hansi', 'McTest', 'Go Team!', 'hello@example.com',
        'somewhere', 'super-duper', '10',
        'LGR-HANSI', false, '20e47480-3165-45fe-bdbf-64dee3c347bd',
        false, '10', true),
       (111, 'Flora', 'McTest', 'Go Team!', NULL,
        'somewhere', 'super-duper', '10',
        'LGR-FLORA', false, 'c73b558b-3c1b-4602-a1e3-e98ae5d7b571',
        false, '10', true);",
    )
    .execute(&mut connection)
    .unwrap();

    let result = test_app
        .get_runners(
            1,
            "email",
            "@example.com",
            false,
            Some(test_app.get_admin_cookie().await),
        )
        .await;

    assert_eq!(result.status(), actix_web::http::StatusCode::OK);
    assert_eq!(
        result
            .json::<serde_json::Value>()
            .await
            .unwrap()
            .get("runner_list")
            .unwrap()
            .get(0)
            .unwrap()
            .get("start_number")
            .unwrap()
            .as_i64()
            .unwrap(),
        42i64
    );
}

#[actix_web::test]
async fn get_runners_should_return_runners_with_matching_email_addresses_and_bsv_filter() {
    let docker = testcontainers::clients::Cli::default();
    let test_app = TestApp::new(&docker).await;

    let mut connection = test_app.get_connection();
    diesel::sql_query(
        "\
INSERT INTO runners (start_number, firstname, lastname, team, email,
                     starting_point, running_level, donation,
                     reason_for_payment, payment_status, verification_code,
                     payment_confirmation_mail_sent, tshirt_cost, bsv_participant)
VALUES (42, 'Testy', 'McTest', 'Team Testy', 'some.email@example.com',
        'somewhere', 'super-duper', '10',
        'LGR-TESTY', false, 'befcf8a1-5acf-4590-ba96-9e95a3f82251',
        false, '10', false),
       (73, 'Hansi', 'McTest', 'Go Team!', 'hello@example.com',
        'somewhere', 'super-duper', '10',
        'LGR-HANSI', false, '20e47480-3165-45fe-bdbf-64dee3c347bd',
        false, '10', true),
       (111, 'Flora', 'McTest', 'Go Team!', NULL,
        'somewhere', 'super-duper', '10',
        'LGR-FLORA', false, 'c73b558b-3c1b-4602-a1e3-e98ae5d7b571',
        false, '10', true);",
    )
    .execute(&mut connection)
    .unwrap();

    let result = test_app
        .get_runners(
            1,
            "email",
            "@example.com",
            true,
            Some(test_app.get_admin_cookie().await),
        )
        .await;

    assert_eq!(result.status(), actix_web::http::StatusCode::OK);
    assert_eq!(
        result
            .json::<serde_json::Value>()
            .await
            .unwrap()
            .get("runner_list")
            .unwrap()
            .get(0)
            .unwrap()
            .get("start_number")
            .unwrap()
            .as_i64()
            .unwrap(),
        73i64
    );
}

#[actix_web::test]
async fn get_runners_should_return_runners_with_matching_payment_reference() {
    let docker = testcontainers::clients::Cli::default();
    let test_app = TestApp::new(&docker).await;

    let mut connection = test_app.get_connection();
    diesel::sql_query(
        "\
INSERT INTO runners (start_number, firstname, lastname, team, email,
                     starting_point, running_level, donation,
                     reason_for_payment, payment_status, verification_code,
                     payment_confirmation_mail_sent, tshirt_cost, bsv_participant)
VALUES (42, 'Testy', 'McTest', 'Team Testy', 'some.email@example.com',
        'somewhere', 'super-duper', '10',
        'LGR-TESTY', false, 'befcf8a1-5acf-4590-ba96-9e95a3f82251',
        false, '10', false),
       (73, 'Hansi', 'McTest', 'Go Team!', 'hello@example.com',
        'somewhere', 'super-duper', '10',
        'LGR-HANSI', false, '20e47480-3165-45fe-bdbf-64dee3c347bd',
        false, '10', true),
       (111, 'Flora', 'McTest', 'Go Team!', NULL,
        'somewhere', 'super-duper', '10',
        'LGR-FLORA', false, 'c73b558b-3c1b-4602-a1e3-e98ae5d7b571',
        false, '10', true);",
    )
    .execute(&mut connection)
    .unwrap();

    let result = test_app
        .get_runners(
            1,
            "reason_for_payment",
            "LGR-TEST",
            false,
            Some(test_app.get_admin_cookie().await),
        )
        .await;

    assert_eq!(result.status(), actix_web::http::StatusCode::OK);
    assert_eq!(
        result
            .json::<serde_json::Value>()
            .await
            .unwrap()
            .get("runner_list")
            .unwrap()
            .get(0)
            .unwrap()
            .get("start_number")
            .unwrap()
            .as_i64()
            .unwrap(),
        42i64
    );
}

#[actix_web::test]
async fn get_runners_should_return_runners_with_matching_payment_reference_and_bsv_filter() {
    let docker = testcontainers::clients::Cli::default();
    let test_app = TestApp::new(&docker).await;

    let mut connection = test_app.get_connection();
    diesel::sql_query(
        "\
INSERT INTO runners (start_number, firstname, lastname, team, email,
                     starting_point, running_level, donation,
                     reason_for_payment, payment_status, verification_code,
                     payment_confirmation_mail_sent, tshirt_cost, bsv_participant)
VALUES (42, 'Testy', 'McTest', 'Team Testy', 'some.email@example.com',
        'somewhere', 'super-duper', '10',
        'LGR-TESTY', false, 'befcf8a1-5acf-4590-ba96-9e95a3f82251',
        false, '10', false),
       (73, 'Hansi', 'McTest', 'Go Team!', 'hello@example.com',
        'somewhere', 'super-duper', '10',
        'LGR-HANSI', false, '20e47480-3165-45fe-bdbf-64dee3c347bd',
        false, '10', true),
       (111, 'Flora', 'McTest', 'Go Team!', NULL,
        'somewhere', 'super-duper', '10',
        'LGR-FLORA', false, 'c73b558b-3c1b-4602-a1e3-e98ae5d7b571',
        false, '10', true);",
    )
    .execute(&mut connection)
    .unwrap();

    let result = test_app
        .get_runners(
            1,
            "reason_for_payment",
            "LGR-",
            true,
            Some(test_app.get_admin_cookie().await),
        )
        .await;

    assert_eq!(result.status(), actix_web::http::StatusCode::OK);
    assert_eq!(
        result
            .json::<serde_json::Value>()
            .await
            .unwrap()
            .get("runner_list")
            .unwrap()
            .get(0)
            .unwrap()
            .get("start_number")
            .unwrap()
            .as_i64()
            .unwrap(),
        73i64
    );
}

#[actix_web::test]
async fn update_payment_status_should_fail_if_user_is_unauthorized() {
    let docker = testcontainers::clients::Cli::default();
    let test_app = TestApp::new(&docker).await;

    let result = test_app.update_payment_status(1, true, None).await;

    assert_eq!(result.status(), actix_web::http::StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn update_payment_status_should_set_correct_value_in_db() {
    let docker = testcontainers::clients::Cli::default();
    let test_app = TestApp::new(&docker).await;

    let runner_id = diesel::sql_query(
        "\
INSERT INTO runners (start_number, firstname, lastname, team, email,
                     starting_point, running_level, donation,
                     reason_for_payment, payment_status, verification_code,
                     payment_confirmation_mail_sent, tshirt_cost, bsv_participant)
VALUES (42, 'Testy', 'McTest', 'Team Testy', 'some.email@example.com',
        'somewhere', 'super-duper', '10',
        'LGR-TESTY', false, 'befcf8a1-5acf-4590-ba96-9e95a3f82251',
        false, '10', false)
RETURNING *;",
    )
    .get_result::<Runner>(&mut test_app.get_connection())
    .unwrap()
    .id;

    let result = test_app
        .update_payment_status(runner_id, true, Some(test_app.get_admin_cookie().await))
        .await;

    assert_eq!(result.status(), actix_web::http::StatusCode::OK);

    let runner = diesel::sql_query("SELECT * FROM runners WHERE id = $1;")
        .bind::<diesel::sql_types::Integer, i32>(runner_id)
        .get_result::<Runner>(&mut test_app.get_connection())
        .unwrap();
    assert!(runner.payment_status);
}

#[actix_web::test]
async fn update_payment_status_should_send_email_if_payment_status_is_true_and_email_provided() {
    let docker = testcontainers::clients::Cli::default();
    let test_app = TestApp::new(&docker).await;

    let runner = diesel::sql_query(
        "\
INSERT INTO runners (start_number, firstname, lastname, team, email,
                     starting_point, running_level, donation,
                     reason_for_payment, payment_status, verification_code,
                     payment_confirmation_mail_sent, tshirt_cost, bsv_participant)
VALUES (42, 'Testy', 'McTest', 'Team Testy', 'some.email@example.com',
        'somewhere', 'super-duper', '10',
        'LGR-TESTY', false, 'befcf8a1-5acf-4590-ba96-9e95a3f82251',
        false, '10', false)
RETURNING *;",
    )
    .get_result::<Runner>(&mut test_app.get_connection())
    .unwrap();

    let result = test_app
        .update_payment_status(runner.id, true, Some(test_app.get_admin_cookie().await))
        .await;

    assert_eq!(result.status(), actix_web::http::StatusCode::OK);

    let test_email_server = test_app.get_email_server().unwrap();

    assert_eq!(
        test_email_server.get_last_recipient_email_addresses(),
        vec![runner.email.unwrap()]
    );
    assert!(test_email_server
        .get_last_mail_data()
        .unwrap()
        .contains(&runner.verification_code));
}

#[actix_web::test]
async fn get_full_runner_should_fail_if_user_is_unauthorized() {
    let docker = testcontainers::clients::Cli::default();
    let test_app = TestApp::new(&docker).await;

    let result = test_app.get_full_runner(1, None).await;

    assert_eq!(result.status(), actix_web::http::StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn get_full_runner_should_only_return_runner_info_if_no_shipping_info_exists() {
    let docker = testcontainers::clients::Cli::default();
    let test_app = TestApp::new(&docker).await;

    let runner_id = diesel::sql_query(
        "\
INSERT INTO runners (start_number, firstname, lastname, team, email,
                     starting_point, running_level, donation,
                     reason_for_payment, payment_status, verification_code,
                     payment_confirmation_mail_sent, tshirt_cost, bsv_participant)
VALUES (42, 'Testy', 'McTest', 'Team Testy', 'some.email@example.com',
        'somewhere', 'super-duper', '10',
        'LGR-TESTY', false, 'befcf8a1-5acf-4590-ba96-9e95a3f82251',
        false, '0', false)
RETURNING *;",
    )
    .get_result::<Runner>(&mut test_app.get_connection())
    .unwrap()
    .id;

    let result = test_app
        .get_full_runner(runner_id, Some(test_app.get_admin_cookie().await))
        .await;

    assert_eq!(result.status(), actix_web::http::StatusCode::OK);
    let json_body = result.json::<serde_json::Value>().await.unwrap();
    assert_eq!(
        json_body
            .get("runner_id")
            .unwrap()
            .as_str()
            .unwrap()
            .parse::<i32>()
            .unwrap(),
        runner_id
    );
    assert!(json_body.get("delivery_status").is_none());
}

#[actix_web::test]
async fn get_full_runner_should_include_shipping_info_if_matching_exists() {
    let docker = testcontainers::clients::Cli::default();
    let test_app = TestApp::new(&docker).await;

    let runner_id = diesel::sql_query(
        "\
INSERT INTO runners (start_number, firstname, lastname, team, email,
                     starting_point, running_level, donation,
                     reason_for_payment, payment_status, verification_code,
                     payment_confirmation_mail_sent, tshirt_cost, bsv_participant)
VALUES (42, 'Testy', 'McTest', 'Team Testy', 'some.email@example.com',
        'somewhere', 'super-duper', '10',
        'LGR-TESTY', false, 'befcf8a1-5acf-4590-ba96-9e95a3f82251',
        false, '15', false)
RETURNING *;",
    )
    .get_result::<Runner>(&mut test_app.get_connection())
    .unwrap()
    .id;

    diesel::sql_query(
        "\
INSERT INTO shippings (tshirt_model, tshirt_size, country, firstname, lastname,
                       street_name, house_number, address_extra, postal_code,
                       city, runner_id)
VALUES ('unisex', 'm', 'Deutschland', 'Testy', 'McTest',
        'Testy-McTest-Str', '42', NULL, '12345',
        'Metropolis', $1);",
    )
    .bind::<diesel::sql_types::Integer, i32>(runner_id)
    .execute(&mut test_app.get_connection())
    .unwrap();

    let result = test_app
        .get_full_runner(runner_id, Some(test_app.get_admin_cookie().await))
        .await;

    assert_eq!(result.status(), actix_web::http::StatusCode::OK);
    let json_body = result.json::<serde_json::Value>().await.unwrap();
    assert_eq!(
        json_body
            .get("runner_id")
            .unwrap()
            .as_str()
            .unwrap()
            .parse::<i32>()
            .unwrap(),
        runner_id
    );
    assert_eq!(
        json_body
            .get("delivery_status")
            .map(|v| v.as_str().unwrap()),
        Some("Noch nicht verschickt")
    );
}

#[actix_web::test]
async fn update_runner_should_fail_if_user_is_unauthorized() {
    let docker = testcontainers::clients::Cli::default();
    let test_app = TestApp::new(&docker).await;

    let runner_id = diesel::sql_query(
        "\
INSERT INTO runners (start_number, firstname, lastname, team, email,
                     starting_point, running_level, donation,
                     reason_for_payment, payment_status, verification_code,
                     payment_confirmation_mail_sent, tshirt_cost, bsv_participant)
VALUES (42, 'Testy', 'McTest', 'Team Testy', 'some.email@example.com',
        'somewhere', 'super-duper', '10',
        'LGR-TESTY', false, 'befcf8a1-5acf-4590-ba96-9e95a3f82251',
        false, '15', false)
RETURNING *;",
    )
    .get_result::<Runner>(&mut test_app.get_connection())
    .unwrap()
    .id;

    let body = format!(
        "{{\
            \"is_tshirt_booked\": false,\
            \"runner_id\": \"{runner_id}\",\
            \"firstname\": \"Tasty\",\
            \"lastname\": \"McTest\",\
            \"team\": \"Team Tasty\",\
            \"bsv_participant\": false,\
            \"email\": \"tasty@nomnom.com\",\
            \"starting_point\": \"nowhere\",\
            \"running_level\": \"over 9000\",\
            \"donation\": \"10\",\
            \"start_number\": \"42\",\
            \"verification_code\": \"befcf8a1-5acf-4590-ba96-9e95a3f82251\",\
            \"reason_for_payment\": \"LGR-TASTY\",\
            \"payment_status\": false,\
            \"payment_confirmation_mail_sent\": false\
        }}"
    );

    let result = test_app.update_runner(runner_id, body, None).await;

    assert_eq!(result.status(), actix_web::http::StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn update_runner_should_update_runner_info() {
    let docker = testcontainers::clients::Cli::default();
    let test_app = TestApp::new(&docker).await;

    let runner_id = diesel::sql_query(
        "\
INSERT INTO runners (start_number, firstname, lastname, team, email,
                     starting_point, running_level, donation,
                     reason_for_payment, payment_status, verification_code,
                     payment_confirmation_mail_sent, tshirt_cost, bsv_participant)
VALUES (42, 'Testy', 'McTest', 'Team Testy', 'some.email@example.com',
        'somewhere', 'super-duper', '10',
        'LGR-TESTY', false, 'befcf8a1-5acf-4590-ba96-9e95a3f82251',
        false, '15', false)
RETURNING *;",
    )
    .get_result::<Runner>(&mut test_app.get_connection())
    .unwrap()
    .id;

    let body = format!(
        "{{\
            \"is_tshirt_booked\": false,\
            \"runner_id\": \"{runner_id}\",\
            \"firstname\": \"Tasty\",\
            \"lastname\": \"McTest\",\
            \"team\": \"Team Tasty\",\
            \"bsv_participant\": false,\
            \"email\": \"tasty@nomnom.com\",\
            \"starting_point\": \"nowhere\",\
            \"running_level\": \"over 9000\",\
            \"donation\": \"10\",\
            \"start_number\": \"42\",\
            \"verification_code\": \"befcf8a1-5acf-4590-ba96-9e95a3f82251\",\
            \"reason_for_payment\": \"LGR-TASTY\",\
            \"payment_status\": false,\
            \"payment_confirmation_mail_sent\": false\
        }}"
    );

    let result = test_app
        .update_runner(runner_id, body, Some(test_app.get_admin_cookie().await))
        .await;

    assert_eq!(result.status(), actix_web::http::StatusCode::OK);

    let runner = diesel::sql_query("SELECT * FROM runners WHERE id = $1;")
        .bind::<diesel::sql_types::Integer, i32>(runner_id)
        .get_result::<Runner>(&mut test_app.get_connection())
        .unwrap();

    assert_eq!(runner.email, Some("tasty@nomnom.com".to_string()));
}

#[actix_web::test]
async fn update_runner_should_update_shipping_info_if_present() {
    let docker = testcontainers::clients::Cli::default();
    let test_app = TestApp::new(&docker).await;

    let runner_id = diesel::sql_query(
        "\
INSERT INTO runners (start_number, firstname, lastname, team, email,
                     starting_point, running_level, donation,
                     reason_for_payment, payment_status, verification_code,
                     payment_confirmation_mail_sent, tshirt_cost, bsv_participant)
VALUES (42, 'Testy', 'McTest', 'Team Testy', 'some.email@example.com',
        'somewhere', 'super-duper', '10',
        'LGR-TESTY', false, 'befcf8a1-5acf-4590-ba96-9e95a3f82251',
        false, '15', false)
RETURNING *;",
    )
    .get_result::<Runner>(&mut test_app.get_connection())
    .unwrap()
    .id;

    diesel::sql_query(
        "\
INSERT INTO shippings (tshirt_model, tshirt_size, country, firstname, lastname,
                       street_name, house_number, address_extra, postal_code,
                       city, runner_id)
VALUES ('unisex', 'm', 'Deutschland', 'Testy', 'McTest',
        'Testy-McTest-Str', '42', NULL, '12345',
        'Metropolis', $1);",
    )
    .bind::<diesel::sql_types::Integer, i32>(runner_id)
    .execute(&mut test_app.get_connection())
    .unwrap();

    let body = format!(
        "{{\
            \"is_tshirt_booked\": true,\
            \"runner_id\": \"{runner_id}\",\
            \"firstname\": \"Tasty\",\
            \"lastname\": \"McTest\",\
            \"team\": \"Team Tasty\",\
            \"bsv_participant\": false,\
            \"email\": \"tasty@nomnom.com\",\
            \"starting_point\": \"nowhere\",\
            \"running_level\": \"over 9000\",\
            \"donation\": \"10\",\
            \"start_number\": \"42\",\
            \"verification_code\": \"befcf8a1-5acf-4590-ba96-9e95a3f82251\",\
            \"reason_for_payment\": \"LGR-TASTY\",\
            \"payment_status\": false,\
            \"payment_confirmation_mail_sent\": false,\
            \"tshirt_model\": \"unisex\",\
            \"tshirt_size\": \"l\",\
            \"country\": \"Deutschland\",\
            \"address_firstname\": \"Tasty\",\
            \"address_lastname\": \"McTest\",\
            \"street_name\": \"Tasty-McTest-Str\",\
            \"house_number\": \"42\",\
            \"postal_code\": \"12345\",\
            \"city\": \"Metropolis\",\
            \"delivery_status\": \"In Bearbeitung\"\
        }}"
    );

    let result = test_app
        .update_runner(runner_id, body, Some(test_app.get_admin_cookie().await))
        .await;

    assert_eq!(result.status(), actix_web::http::StatusCode::OK);

    use diesel::{prelude::*, QueryDsl, RunQueryDsl};

    let shipping = pace::schema::shippings::dsl::shippings
        .filter(pace::schema::shippings::dsl::runner_id.eq(&runner_id))
        .get_result::<Shipping>(&mut test_app.get_connection())
        .unwrap();

    assert_eq!(shipping.tshirt_size, "l");
}
