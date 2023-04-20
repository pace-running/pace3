use pace::handlers::theme::ThemeData;
use reqwest::StatusCode;
mod helpers;
use crate::helpers::{extract_json_values, TestApp};

#[actix_web::test]
async fn get_theme_should_return_theme_settings() {
    let docker = testcontainers::clients::Cli::default();
    let test_app = TestApp::new(&docker).await;
    let address = test_app.get_address();
    let client = test_app.get_client();

    let actual_response = client
        .get(format!("{address}/api/theme"))
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("unable to send request");

    assert_eq!(actual_response.status(), StatusCode::OK);
    let response_json = extract_json_values(actual_response).await;

    assert_eq!(
        "Lauf gegen Rechts",
        response_json.get("event_name").unwrap()
    )
}

#[actix_web::test]
async fn update_theme_should_be_successful_with_valid_data() {
    let docker = testcontainers::clients::Cli::default();
    let test_app = TestApp::new(&docker).await;
    let data = ThemeData {
        event_title: "test title".to_string(),
        event_description: "description".to_string(),
        closed_registration_message: "registration is closed".to_string(),
        is_registration_open: false,
        tshirts_enabled: true,
    };

    let admin_cookie = test_app.get_admin_cookie().await;

    let response = test_app.update_theme(data, Some(admin_cookie)).await;
    assert_eq!(response.status(), actix_web::http::StatusCode::OK.as_u16());
}

#[actix_web::test]
async fn update_theme_should_fail_without_login() {
    let docker = testcontainers::clients::Cli::default();
    let test_app = TestApp::new(&docker).await;
    let data = ThemeData {
        event_title: "test title".to_string(),
        event_description: "description".to_string(),
        closed_registration_message: "registration is closed".to_string(),
        is_registration_open: false,
        tshirts_enabled: true,
    };

    let response = test_app.update_theme(data, None).await;
    assert_eq!(
        response.status(),
        actix_web::http::StatusCode::UNAUTHORIZED.as_u16()
    );
}
