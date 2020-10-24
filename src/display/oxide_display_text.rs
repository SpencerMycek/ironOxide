//! # Iron Oxide Text Display
//!
//! Displays the web-content, typucally an html webpage, by printing raw text

#![allow(dead_code)]
#![deny(warnings)]
#![warn(rust_2018_idioms)]

use super::super::dom::Dom;
use super::super::dom::node::Node;
use super::super::dom::element::HIDDEN_TAGS;
use super::{get_title, get_visible_nodes};

/// Displays the provided DOM using raw text
pub fn display(dom: &Dom) {
    //println!("{}", dom.to_json_pretty().expect("JSON Print failed"));
    if let Some(s) = get_title(dom) {
        println!("/====={}=====/", s);
    } else {
        println!("/=====No=Title=====/");
    }
    let mut text = "".to_string();
    println!("{}", text);
    let nodes = get_visible_nodes(&dom);
    get_text(&mut text, nodes);
    println!("{}", text);
}

/// Builds a String buffer with all of the text in an element, 
/// recurses into the element and all children
fn get_text(buf: &mut String, nodes: &Vec<Node>) {
    for node in nodes {
        match node {
            Node::Text(s) => {
                buf.push_str(&s);
                buf.push('\n');
            },
            Node::Element(el) => {
                if !(HIDDEN_TAGS.iter().any(|&i| i == el.name.to_lowercase())) {
                    get_text(buf, &el.children);
                }
            },
            Node::Comment(_) => {},
        }
    }
}

