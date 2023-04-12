use reqwest::StatusCode;

mod helpers;

#[actix_web::test]
async fn get_theme_should_return_theme_settings() {
    let address = helpers::create_app().await;
    let client = reqwest::Client::new();

    let actual_response = client
        .get(format!("{address}/api/theme"))
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("unable to send request");

    assert_eq!(actual_response.status(), StatusCode::OK);
    let response_json = helpers::extract_json_values(actual_response).await;

    assert_eq!(
        "Lauf gegen Rechts",
        response_json.get("event_name").unwrap()
    )
}
