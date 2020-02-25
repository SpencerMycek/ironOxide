#![deny(warnings)]
#![warn(rust_2018_idioms)]

use std::{env, str};

use failure::Fail;
#[macro_use] extern crate failure;

use hyper::{body::HttpBody as _1, Client};
use hyper_rustls::HttpsConnector;
//use tokio::io::{self, AsyncWriteExt as _};

// Type alias so as to DRY
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Fail, Debug)]
#[fail(display = "There is an error: {}.", _0)]
struct HttpsError(String);


pub async fn run() -> Result<()> {
    // Some simple CLI args requirements...
    let url = match env::args().nth(1) {
        Some(url) => url,
        None => {
            println!("Usage: client <url>");
            return Ok(());
        }
    };

    let body = fetch_url(&url).await?;

    println!("{}", body);

    Ok(())
}


async fn fetch_url(url:& str) -> Result<String> {
    let url = url.parse::<hyper::Uri>().unwrap();
   
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let mut res = client.get(url).await?;

    println!("Response: {}", res.status());
    println!("Headers: {:#?}\n", res.headers());

    let mut body = String::from("");

    // Stream the body, writing each chunk to stdout as we get it
    // (instead of buffering and printing at the end).
    while let Some(next) = res.data().await {
        let chunk = next?;
        body.push_str(str::from_utf8(&chunk)?);
        //io::stdout().write_all(&chunk).await?;
    }

    Ok(body)

}

