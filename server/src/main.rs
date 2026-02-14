use axum::{Router, routing::get};

mod config;
mod handlers;

use handlers::hello_world;

#[tokio::main]
async fn main() {
    let config::Config { addr } = config::init();

    let app = Router::new().route("/api", get(hello_world));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Server started successfully at {}", addr);
    axum::serve(listener, app).await.unwrap();
}
