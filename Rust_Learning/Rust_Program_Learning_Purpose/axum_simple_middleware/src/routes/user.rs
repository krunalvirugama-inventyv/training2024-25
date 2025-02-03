use axum::{Json, response::IntoResponse};

pub async fn get_users() -> impl IntoResponse {
    Json(vec!["user1", "user2", "user3"])
}