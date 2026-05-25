use axum::{Router, routing::get};

use crate::handlers;

pub fn init() -> Router {
    Router::new()
        .route("/health", get(handlers::health_check))
        .route("/artist/metadata", get(handlers::fetch_metadata))
        .route("/artist/similar", get(handlers::fetch_similar_artists))
        .route("/artist/albums", get(handlers::fetch_top_albums))
}
