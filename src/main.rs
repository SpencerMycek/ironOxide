//#![deny(warnings)]
#![warn(rust_2018_idioms)]

use std::{env, str};

use hyper::{body::HttpBody as _, Client};
use tokio::io::{self, AsyncWriteExt as _};

use failure::Fail;
#[macro_use] extern crate failure;

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;


#[derive(Fail, Debug)]
#[fail(display = "There is an error: {}.", _0)]
struct HttpsError(String);


#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

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
    let client = Client::new();
    
    // HTTPS requires picking a TLS implementation, so give a better
    // warning if the user tries to request an 'https' URL.
    let url = url.parse::<hyper::Uri>().unwrap();
    if url.scheme_str() != Some("http") {
        return Result::Err(Box::new(HttpsError("This example only works with 'http' URLs.".into()).compat()));
    }

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

