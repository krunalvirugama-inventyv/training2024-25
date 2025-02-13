use tokio::net::TcpListener;

mod routes;
mod handlers;
mod middleware;
mod utils;
mod db;
use routes::create_routes;
use db::connection;

pub async fn start_server() {
    let pool = connection::create_db_pool().await;

    let app = create_routes(pool).await;
    let listner =  TcpListener::bind(&"0.0.0.0:3001").await.unwrap();
    axum::serve(listner,app.into_make_service()).await.unwrap();
}
