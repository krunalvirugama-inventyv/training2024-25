mod hello_world;
mod mirror_body_string;
mod mirror_body_json;
mod path_variable;
mod query_params;
mod mirror_user_agent;
mod mirror_custom_header;

use hello_world::hello_world;
use mirror_body_string::mirror_body_string;
use mirror_body_json::mirror_body_json;
use path_variable::{hard_code_path, path_variable};
use query_params::query_params;
use mirror_user_agent::mirror_user_agent;
use mirror_custom_header::mirror_custom_header;
use axum::{ routing::{get, post}, Router};

pub fn create_routes() -> Router<> {
    Router::new().route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variable/{id}", get(path_variable))
        .route("/path_variable/15", get(hard_code_path))
        .route("/query_params" , get(query_params))
        .route("/mirror_user_agent", get(mirror_user_agent))
        .route("/mirror_custom_header", get(mirror_custom_header))

}


