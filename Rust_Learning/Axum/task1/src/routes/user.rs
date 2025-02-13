use axum::{routing::{get, post}, Router};
use sqlx::MySqlPool;
use crate::handlers::user::{login, register_user};

pub fn routes() -> Router<MySqlPool>{
    Router::new().route("/register_user", post(register_user))
    .route("/login", get(login))
}

