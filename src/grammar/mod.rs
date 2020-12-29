//! # Iron Oxide Grammar
//!
//! Defines the HTML parser using PEST

use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar/html.pest"]
pub struct HTMLParser;
