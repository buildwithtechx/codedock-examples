use std::env;
use std::net::SocketAddr;

use axum::{Json, Router, routing::get};
use serde_json::{Value, json};

async fn hello() -> Json<Value> {
    Json(json!({"message": "Hello from Codedock Rust Axum Example!"}))
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello));
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{port}").parse().expect("valid socket address");
    let listener = tokio::net::TcpListener::bind(addr).await.expect("bind listener");
    axum::serve(listener, app).await.expect("serve app");
}