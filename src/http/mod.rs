//! # Iron Oxide HTTP Library
//!
//! Provides functions used to make various HTTP requests

use hyper::{body::HttpBody as _1, Client};
use hyper_rustls::HttpsConnector;

use anyhow::Result;

///Returns a [`Result`] containing the body of the http response, or the propegated error.
///Sends an HTTP get request to the provided URL
///The body is returned as a [`String`]
///
///[`Result`]: https://doc.rust-lang.org/std/result/
///[`String`]: https://doc.rust-lang.org/std/string/struct.String.html
pub async fn get(url: &str) -> Result<String> {
    let url = url.parse::<hyper::Uri>().unwrap();

    let https = HttpsConnector::with_native_roots();
    let client: Client<_, hyper::Body> = Client::builder().build(https);

    let mut res = client.get(url).await?;

    let mut body = String::new();
    while let Some(next) = res.data().await {
        let chunk = next?;
        body.push_str(&String::from_utf8_lossy(&chunk));
    }
    Ok(body)
}

