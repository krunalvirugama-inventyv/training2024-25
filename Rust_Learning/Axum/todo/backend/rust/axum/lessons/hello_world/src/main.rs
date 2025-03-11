use axum::{routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // let app = Router::new().route("/" , get(|| async {"Hello Wold!!"}));

    let app = Router::new().route("/", get(hello_world));

    let listner: TcpListener = TcpListener::bind(&"0.0.0.0:3000").await.unwrap();

    axum::serve(listner, app.into_make_service()).await.unwrap();
}

async fn hello_world() -> String {
    //    "Hello World !!!".to_string()
    "Hello World Changes !!!".to_owned()
}
