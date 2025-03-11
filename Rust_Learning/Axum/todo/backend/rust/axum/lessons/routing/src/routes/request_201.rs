use axum::{http::StatusCode, response::{IntoResponse, Response}};

pub async fn request_201() ->Response {
    (
        StatusCode::CREATED,
        "This is a 201 Created response",
    ).into_response()
}