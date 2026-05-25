use crate::config;
use once_cell::sync::OnceCell;
use reqwest::{
    Client,
    header::{HeaderMap, HeaderValue, USER_AGENT},
};

static CLIENT: OnceCell<Client> = OnceCell::new();

pub fn init() -> &'static Client {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static(config::USER_AGENT));

    CLIENT.get_or_init(|| {
        Client::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to build HTTP client")
    })
}

pub fn get() -> &'static Client {
    CLIENT.get().expect("HTTP client not initialized.")
}
