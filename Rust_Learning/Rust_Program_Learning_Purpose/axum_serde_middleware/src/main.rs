use axum::{middleware, routing::post, Router};

use std::sync::Arc;
use tokio::sync::Mutex;

mod middleware_;
use middleware_::json_extractor::json_extractor_middleware;

mod routes; 
use routes::user::{create_user, AppState};

#[tokio::main]

async fn main() {
    let state = AppState {
        data: Arc::new(Mutex::new(vec![])),
    };


    let user_routes = Router::new()
    .route("/user", post(create_user))
    .layer(middleware::from_fn(json_extractor_middleware));
    let user_routes = Router::new()
    .route("/user", post(create_user))
        .layer(axum::middleware::from_fn(json_extractor_middleware));

        let app = Router::new().nest("/api", user_routes).with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
