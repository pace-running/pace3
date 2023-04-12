use actix_web::http::StatusCode;
use actix_web::web::Json;
use pace::builders::InfoBuilder;
use pace::handlers::runners::{
    Response, ResponseWithBody, RunnerDetails, RunnerResponse, ShippingDetails,
};
use pace::models::runner::create_verification_code;

mod helpers;

#[actix_web::test]
async fn create_runner_should_be_successful_if_only_participant_info_is_provided() {
    let address = helpers::create_app().await;
    let client = reqwest::Client::new();

    let participant = InfoBuilder::minimal_default().build();
    let input_data = Json(participant);

    let actual_response = client
        .post(format!("{address}/api/runners"))
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&input_data).unwrap())
        .send()
        .await
        .expect("Unable to send request.");

    assert_eq!(actual_response.status(), StatusCode::OK);

    let response_json = helpers::extract_json_values(actual_response).await;
    assert_eq!(
        "Data received",
        response_json.get("success_message").unwrap()
    )
}

#[actix_web::test]
async fn create_runner_should_be_successful_if_participant_and_shipping_info_are_provided() {
    let address = helpers::create_app().await;
    let client = reqwest::Client::new();

    let participant = InfoBuilder::default_info().build();
    let input_data = Json(participant);

    let actual_response = client
        .post(format!("{address}/api/runners"))
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&input_data).unwrap())
        .send()
        .await
        .expect("Unable to send request.");

    assert_eq!(actual_response.status(), StatusCode::OK);

    let response_json = helpers::extract_json_values(actual_response).await;
    assert_eq!(
        "Data received",
        response_json.get("success_message").unwrap()
    )
}

#[actix_web::test]
async fn create_runner_should_fail_if_participant_info_is_incomplete() {
    let address = helpers::create_app().await;
    let client = reqwest::Client::new();

    let participant = InfoBuilder::default_info().with_house_number("").build();
    let input_data = Json(participant);

    let actual_response = client
        .post(format!("{address}/api/runners"))
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&input_data).unwrap())
        .send()
        .await
        .expect("Unable to send request.");

    assert_eq!(actual_response.status(), StatusCode::BAD_REQUEST);

    let response_json = helpers::extract_json_values(actual_response).await;
    assert_eq!("Bad data", response_json.get("error_message").unwrap())
}

#[actix_web::test]
async fn get_runner_should_return_runner_info_for_correct_runner_id_and_verification_code() {
    let address = helpers::create_app().await;
    let client = reqwest::Client::new();

    // test setup

    let participant = InfoBuilder::default_info().build();
    let input_data = Json(participant.clone());

    let create_runner_response = client
        .post(format!("{address}/api/runners"))
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&input_data).unwrap())
        .send()
        .await
        .expect("Unable to send request.");

    assert_eq!(create_runner_response.status(), StatusCode::OK);

    let bytes = create_runner_response.bytes().await.unwrap();
    let created_runner: ResponseWithBody = serde_json::from_slice(&bytes).unwrap();
    let runner_id = created_runner
        .runner_id
        .as_ref()
        .unwrap()
        .parse::<i32>()
        .unwrap();
    let verification_code = created_runner.verification_code.unwrap();

    // actual test

    let actual_response = client
        .get(format!(
            "{address}/api/runners/{runner_id}?verification_code={verification_code}"
        ))
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("Unable to send request.");

    assert_eq!(actual_response.status(), StatusCode::OK);
    let bytes = actual_response.bytes().await.unwrap();
    let returned_runner: RunnerResponse = serde_json::from_slice(&bytes).unwrap();
    let expected_runner = RunnerResponse {
        runner_details: Option::from(RunnerDetails {
            runner_id: created_runner.runner_id.unwrap(),
            start_number: created_runner.start_number.unwrap().to_string(),
            tshirt_cost: created_runner.tshirt_cost.unwrap(),
            donation: created_runner.donation.unwrap(),
            payment: created_runner.reason_for_payment.unwrap(),
            is_paid: false,
        }),
        is_tshirt_booked: true,
        shipping_details: Option::from(ShippingDetails {
            tshirt_model: participant.shipping_info.tshirt_model.to_string(),
            tshirt_size: participant.shipping_info.tshirt_size.to_string(),
            country: participant.shipping_info.country.to_string(),
            address_firstname: participant.shipping_info.address_firstname.to_string(),
            address_lastname: participant.shipping_info.address_lastname.to_string(),
            street_name: participant.shipping_info.street_name.to_string(),
            house_number: participant.shipping_info.house_number.to_string(),
            address_extra: Some(participant.shipping_info.address_extra.to_string()),
            postal_code: participant.shipping_info.postal_code.to_string(),
            city: participant.shipping_info.city.to_string(),
            delivery_status: "In Bearbeitung".to_string(),
        }),
        inner_response: Response {
            success_message: Some("Data received".to_string()),
            error_message: None,
            status_code: StatusCode::OK.as_u16(),
        },
    };
    assert_eq!(returned_runner, expected_runner);
}

#[actix_web::test]
async fn get_runner_should_fail_if_wrong_verification_code_is_send() {
    let address = helpers::create_app().await;
    let client = reqwest::Client::new();

    // test setup

    let participant = InfoBuilder::default_info().build();
    let input_data = Json(participant.clone());

    let create_runner_response = client
        .post(format!("{address}/api/runners"))
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&input_data).unwrap())
        .send()
        .await
        .expect("Unable to send request.");

    assert_eq!(create_runner_response.status(), StatusCode::OK);

    let bytes = create_runner_response.bytes().await.unwrap();
    let created_runner: ResponseWithBody = serde_json::from_slice(&bytes).unwrap();
    let runner_id = created_runner
        .runner_id
        .as_ref()
        .unwrap()
        .parse::<i32>()
        .unwrap();
    let verification_code = create_verification_code();

    // actual test

    let actual_response = client
        .get(format!(
            "{address}/api/runners/{runner_id}?verification_code={verification_code}"
        ))
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("Unable to send request.");

    assert_eq!(actual_response.status(), StatusCode::FORBIDDEN);
    let bytes = actual_response.bytes().await.unwrap();
    let actual_runner_response: Response = serde_json::from_slice(&bytes).unwrap();
    let expected_runner_response = Response {
        success_message: None,
        error_message: Some("Code could not be verified".to_string()),
        status_code: StatusCode::FORBIDDEN.as_u16(),
    };
    assert_eq!(expected_runner_response, actual_runner_response);
}
