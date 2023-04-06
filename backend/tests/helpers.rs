use std::net::TcpListener;
use reqwest::Response;
use serde_json::Map;

use pace::run;

pub fn create_app() -> String {
    let listener = TcpListener::bind(("127.0.0.1", 0)).expect("Unable to bind random port.");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Unable to bind address");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{port}")
}

pub async fn extract_json_values(actual_response: Response) -> Map<String, serde_json::Value> {
    actual_response.json::<serde_json::Value>().await.unwrap().as_object().unwrap().clone()
}
