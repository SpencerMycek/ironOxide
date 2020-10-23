//! # Iron Oxide Text Display
//!
//! Displays the web-content, typucally an html webpage, by printing raw text

#![allow(dead_code)]
#![deny(warnings)]
#![warn(rust_2018_idioms)]

use super::super::dom::{DomVariant, Dom};
use super::super::dom::node::Node;
use super::get_title;

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
    let root = match dom.tree_type {
        DomVariant::Document => {
            let mut root = (&dom.children[0]).element().unwrap();
            for child in &root.children {
                if let Node::Element(el) = child {
                    if el.name.to_lowercase() == "body" {
                        root = el;
                    }
                }
            }
            &root.children
        },
        _ => { &dom.children },
    };
    get_text(&mut text, root);
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
            Node::Element(el) => {get_text(buf, &el.children);},
            Node::Comment(_) => {},
        }
    }
}

