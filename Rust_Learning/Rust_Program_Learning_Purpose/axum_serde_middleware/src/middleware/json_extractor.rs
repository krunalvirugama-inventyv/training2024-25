use axum::{
    body::Bytes,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    Json,
};

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub mo_no: String,
}

pub async fn json_extractor_middleware<B>(
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let (parts, body) = request.into_parts();

    let bytes = hyper::body::to_bytes(body)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Parse JSON
    let user: User = serde_json::from_slice(&bytes).map_err(|_| StatusCode::BAD_REQUEST)?;

    // Attach user data back to request extensions
    let mut req = Request::from_parts(parts, Bytes::from(bytes));
    req.extensions_mut().insert(user);

    // Proceed to the next middleware/handler
    Ok(next.run(req).await)
}


