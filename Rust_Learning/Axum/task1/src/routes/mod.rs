use axum::{middleware, routing::get, Router};
use sqlx::MySqlPool;
use tower_http::trace::TraceLayer;
mod user;
mod employee;
use crate::middleware::log::log;
use tower_cookies::CookieManagerLayer; 

pub async fn create_routes(pool: MySqlPool) -> Router {
    Router::new()
        .nest("/employee" ,employee::routes(pool.clone()))
        // here add auth middleware problem /employee to check unneccary check auth middleware if not match extact path only match /employee than checking
        .nest("/user", user::routes())
        .with_state(pool)
        .layer(TraceLayer::new_for_http())
        .layer(CookieManagerLayer::new())
        // .layer(middleware::from_fn(log))
}
