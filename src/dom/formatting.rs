//! # Iron Oxide Dom Error Formatting
//!
//! Makes the Errors from PEST more user friendly

use crate::error::Error;
use crate::Rule;
use anyhow::Result;
use pest::error::Error as PestError;

/// Abstracts the formatting of errors away from the core logic inside the parser,
/// so that the file is easier to read.
pub fn error_msg(error: PestError<Rule>) -> Result<super::Dom> {
    let message = error.renamed_rules(|rule| match *rule {
        Rule::EOI => "end of input".to_string(),
        Rule::doctype => "doctype element".to_string(),
        Rule::attr_key => "attribute key".to_string(),
        // TODO: Continue
        x => format!("{:?} ", x),
    });
    Err(Error::Parsing(message.to_string()).into())
}

