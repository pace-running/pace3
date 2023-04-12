use reqwest::Response;
use serde_json::Map;
use std::net::TcpListener;

use pace::{get_connection_pool, run};

pub async fn create_app() -> String {
    let listener = TcpListener::bind(("127.0.0.1", 0)).expect("Unable to bind random port.");
    let port = listener.local_addr().unwrap().port();
    let pool = get_connection_pool().expect("Could not initialize connection pool");
    let server = run(listener, pool).expect("Unable to bind address");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{port}")
}

pub async fn extract_json_values(actual_response: Response) -> Map<String, serde_json::Value> {
    actual_response
        .json::<serde_json::Value>()
        .await
        .unwrap()
        .as_object()
        .unwrap()
        .clone()
}
