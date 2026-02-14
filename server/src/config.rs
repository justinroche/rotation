use once_cell::sync::OnceCell;
use std::{env, net::SocketAddr};

#[derive(Debug)]
pub struct Config {
    pub addr: SocketAddr,
}

static CONFIG: OnceCell<Config> = OnceCell::new();

/// Initialize the global config. Call once from main.
pub fn init() -> &'static Config {
    dotenvy::dotenv().ok();

    let host = env::var("SERVER_HOST").expect("SERVER_HOST environment variable is required");
    let port = env::var("SERVER_PORT").expect("SERVER_PORT environment variable is required");
    let addr: SocketAddr =
        format!("{}:{}", host, port).parse().expect("SERVER_HOST or SERVER_PORT is invalid");

    CONFIG.set(Config { addr }).expect("Config already initialized");

    CONFIG.get().unwrap()
}

#[allow(dead_code)]
/// Get a reference to the global config. Panics if `init()` wasn't called.
pub fn get() -> &'static Config {
    CONFIG.get().expect("Config not initialized. Call config::init() in main.")
}
