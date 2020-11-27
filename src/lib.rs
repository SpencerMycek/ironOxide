//! # Iron Oxide
//!
//! This is the main running process of the `Iron Oxide` browser and
//! orchestrates all of the moving pieces

#![deny(warnings)]
#![warn(rust_2018_idioms)]

use std::{str};
use hyper::{body::HttpBody as _1, Client};
use hyper_rustls::HttpsConnector;

mod dom;
mod error;
mod grammar;
mod display;
pub mod cli;

use grammar::Rule;

pub use crate::dom::Dom;
pub use crate::error::Error;
pub use crate::cli::Opts;
pub use anyhow::Result;

// Type alias so as to DRY
//pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Executes as the main process for `Iron Oxide`
pub async fn run(args: cli::Opts) -> Result<()> {
    let url = args.url;

    let body = http_get(&url).await?;
    let dom = Dom::parse(&body)?;

    display::display(&dom, args.ncurses);

    Ok(())
}

///Returns a [`Result`] containing the body of the http response, or the propegated error.
///Sends an HTTP get request to the provided URL
///The body is returned as a [`String`]
///
///[`Result`]: https://doc.rust-lang.org/std/result/
///[`String`]: https://doc.rust-lang.org/std/string/struct.String.html
async fn http_get(url:& str) -> Result<String> {
    let url = url.parse::<hyper::Uri>().unwrap();
   
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let mut res = client.get(url).await?;

    //println!("Response: {}", res.status());
    //println!("Headers: {:#?}\n", res.headers());

    let mut body = String::new();

    // Stream the body, writing each chunk to stdout as we get it
    // (instead of buffering and printing at the end).
    // Actually saves data into a String to return
    while let Some(next) = res.data().await {
        let chunk = next?;
        body.push_str(&String::from_utf8_lossy(&chunk));
        //io::stdout().write_all(&chunk).await?;
    }

    Ok(body)

}

