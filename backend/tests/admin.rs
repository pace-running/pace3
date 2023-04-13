use pace::models::users::{LoginData, PasswordChangeData};
use pace::{
    get_connection_pool, insert_rejected_transaction,
    models::rejected_transaction::NewRejectedTransaction,
};

mod helpers;

#[test]
fn put_rejected_transaction_into_database() {
    let conn = &mut get_connection_pool()
        .expect("Unable to get connection pool.")
        .get()
        .expect("Unable to get connection.");
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
    let address = helpers::create_app().await;
    let client = reqwest::Client::new();

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
    // TODO: ensure that there is a user with username admin
    //       and a password different than "not the correct password!".
    //       Right now this tests expects a db running via docker-compose
    //       With certain values already existing in the DB.

    let address = helpers::create_app().await;
    let client = reqwest::Client::new();

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
    let address = helpers::create_app().await;
    let client = reqwest::Client::new();

    let login_response = client
        .post(format!("{address}/api/admin/login"))
        .header("Content-Type", "application/json")
        .body(
            serde_json::to_string(&LoginData {
                username: "admin".to_string(),
                password: "xoh7Ongui4oo".to_string(),
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
        old_password: "xoh7Ongui4oo".to_string(),
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
    let address = helpers::create_app().await;
    let client = reqwest::Client::new();

    let login_response = client
        .post(format!("{address}/api/admin/login"))
        .header("Content-Type", "application/json")
        .body(
            serde_json::to_string(&LoginData {
                username: "admin".to_string(),
                password: "xoh7Ongui4oo".to_string(),
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
        new_password: "new_password".to_string(),
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
    let address = helpers::create_app().await;
    let client = reqwest::Client::new();

    let login_response = client
        .post(format!("{address}/api/admin/login"))
        .header("Content-Type", "application/json")
        .body(
            serde_json::to_string(&LoginData {
                username: "admin".to_string(),
                password: "xoh7Ongui4oo".to_string(),
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
        new_password: "new_password".to_string(),
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
    let address = helpers::create_app().await;
    let client = reqwest::Client::new();

    let login_response = client
        .post(format!("{address}/api/admin/login"))
        .header("Content-Type", "application/json")
        .body(
            serde_json::to_string(&LoginData {
                username: "admin".to_string(),
                password: "xoh7Ongui4oo".to_string(),
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
        old_password: "xoh7Ongui4oo".to_string(),
        new_password: "new_password".to_string(),
    };

    let actual_response = client
        .put(format!("{address}/api/admin/change_password"))
        .header("Content-Type", "application/json")
        .header("Cookie", format!("{}={}", cookie.name(), cookie.value()))
        .body(serde_json::to_string(&password_change_data).unwrap())
        .send()
        .await
        .expect("Unable to send request.");

    // TODO we need to change the password back, as it is required in other places
    let _ = client
        .put(format!("{address}/api/admin/change_password"))
        .header("Content-Type", "application/json")
        .header("Cookie", format!("{}={}", cookie.name(), cookie.value()))
        .body(
            serde_json::to_string(&PasswordChangeData {
                old_password: "new_password".to_string(),
                new_password: "xoh7Ongui4oo".to_string(),
            })
            .unwrap(),
        )
        .send()
        .await
        .expect("Unable to send request.");

    assert_eq!(actual_response.status(), actix_web::http::StatusCode::OK);
}
