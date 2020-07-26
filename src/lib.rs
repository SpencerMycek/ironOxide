#![allow(dead_code)]
#![deny(warnings)]
#![warn(rust_2018_idioms)]

use std::{env, str};
use hyper::{body::HttpBody as _1, Client};
use hyper_rustls::HttpsConnector;

use pest;
//#[macro_use] extern crate pest_derive;

mod dom;
mod grammar;

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

    let body = http_get(&url).await?;

    println!("{}", body);
   /* 
    *let mut text1 = dom::text("Hello, World".to_string());
    *let text2 = dom::text("Hello, 2!".to_string());
    *text1.children.push(text2);
    *let text2 = dom::text("Hello, 2!".to_string());
    *let comment = dom::comment("<!--Comment!-->".to_string());
    *let mut attrs = dom::AttrMap::new();
    *attrs.insert("Attr 1".to_string(), "Value 1".to_string());
    *attrs.insert("Attr 2".to_string(), "Value 2".to_string());
    *attrs.insert("Attr 3".to_string(), "Value 3".to_string());
    *let element = dom::elem("Elem1".to_string(), attrs, vec![text1, comment, text2]);
    *dom::print_dom(&element);
    */
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

