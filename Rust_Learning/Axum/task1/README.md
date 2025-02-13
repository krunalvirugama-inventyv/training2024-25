# Task 1: Employee Management With Super User using Axum

## Overview
This task aims to develop a simple Employee and User (Super Admin) Management System using the **Axum** web framework in Rust. The task is designed to help understand Axum concepts, including routing, middleware, database integration, and authentication. The database used for this project is **TiDB**.

## Objective
- **Employee Module:** Handle Employee data CRUD operations with authentication.
- **User Module:** Handle Super Admin (User) functionalities.
- **Middleware Implementation:**
  - **Authentication Middleware:** Validate user authentication token.
  - **Log Middleware:** Log request timestamp, method, and URL.
  - **Extract ID to Header Middleware:** Extract ID from URL for specific endpoints and attach it to the request header.

---

## Folder Structure
```
Axum/
├── task1/
│   ├── logs/
│   ├── src/
│   │   ├── db/
│   │   │   ├── connection.rs
│   │   │   ├── mod.rs
│   │   ├── handlers/
│   │   │   ├── employee.rs
│   │   │   ├── user.rs
│   │   │   ├── mod.rs
│   │   ├── middleware/
│   │   │   ├── auth.rs
│   │   │   ├── log.rs
│   │   │   ├── extract_id_to_header.rs
│   │   │   ├── rotate_token.rs
│   │   │   ├── mod.rs
│   │   ├── routes/
│   │   │   ├── employee.rs
│   │   │   ├── user.rs
│   │   │   ├── mod.rs
│   │   ├── utils/
│   │   │   ├── responce.rs
│   │   │   ├── mod.rs
│   │   ├── lib.rs
│   │   ├── main.rs
│   ├── .env
│   ├── .gitignore
│   ├── Cargo.toml
│   ├── Cargo.lock
```

---

## Routing

### Employee Routes (With Authentication Middleware)
| Route                        | Method | Description                                |
|------------------------------|--------|---------------------------------------------|
| `/employee/get_emp/{id}`     | GET    | Get an employee by ID (Uses Extract ID Middleware) |
| `/employee/del_emp/{id}`     | DELETE | Delete an employee by ID (Uses Extract ID Middleware) |
| `/employee/get_emps`         | GET    | Get all employees                          |
| `/employee/create_emp`       | POST   | Create a new employee                      |

### User (Super Admin) Routes
| Route                   | Method | Description                                |
|-------------------------|--------|---------------------------------------------|
| `user/login`            | GET    | User login, returns auth token as a cookie |
| `user/register_user`    | POST   | User Register as SuperUser                 |

---

## Middleware

### 1. **Authentication Middleware (auth.rs)**
- Validates the **auth_token** from the request cookies.
- Checks the token against the database.
- If valid, allows the request to proceed.
- If invalid, responds with `401 Unauthorized`.

### 2. **Log Middleware (log.rs)**
- Logs the **request timestamp**, **HTTP method**, and **request URL** for every request.

### 3. **Extract ID to Header Middleware (extract_id_to_header.rs)**
- Applies only to the `/employee/get_emp/{id}` and `/employee/del_emp/{id}` routes.
- Extracts the `id` from the URL path and adds it to the request header as `X-Employee-ID`.

### 4. **Token Rotation Middleware (rotate_token.rs)**
- After a successful request, generates a **new token**.
- Updates the token in the database.
- Sets the new token in the `auth_token` cookie in the response.

---

## Utils Module
- `responce.rs`: Contains utility functions:
  - `success_response`: Standardized success response generator.
  - `error_response`: Standardized error response generator.

These are useful for maintaining consistent responses throughout the system.

---

## Database Integration (TiDB)
- Connection is managed in `db/connection.rs`.
- Queries are performed using **sqlx**.

---

## Environment Variables
- `.env` file holds the database connection string and other secrets.

---

## Running the Project
1. Create a `.env` file with your TiDB credentials.
2. Run `cargo build` to compile the project.
3. Run `cargo run` to start the server.
4. Access the routes via `http://localhost:3001`.

---

## Key Learnings
- **Routing** in Axum.
- Building and using **Custom Middleware**.
- Handling **Database Operations** with TiDB and sqlx.
- Using **Cookies** for Authentication and Token Rotation.
- Extracting **Path Parameters** and Modifying **Request Headers**.

---






