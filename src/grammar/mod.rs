//! # Iron Oxide Grammar
//!
//! Defines the HTML parser using PEST

#![allow(dead_code)]
#![deny(warnings)]
#![warn(rust_2018_idioms)]

use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar/html.pest"]
pub struct HTMLParser;
