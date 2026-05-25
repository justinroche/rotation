use axum::{Json, extract::Query, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use serde_json::{Value, json};

use crate::lastfm;

#[derive(Deserialize)]
pub struct ArtistMetadataParams {
    artist_name: String,
}

#[derive(Deserialize)]
pub struct ArtistFetchParams {
    artist_mbid: String,
}

fn ok_json(data: Value) -> impl IntoResponse {
    Json(json!({ "status": "ok", "data": data }))
}

pub async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "ok"
    }))
}

pub async fn fetch_metadata(
    Query(params): Query<ArtistMetadataParams>,
) -> Result<impl IntoResponse, StatusCode> {
    lastfm::fetch_metadata(params.artist_name)
        .await
        .map(ok_json)
}

pub async fn fetch_similar_artists(
    Query(params): Query<ArtistFetchParams>,
) -> Result<impl IntoResponse, StatusCode> {
    lastfm::fetch_similar_artists(params.artist_mbid)
        .await
        .map(ok_json)
}

pub async fn fetch_top_albums(
    Query(params): Query<ArtistFetchParams>,
) -> Result<impl IntoResponse, StatusCode> {
    lastfm::fetch_top_albums(params.artist_mbid)
        .await
        .map(ok_json)
}
