//! # Iron Oxide Node
//!
//! Defines html nodes that will be inside of a DOM tree

use std::convert::Into;

use super::element::Element;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Node {
    Text(String),
    Element(Element),
    Comment(String),
}

impl Into<Option<Element>> for Node {
    fn into(self) -> Option<Element> {
        match self {
            Node::Element(i) => Some(i),
            _ => None,
        }
    }
}

impl Node {
    pub fn element(&self) -> Option<&Element> {
        match self {
            Node::Element(e) => Some(e),
            _ => None,
        }
    }
    pub fn text(&self) -> Option<&str> {
        match self {
            Node::Text(t) => Some(t),
            _ => None,
        }
    }
}

