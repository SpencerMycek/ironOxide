#![allow(dead_code)]
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

pub use crate::dom::element::{Element, ElementVariant};
pub use crate::dom::node::Node;
pub use crate::dom::Dom;
pub use crate::dom::DomVariant;
pub use crate::error::Error;
pub use crate::cli::Opts;
pub use anyhow::Result;

// Type alias so as to DRY
//pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn run(args: cli::Opts) -> Result<()> {
    let url = args.url;

    let body = http_get(&url).await?;
    let dom = Dom::parse(&body)?;

    println!("{}", dom.to_json_pretty()?);

    display::draw();

    Ok(())
}


/*
 * Function: http_get
 * -------------------
 * async Function, Makes an HTTP get request to the given url
 *  and returns a Result wrapped String
 *
 * url:     The url to request
 *
 * return:  Result containing Error or HTTP Response body
 */
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

