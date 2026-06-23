use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::{
    models::{CreateUserRequest, User},
    state::AppState,
};

// GET /users
pub async fn list_users(State(state): State<AppState>) -> Json<Vec<User>> {
    let users = state.lock().unwrap();
    Json(users.clone())
}

// GET /users/:id
pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<User>, StatusCode> {
    let users = state.lock().unwrap();
    users
        .iter()
        .find(|u| u.id == id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

// POST /users
pub async fn create_user(
    State(state): State<AppState>,
    Json(body): Json<CreateUserRequest>,
) -> (StatusCode, Json<User>) {
    let user = User {
        id: Uuid::new_v4(),
        name: body.name,
        email: body.email,
    };
    state.lock().unwrap().push(user.clone());
    (StatusCode::CREATED, Json(user))
}

// DELETE /users/:id
pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> StatusCode {
    let mut users = state.lock().unwrap();
    let before = users.len();
    users.retain(|u| u.id != id);
    if users.len() < before {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}