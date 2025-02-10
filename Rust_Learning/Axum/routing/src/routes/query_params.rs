use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};

// Deseralize means Json into Object - Entry Time ex
// Serialize measn Object into Json - Exit Time ex
#[derive(Serialize, Deserialize)]
pub struct QueryParams {
    message: String,
    id: u32,
}

pub async fn query_params(Query(body): Query<QueryParams>) -> Json<QueryParams> {
    Json(body)
}
