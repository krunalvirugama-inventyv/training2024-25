mod hello_world;
mod middleware_message;
mod mirror_body_json;
mod mirror_body_string;
mod mirror_custom_header;
mod mirror_user_agent;
mod path_variable;
mod query_params;
mod read_middleware_custom_header;
mod set_middleware_custom_header;
mod always_errors;

use axum::{
    http::Method,
    middleware,
    routing::{get, post},
    Extension, Router,
};
use hello_world::hello_world;
use middleware_message::middleware_message;
use mirror_body_json::mirror_body_json;
use mirror_body_string::mirror_body_string;
use mirror_custom_header::mirror_custom_header;
use mirror_user_agent::mirror_user_agent;
use path_variable::{hard_code_path, path_variable};
use query_params::query_params;
use read_middleware_custom_header::read_middleware_custom_header;
use set_middleware_custom_header::set_middleware_custom_header;
use tower_http::cors::{Any, CorsLayer};
use always_errors::always_errors;

#[derive(Clone)]
pub struct SharedData {
    pub message: String,
}

pub fn create_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::POST, Method::GET])
        .allow_origin(Any);

    let shared_data = SharedData {
        message: String::from("Hello I Am Shared State Message "),
    };

    Router::new()
        .route(
            "/read_middleware_custom_header",
            get(read_middleware_custom_header),
        )
        .route_layer(middleware::from_fn(set_middleware_custom_header))
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variable/{id}", get(path_variable))
        .route("/path_variable/15", get(hard_code_path))
        .route("/query_params", get(query_params))
        .route("/mirror_user_agent", get(mirror_user_agent))
        .route("/mirror_custom_header", get(mirror_custom_header))
        .route("/middleware_message", get(middleware_message))
        .layer(Extension(shared_data))
        .layer(cors)
        .route("/always_errors", get(always_errors))
}
