use std::collections::HashMap;

use axum::http::StatusCode;
use serde_json::Value;

use crate::{config, http};

fn base_params(method: &str) -> HashMap<&str, &str> {
    HashMap::from([
        ("api_key", config::get().lastfm_api_key.as_str()),
        ("method", method),
        ("format", "json"),
    ])
}

async fn get(params: HashMap<&str, &str>) -> Result<Value, StatusCode> {
    http::get()
        .get(config::LASTFM_URL)
        .query(&params)
        .send()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?
        .json::<Value>()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)
}

pub async fn fetch_metadata(artist_name: String) -> Result<Value, StatusCode> {
    let mut params = base_params("artist.getInfo");
    params.insert("artist", artist_name.as_str());

    get(params).await
}

pub async fn fetch_similar_artists(artist_mbid: String) -> Result<Value, StatusCode> {
    let mut params = base_params("artist.getSimilar");
    params.insert("mbid", artist_mbid.as_str());
    params.insert("limit", "20");

    get(params).await
}

pub async fn fetch_top_albums(artist_mbid: String) -> Result<Value, StatusCode> {
    let mut params = base_params("artist.getTopAlbums");
    params.insert("mbid", artist_mbid.as_str());
    params.insert("limit", "20");

    get(params).await
}
