use crate::utils::responce::{error_response, success_response};
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::Response,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, MySqlPool};

#[derive(Debug, Deserialize)]
pub struct Employee {
    employee_name: String,
    salary: u32,
}

#[derive(Debug, Serialize, FromRow)]
pub struct EmployeeResponse {
    pub id: i32,
    pub employee_name: String,
    pub salary: i32,
}

pub async fn create_employee(
    State(pool): State<MySqlPool>,
    Json(employee): Json<Employee>,
) -> Response {
    let query = "INSERT INTO employees (employee_name, salary) VALUES (?, ?)";

    match sqlx::query(query)
        .bind(&employee.employee_name)
        .bind(&employee.salary)
        .execute(&pool)
        .await
    {
        Ok(_) => success_response(
            "Employee created successfully",
            Vec::<serde_json::Value>::new(),
            StatusCode::CREATED,
        ),
        Err(err) => {
            eprintln!("Database error: {:?}", err);
            error_response(
                "Failed to create employee",
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        }
    }
}


pub async fn get_employees(State(pool): State<MySqlPool>) -> Response {
    let query = "SELECT id, employee_name, salary FROM employees";

    match sqlx::query_as::<_, EmployeeResponse>(query)
        .fetch_all(&pool)
        .await
    {
        Ok(employees) => {
            success_response("Employees fetched successfully", employees, StatusCode::OK)
        }
        Err(err) => {
            eprintln!("Database error: {:?}", err);
            error_response(
                "Failed to fetch employees",
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        }
    }
}

pub async fn get_employee_by_id(State(pool): State<MySqlPool>, headers: HeaderMap) -> Response {
    let id = match headers.get("x-employee-id") {
        Some(value) => match value.to_str().ok().and_then(|v| v.parse::<i32>().ok()) {
            Some(id) => id,
            None => {
              return error_response(
                    "Invalid Employee ID in header",
                    StatusCode::BAD_REQUEST,
                );
            }
        },
        None => {
            return error_response(
                "Employee ID not found in header",
                StatusCode::BAD_REQUEST,
            );  
        }
    };

    let query = "SELECT id, employee_name, salary FROM employees WHERE id = ? ";

    match sqlx::query_as::<_, EmployeeResponse>(query)
        .bind(id)
        .fetch_optional(&pool)
        .await
    {
        Ok(Some(employee)) => {
            success_response("Employee fetched successfully", employee, StatusCode::OK)
        }

        Ok(None) => success_response("Employee Not Found ",   Vec::<serde_json::Value>::new(),  StatusCode::NOT_FOUND),
        Err(err) => {
            eprintln!("Database error: {:?}", err);
            error_response(
                "Failed to fetch employee",
                StatusCode::INTERNAL_SERVER_ERROR,
            ) 
        }
    }
}

pub async fn delete_employee_by_id(State(pool): State<MySqlPool>, headers: HeaderMap) -> Response {
    let id = match headers.get("x-employee-id") {
        Some(value) => match value.to_str().ok().and_then(|v| v.parse::<i32>().ok()) {
            Some(id) => id,
            None => {
                return error_response(
                    "Invalid Employee ID in header",
                    StatusCode::BAD_REQUEST,
                );
            }
        },
        None => {
            return error_response(
                "Employee ID not found in header",
                StatusCode::BAD_REQUEST,
            );  
        }
    };

    let query = "DELETE FROM employees WHERE id = ? ";

    match sqlx::query(query).bind(id).execute(&pool).await {
        Ok(result) => {
            if result.rows_affected() > 0 {
                success_response("Employee Deleted successfully", Vec::<serde_json::Value>::new(), StatusCode::OK)
                
            } else {
                success_response("Employee Not Found",   Vec::<serde_json::Value>::new(),  StatusCode::NOT_FOUND)
            }
        }
        Err(err) => {
            eprintln!("Database error: {:?}", err);
            error_response(
                "Failed to fetch employee",
                StatusCode::INTERNAL_SERVER_ERROR,
            ) 
        }
    }
}
