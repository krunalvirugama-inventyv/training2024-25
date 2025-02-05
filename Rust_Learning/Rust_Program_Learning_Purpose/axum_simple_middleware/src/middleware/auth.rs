use axum::{extract::Request, middleware::Next, response::{IntoResponse, Response}};
use hyper::StatusCode;
        
pub async fn auth_middleware(request: Request, next: Next) -> Response {
    match request.headers().get("authorization") {
        Some(_) => next.run(request).await.into_response(),
        None => StatusCode::UNAUTHORIZED.into_response(),
    }
}