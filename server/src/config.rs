use once_cell::sync::OnceCell;
use std::{env, net::SocketAddr};

#[derive(Debug)]
pub struct Config {
    pub addr: SocketAddr,
    pub lastfm_api_key: String,
}

static CONFIG: OnceCell<Config> = OnceCell::new();

/// Initialize the global config. Call once from main.
pub fn init() -> &'static Config {
    dotenvy::dotenv().ok();

    let addr: SocketAddr = env::var("BIND_ADDR")
        .expect("BIND_ADDR environment variable is missing")
        .parse()
        .expect("BIND_ADDR is invalid");

    let lastfm_api_key =
        env::var("LASTFM_API_KEY").expect("LASTFM_API_KEY environment variable is missing");

    CONFIG
        .set(Config {
            addr,
            lastfm_api_key,
        })
        .expect("Config already initialized");

    CONFIG.get().unwrap()
}

/// Get a reference to the global config. Panics if `init()` wasn't called.
pub fn get() -> &'static Config {
    CONFIG
        .get()
        .expect("Config not initialized. Call config::init() in main.")
}
