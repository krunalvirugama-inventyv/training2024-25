use axum::{
    body::Body, http::{header::HeaderValue, Request, StatusCode}, middleware::Next, response::{IntoResponse, Response}
};

use crate::utils::responce::error_response;


pub async fn extract_id_to_header(mut req:Request<Body>, next : Next) -> Response {
    let path = req.uri().path().to_string();

    let id_segment = path.split('/').last();

    match id_segment.and_then(|id| id.parse::<i32>().ok()) {
        Some(id_value) => {
            req.headers_mut()
                .insert("x-employee-id", HeaderValue::from_str(&id_value.to_string()).unwrap());
            next.run(req).await.into_response()
        }
        None => {
            error_response("Invalid ID in URL", StatusCode::BAD_REQUEST)
        }
    }


}