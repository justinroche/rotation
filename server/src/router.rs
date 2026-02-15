use axum::{Router, routing::get};

use crate::handlers;

pub fn init() -> Router {
    Router::new().route("/health", get(handlers::health_check))
}
