mod config;
mod handlers;
mod http;
mod lastfm;
mod router;

#[tokio::main]
async fn main() {
    let config::Config { addr, .. } = config::init();
    http::init();

    let app = router::init();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Server started successfully at {}", addr);
    axum::serve(listener, app).await.unwrap();
}
