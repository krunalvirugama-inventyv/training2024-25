use axum::{
    body::Bytes,
    http::Request,
    middleware::Next,
    response::Response,
};
use serde_json::Value;
use std::sync::Arc;

pub async fn json_extractor<B>(req: Request<B>, next: Next<B>) -> Result<Response, axum::Error> 
where
    B: axum::body::HttpBody + Send + Sync + 'static,
    B::Data: Send,
    B::Error: std::fmt::Debug,
{
    let (parts, body) = req.into_parts();
    let bytes = hyper::body::to_bytes(body).await.unwrap_or_else(|_| Bytes::new());
    
    if let Ok(json) = serde_json::from_slice::<Value>(&bytes) {
        println!("Extracted JSON: {:?}", json);
    }

    let req = Request::from_parts(parts, Bytes::from(bytes));
    Ok(next.run(req).await)
}
