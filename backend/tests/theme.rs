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
