use axum::{Json, Router, routing::get};

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct Hello {
    message: String,
}

async fn hello() -> Json<Hello> {
    Json(Hello {
        message: "Hello, World!".to_string(),
    })
}

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    message: String,
    username: String,
    password: String,
    success: bool,
}

async fn login(Json(payload): Json<LoginRequest>) -> Json<LoginResponse> {
    Json(LoginResponse {
        message: "Login berhasil".to_string(),
        username: payload.username,
        password: payload.password,
        success: true,
    })
}

#[tokio::main]
async fn main() {
    println!("Server is starting ...");

    let app = Router::new()
        .route("/", get(hello))
        .route("/login", axum::routing::post(login));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    println!("Server is running on http://0.0.0.0:8080");

    axum::serve(listener, app).await.unwrap();
}
