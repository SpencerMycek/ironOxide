#![allow(dead_code)]
#![deny(warnings)]
#![warn(rust_2018_idioms)]

use super::dom::{DomVariant, Dom};
use super::dom::node::Node;

mod oxide_display_text;
mod oxide_display_ncurses;

use oxide_display_text as text;
use oxide_display_ncurses as ncurses;


pub fn display(dom: &Dom, ncurses: bool) {
    if ncurses {
        ncurses::display(dom);
    } else { 
        text::display(dom);
    }
}

fn display_ncurses(_dom: &Dom) {
}

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
            return None
        }
        DomVariant::DocumentFragment => {
            return Some("/=====Document=Fragment=====/".to_string())
        }
        DomVariant::Empty => {
            return Some("/=====Empty=Document=====/".to_string())
        }
    };
}


