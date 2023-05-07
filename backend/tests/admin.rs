use diesel::RunQueryDsl;
use pace::models::users::{LoginData, PasswordChangeData};
use pace::{insert_rejected_transaction, models::rejected_transaction::NewRejectedTransaction};
use std::collections::HashMap;

pub mod helpers;
pub use helpers::{TestApp, TestDatabase};

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
