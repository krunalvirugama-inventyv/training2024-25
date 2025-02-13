use axum::{
    body::Body,
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};
use chrono::Utc;
use std::fs;
use std::io::Write;
use std::path::Path;

pub async fn log(req: Request<Body>, next: Next) -> Response {
    let uri = req.uri().to_string();
    let method = req.method().to_string();
    let timestamp = Utc::now().to_string();

    let log_entry = format!("[{timestamp}]  method={method} uri={uri}\n");

    println!("{log_entry}");

    if let Err(e) = append_to_file("logs/request.log", &log_entry) {
        eprintln!("Failed to write to log file: {}", e);
    }

    next.run(req).await.into_response()
}

fn append_to_file(file_path: &str, content: &str) -> std::io::Result<()> {
    if let Some(parent) = Path::new(file_path).parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;

    file.write_all(content.as_bytes())
}
