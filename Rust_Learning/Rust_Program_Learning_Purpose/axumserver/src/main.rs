// axum is a web application framework that focuses on ergonomics and modularity.

use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main(){
     // build our application with a single route
    let app = Router::new()
              .route("/" , get(root))
              .route("/foo" , get(get_foo).post(post_foo))
              .route("/foo/bar" ,get(foo_bar));

  // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener,app).await.unwrap();
}

async fn root() -> &'static str { "Root Hello World"}
async fn post_foo()  -> &'static str  { "Post Foo /foo "}
async fn get_foo()  -> &'static str  { "Get Foo /foo "}
async fn foo_bar()  -> &'static str  { "Get Foo in Bar /foo/bar" }

