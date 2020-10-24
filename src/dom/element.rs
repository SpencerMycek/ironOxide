//! # Iron Oxide Element
//!
//! Defines html elements that will be inside of a DOM

#![deny(warnings)]
#![warn(rust_2018_idioms)]

use super::node::Node;
use anyhow::Result;
use serde::{Serialize, Serializer};
use std::collections::{BTreeMap, HashMap};
use std::default::Default;

/// Normal: `<div></div>` or Void: `<meta/>` and `<meta>`
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
// TODO: Not fully aligned with https://html.spec.whatwg.org/multipage/syntax.html#elements-2
pub enum ElementVariant {
    // A normal element can have children
    Normal,
    // A void element can't have children
    Void,
}

/// A list of identifiers of non-visible element names.
/// i.e. "style", "script"
pub static HIDDEN_TAGS: [&'static str; 2] = ["style", "script"];

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Element {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    pub name: String,

    pub variant: ElementVariant,

    #[serde(skip_serializing_if = "HashMap::is_empty")]
    #[serde(serialize_with = "ordered_map")]
    pub attributes: HashMap<String, Option<String>>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub classes: Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<Node>,
}

impl Default for Element {
    fn default() -> Self {
        Self {
            id: None,
            name: "".to_string(),
            variant: ElementVariant::Void,
            classes: vec![],
            attributes: HashMap::new(),
            children: vec![],
        }
    }
}

fn ordered_map<S: Serializer>(value: &HashMap<String, Option<String>>,serializer: S,) -> Result<S::Ok, S::Error> {
    let ordered: BTreeMap<_, _> = value.iter().collect();
    ordered.serialize(serializer)
}

