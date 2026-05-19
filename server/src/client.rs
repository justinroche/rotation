use std::collections::HashMap;

use axum::{Json, http::StatusCode, response::IntoResponse};
use once_cell::sync::OnceCell;
use reqwest::{
    Client,
    header::{HeaderMap, HeaderValue, USER_AGENT},
};
use serde_json::{Value, json};

use crate::config;

static CLIENT: OnceCell<Client> = OnceCell::new();

/// Initialize the global client.
pub fn init() -> &'static Client {
    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static(config::USER_AGENT));

    CLIENT.get_or_init(|| {
        Client::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to build HTTP client")
    })
}

/// Get a ref to the global client.
fn get() -> &'static Client {
    CLIENT.get().expect("HTTP client not initialized.")
}

pub async fn fetch_similar_artists(artist: String) -> Result<impl IntoResponse, StatusCode> {
    let config::Config { lastfm_api_key, .. } = config::get();

    let params = HashMap::from([
        ("api_key", lastfm_api_key.as_str()),
        ("artist", artist.as_str()),
        ("method", "artist.getSimilar"),
        ("format", "json"),
        ("limit", "20"),
    ]);

    let response = get()
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
