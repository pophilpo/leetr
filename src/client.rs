use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Client;
use std::error::Error;

pub fn generate_client() -> Result<Client, Box<dyn Error>> {
    let mut headers = HeaderMap::new();

    let client = Client::builder()
        .default_headers(headers)
        .redirect(reqwest::redirect::Policy::limited(10))
        .build()?;

    Ok(client)
}
