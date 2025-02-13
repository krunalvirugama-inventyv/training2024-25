use axum::{extract::State, http::StatusCode, response::Response, Json};
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::Deserialize;
use sqlx::MySqlPool;
use uuid::Uuid;
use tower_cookies::{Cookies,Cookie};

use crate::utils::responce::{error_response, success_response};

#[derive(Debug, Deserialize)]
pub struct User {
    name: String,
    email: String,
    password: String,
}

#[derive(Deserialize, Debug)]
pub struct UserLogin {
    email: String,
    password: String,
}

pub async fn register_user(
    State(pool): State<MySqlPool>,
    Json(user): Json<User>,
) -> Response {
    println!("{user:?}");
    let query = "INSERT INTO users (name, email, password) VALUES (?, ?, ?)";

    let hashed_password = match hash(&user.password, DEFAULT_COST) {
        Ok(hash) => hash,
        Err(err) => {
            eprintln!("Password hashing error: {:?}", err);

            return error_response(
                "Error hashing password",
                StatusCode::INTERNAL_SERVER_ERROR,
            );
        }       
    };

    match sqlx::query(query)
        .bind(&user.name)
        .bind(&user.email)
        .bind(&hashed_password)
        .execute(&pool)
        .await
    {
        Ok(_) =>   success_response("User registered successfully", Vec::<serde_json::Value>::new(), StatusCode::OK) ,
        Err(err) => {
            eprintln!("Database error: {:?}", err);
           
            error_response(
                "User Not registered successfully ",
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        }
    }
}

pub async fn login(
    State(pool): State<MySqlPool>,
    cookies : Cookies,
    Json(credentials): Json<UserLogin>,
) -> Response {
    println!("{credentials:?}");


    let hashed_password = match hash(&credentials.password, DEFAULT_COST) {
        Ok(hash) => hash,
        Err(err) => {
            eprintln!("Password hashing error: {:?}", err);
            cookies.remove(Cookie::named("auth_token"));
            return error_response(
                "Error hashing password",
                StatusCode::INTERNAL_SERVER_ERROR,
            );
        }
    };

    let query = "SELECT password FROM users WHERE email = ?";

    match sqlx::query_scalar::<_, String>(&query)
        .bind(&credentials.email)
        .fetch_one(&pool)
        .await
    {
        Ok(hashed_password) => {
            // Verify password
            if verify(&credentials.password, &hashed_password).unwrap_or(false) {
                let token = Uuid::new_v4().to_string();
                let update_query = "UPDATE users SET token = ? WHERE email = ?";

                if let Err(err) = sqlx::query(&update_query)
                    .bind(&token)
                    .bind(&credentials.email)
                    .execute(&pool)
                    .await
                {
                    eprintln!("Failed to store token: {:?}", err);
                    cookies.remove(Cookie::named("auth_token"));
                    return error_response(
                        "Error storing token",
                        StatusCode::INTERNAL_SERVER_ERROR,
                    );
                }


                let mut cookie = Cookie::new("auth_token", token.clone());
                cookie.set_path("/");
                cookie.set_secure(false);
                cookie.set_http_only(true);
                cookie.set_same_site(tower_cookies::cookie::SameSite::Strict);
                cookies.add(cookie);
              
                success_response("Login successful", Vec::<serde_json::Value>::new(), StatusCode::OK)
            } else {
                
                cookies.remove(Cookie::named("auth_token"));
                return error_response(
                    "Invalid credentials",
                    StatusCode::UNAUTHORIZED
                )
            }
        }
        Err(_) =>{
            cookies.remove(Cookie::named("auth_token"));
             error_response(
            "Invalid credentials",
            StatusCode::UNAUTHORIZED
        )},
    }
}
