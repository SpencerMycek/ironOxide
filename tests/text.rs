#![allow(dead_code)]
#![deny(warnings)]
#![warn(rust_2018_idioms)]

use iron_oxide::{Dom, Result};
use indoc::indoc;
use insta::assert_json_snapshot;

#[test]
fn it_can_parse_document_with_just_text() -> Result<()> {
    let html = "hello world";
    let dom = Dom::parse(html)?;
    assert_json_snapshot!(dom);
    Ok(())
}

#[test]
fn it_can_parse_document_with_text_and_line_breaks() -> Result<()> {
    let html = indoc!(
        r"
        hello world
        here's another line for you!
        The end
    "
    );
    let dom = Dom::parse(html)?;
    assert_json_snapshot!(dom);
    Ok(())
}

#[test]
fn it_can_parse_document_with_multiple_text_elements() -> Result<()> {
    let html = indoc!(
        r"
        hello world
        here's another line for you!
        <div/>
        The end
    "
    );
    let dom = Dom::parse(html)?;
    assert_json_snapshot!(dom);
    Ok(())
}
