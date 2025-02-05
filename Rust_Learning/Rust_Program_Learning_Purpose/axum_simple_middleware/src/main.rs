use axum::{
    routing::get,
    Router,
};

mod middleware;
use middleware::auth::auth_middleware;

mod routes; 
use routes::user::get_users;

#[tokio::main]

async fn main() {
    let layer = Router::new()
        .route("/user", get(get_users))
        .layer(axum::middleware::from_fn(auth_middleware));
    let app = layer;


    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
