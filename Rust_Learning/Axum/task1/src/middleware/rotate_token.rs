use axum::{
    body::Body,
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use sqlx::MySqlPool;
use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;

pub async fn rotate_token(
    State(pool): State<MySqlPool>,
    cookies: Cookies,
    request: Request<Body>,
    next: Next,
) -> Response {

    let old_token = cookies.get("auth_token").map(|c| c.value().to_string());

    let mut response = next.run(request).await;

    if old_token.is_none() || response.status() == StatusCode::UNAUTHORIZED {
        return response;
    }

    let old_token = old_token.unwrap();

    let new_token = Uuid::new_v4().to_string();

    let update_query = "UPDATE users SET token = ? WHERE token = ?";
   
    match sqlx::query(update_query)
        .bind(&new_token)
        .bind(&old_token)
        .execute(&pool)
        .await
    {
        Ok(_) => {
  
            let mut cookie = Cookie::new("auth_token", new_token.clone());
            cookie.set_path("/");
            cookie.set_secure(false);
            cookie.set_http_only(true);
            cookie.set_same_site(tower_cookies::cookie::SameSite::Strict);
            cookies.add(cookie);


        }
        Err(err) => {
            eprintln!("Failed to rotate token: {:?}", err);
        }
    }

    response
}
