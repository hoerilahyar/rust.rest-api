mod handlers;
mod models;
mod state;

use axum::{
    routing::{delete, get, post},
    Router,
};

#[tokio::main]
async fn main() {
    let state = state::new_state();

    let app = Router::new()
        .route("/users", get(handlers::list_users))
        .route("/users", post(handlers::create_user))
        .route("/users/:id", get(handlers::get_user))
        .route("/users/:id", delete(handlers::delete_user))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Server running at http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}