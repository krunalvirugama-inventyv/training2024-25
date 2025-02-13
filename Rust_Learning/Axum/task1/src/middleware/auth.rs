// use axum::{
//     body::Body,
//     extract::{Request, State},
//     http::StatusCode,
//     middleware::Next,
//     response::{IntoResponse, Response},
// };
// use axum_extra::headers::HeaderMap;
// use sqlx::MySqlPool;

// use crate::utils::responce::error_response;

// pub async fn auth(
//     State(pool): State<MySqlPool>,
//     headers: HeaderMap,
//     request: Request<Body>,
//     next: Next,
// ) -> Response {
//     if let Some(auth_header) = headers.get("Authorization") {
//         if let Ok(token) = auth_header.to_str() {
//             let query = "SELECT id FROM users WHERE token = ?";
//             if let Ok(Some(_)) = sqlx::query_scalar::<_, i32>(query)
//                 .bind(token) // Use the token directly
//                 .fetch_optional(&pool)
//                 .await
//             {
//                 return next.run(request).await.into_response();
//             }
//         }
//     }

//     error_response("UNAUTHORIZED", StatusCode::UNAUTHORIZED)
// }

use axum::{
    body::Body,
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use sqlx::MySqlPool;
use tower_cookies::Cookies;
use crate::utils::responce::error_response;

pub async fn auth(
    State(pool): State<MySqlPool>,
    cookies: Cookies, // Use tower_cookies::Cookies extractor
    request: Request<Body>,
    next: Next,
) -> Response {
    
    if let Some(cookie) = cookies.get("auth_token") {
        let token = cookie.value();

        let query = "SELECT id FROM users WHERE token = ?";
        if let Ok(Some(_)) = sqlx::query_scalar::<_, i32>(query)
            .bind(token)
            .fetch_optional(&pool)
            .await
        {
            return next.run(request).await.into_response();
        }
    }

    error_response("UNAUTHORIZED", StatusCode::UNAUTHORIZED)
}
