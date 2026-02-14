use axum::{Json, response::IntoResponse};
use serde_json::json;

pub async fn hello_world() -> impl IntoResponse {
    let json_response = json!({
        "status": "ok",
        "message": "Hello world"
    });
    Json(json_response)
}
