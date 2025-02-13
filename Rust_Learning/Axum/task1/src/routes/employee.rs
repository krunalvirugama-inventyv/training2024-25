use axum::{middleware, routing::{delete, get, post}, Router};
use sqlx::MySqlPool;
use crate::{handlers::employee::{create_employee, delete_employee_by_id, get_employee_by_id, get_employees}, middleware::rotate_token::rotate_token};
use crate::middleware::auth::auth;


use crate::middleware::extract_id_to_header::extract_id_to_header;

pub fn routes(pool: MySqlPool) -> Router<MySqlPool>{
    Router::new()
    .route("/get_emp/{id}", get(get_employee_by_id))
    .route("/del_emp/{id}", delete(delete_employee_by_id))
    .layer(middleware::from_fn(extract_id_to_header))
    .route("/get_emps", get(get_employees))
    .route("/create_emp", post(create_employee))
    .layer(middleware::from_fn_with_state(pool.clone(), auth))
    .layer(middleware::from_fn_with_state(pool, rotate_token))
}

