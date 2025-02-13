use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

use super::read_middleware_custom_header::HeaderMessage;

pub async fn set_middleware_custom_header(
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // get headers
    let headers = request.headers();

    // Featch message inside headers
    let message = headers
        .get("x-message")
        .ok_or_else(|| StatusCode::BAD_REQUEST)?;

    // convert into string
    let message = message
        .to_str()
        .map_err(|_error| StatusCode::BAD_REQUEST)?
        .to_owned();

    // extensions for the add a message in the header
    let extensions = request.extensions_mut();

    // insert the value 
    extensions.insert(HeaderMessage(message));

    // pass the next
    Ok(next.run(request).await)
}
