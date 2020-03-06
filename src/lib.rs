//#![deny(warnings)]
#![warn(rust_2018_idioms)]

use std::{env, str};

//use failure::Fail;
//#[macro_use] extern crate failure;

use hyper::{body::HttpBody as _1, Client};
use hyper_rustls::HttpsConnector;
//use tokio::io::{self, AsyncWriteExt as _};

mod html_dom;
use html_dom::{Dom};

// Type alias so as to DRY
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn run() -> Result<()> {
    // Some simple CLI args requirements...
    let url = match env::args().nth(1) {
        Some(url) => url,
        None => {
            println!("Usage: client <url>");
            return Ok(());
        }
    };

    let __body = http_get(&url).await?;

    //println!("{}", body);
    
    {
        let result = Dom::parse_document("<!DOCTYPE html> AHHHHHHAHAHAHAH");
        let dom = match result {
            Ok(dom) => dom,
            Err(e) => return Err(e),
        };
        println!("{}", dom.root_element().to_string(0));
    };

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

    println!("Response: {}", res.status());
    println!("Headers: {:#?}\n", res.headers());

    let mut body = String::from("");

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

