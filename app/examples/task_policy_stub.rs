use axum::{
    Json, Router,
    http::HeaderMap,
    routing::{get, post},
};
use serde_json::{Value, json};
use std::{path::Path, time::Duration};

async fn check(headers: HeaderMap, Json(_request): Json<Value>) -> Json<Value> {
    eprintln!(
        "task policy request received; traceparent_present={}",
        headers.contains_key("traceparent")
    );
    if let Ok(gate) = std::env::var("TASK_POLICY_STUB_BLOCK_FILE") {
        while Path::new(&gate).exists() {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    }
    Json(json!({"decision": "allowed"}))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address =
        std::env::var("TASK_POLICY_STUB_ADDR").unwrap_or_else(|_| "127.0.0.1:4001".into());
    let listener = tokio::net::TcpListener::bind(&address).await?;
    println!("task policy stub listening; address={address}");
    axum::serve(
        listener,
        Router::new()
            .route("/health/live", get(|| async { "ok" }))
            .route("/check", post(check)),
    )
    .await?;
    Ok(())
}
