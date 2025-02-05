use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use serde::{Deserialize, Serialize};
use std::{fs::OpenOptions, io::Write, sync::Arc};
use tokio::sync::Mutex;
use serde_json;
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub mo_no: String,
}

#[derive(Clone)]
pub struct AppState {
    pub data: Arc<Mutex<Vec<User>>>,
}

// Route Handler
pub async fn create_user(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let mut data = state.data.lock().await;

    // Append user to in-memory list
    data.push(user.clone());

    // Save to JSON file
    match save_to_json_file(&user) {
        Ok(_) => (StatusCode::OK, "User created successfully"),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to save user"),
    }
}

// Save function
fn save_to_json_file(user: &User) -> std::io::Result<()> {
    let file_path = "users.json";

    let mut file = OpenOptions::new().create(true).append(true).open(file_path)?;

    let json_data = serde_json::to_string(user)? + "\n";

    file.write_all(json_data.as_bytes())?;

    Ok(())
}
