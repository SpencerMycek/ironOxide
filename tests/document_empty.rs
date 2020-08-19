#![allow(dead_code)]
#![deny(warnings)]
#![warn(rust_2018_idioms)]

use iron_oxide::{Dom, Result};
use insta::assert_json_snapshot;

#[test]
fn it_can_parse_empty_document() -> Result<()> {
    let html = "";
    let dom = Dom::parse(html)?;
    assert_json_snapshot!(dom);
    Ok(())
}

