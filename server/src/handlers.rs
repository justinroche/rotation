use axum::{Json, extract::Query, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use serde_json::json;

use crate::client;

#[derive(Deserialize)]
pub struct SimilarArtistParams {
    artist: String,
}

pub async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "ok"
    }))
}

pub async fn fetch_similar_artists(
    Query(params): Query<SimilarArtistParams>,
) -> Result<impl IntoResponse, StatusCode> {
    client::fetch_similar_artists(params.artist).await
}
