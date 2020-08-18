#![allow(dead_code)]
#![deny(warnings)]
#![warn(rust_2018_idioms)]

use crate::error::Error;
use crate::Rule;
use anyhow::Result;
use pest::error::Error as PestError;

// Absttact the formatting of errors away from the core logic inside the parser,
// so that the file is easier to read.
pub fn error_msg(error: PestError<Rule>) -> Result<super::Dom> {
    let message = error.renamed_rules(|rule| match *rule {
        Rule::EOI => "end of input".to_string(),
        Rule::doctype => "doctype element".to_string(),
        // TODO: Continue
        x => format!("{:?} ", x),
    });
    Err(Error::Parsing(message.to_string()).into())
}

