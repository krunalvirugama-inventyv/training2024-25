use axum::extract::Path;

// Prority of the select path varibale best match of the path 
pub async fn path_variable(Path(id) : Path<i32>) -> String {
    id.to_string()
}

pub async fn hard_code_path()-> String{
    "You are Hard Code 15".to_string()
}