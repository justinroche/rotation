use once_cell::sync::OnceCell;
use std::{env, net::SocketAddr};

pub const USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("CARGO_PKG_HOMEPAGE"),
    ")"
);

#[derive(Debug)]
pub struct Config {
    pub addr: SocketAddr,
    pub lastfm_api_key: String,
}

static CONFIG: OnceCell<Config> = OnceCell::new();

/// Initialize the global config.
pub fn init() -> &'static Config {
    dotenvy::dotenv().ok();

    let addr: SocketAddr = env::var("BIND_ADDR")
        .expect("BIND_ADDR environment variable is missing")
        .parse()
        .expect("BIND_ADDR is invalid");

    let lastfm_api_key =
        env::var("LASTFM_API_KEY").expect("LASTFM_API_KEY environment variable is missing");

    CONFIG.get_or_init(|| Config {
        addr,
        lastfm_api_key,
    })
}

/// Get a ref to the global config.
pub fn get() -> &'static Config {
    CONFIG.get().expect("Config not initialized.")
}
