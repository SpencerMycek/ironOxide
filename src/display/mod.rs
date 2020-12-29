//! # Iron Oxide Display
//!
//! Provides functions used to display web-content for `Iron Oxide`.
//! And is split into sub-modules depending on the requesed display type.

use super::dom::{DomVariant, Dom};
use super::dom::node::Node;

mod oxide_display_text;
mod oxide_display_ncurses;

use oxide_display_text as text;
use oxide_display_ncurses as ncurses;

/// Passes Display to control to the requested display type
pub fn display(dom: &Dom, ncurses: bool) {
    if ncurses {
        ncurses::display(dom);
    } else { 
        text::display(dom);
    }
}

/// Returns the Title of an html webpage from the Dom as a [`Option<String>`]
/// This is a general use function, available to all display types.
///
/// [`Option<String>`]: https://doc.rust-lang.org/std/option/
fn get_title(dom: &Dom) -> Option<String> {
    match dom.tree_type {
        DomVariant::Document => {
            let html = (&dom.children[0]).element().unwrap();
            for child in &html.children {
                if let Node::Element(head) = child {
                    if head.name.to_lowercase() == "head" {
                        for child in &head.children {
                            if let Node::Element(title) = child {
                                if title.name.to_lowercase() == "title" {
                                    return Some((&title.children[0])
                                                .text().unwrap().to_string())
                                }
                            }
                        }
                    }
                }
            }
            None
        }
        DomVariant::DocumentFragment => {
            Some("/=====Document=Fragment=====/".to_string())
        }
        DomVariant::Empty => {
            Some("/=====Empty=Document=====/".to_string())
        }
    }
}

/// Returns a `Vec<Node>` containing either the Body element of an HTML
/// Document or all the top-level nodes of the document fragment
fn get_visible_nodes(dom: &Dom) -> &Vec<Node> {
    let root: &Vec<Node> = match dom.tree_type {
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
    &root
}

