use std::collections::HashMap;

use axum::{Json, http::StatusCode, response::IntoResponse};
use reqwest::Client;
use serde_json::Value;
use serde_json::json;

use crate::config;

pub async fn fetch_similar_artists(artist: String) -> Result<impl IntoResponse, StatusCode> {
    let config::Config { lastfm_api_key, .. } = config::get();

    let params = HashMap::from([
        ("api_key", lastfm_api_key.as_str()),
        ("artist", artist.as_str()),
        ("method", "artist.getSimilar"),
        ("format", "json"),
        ("limit", "20"),
    ]);

    let response = Client::new()
        .get("https://ws.audioscrobbler.com/2.0/")
        .query(&params)
        .send()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?
        .json::<Value>()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;

    Ok(Json(json!({
        "status": "ok",
        "data": response
    })))
}
