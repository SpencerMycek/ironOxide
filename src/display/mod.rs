#![allow(dead_code)]
#![deny(warnings)]
#![warn(rust_2018_idioms)]

use std::default::Default;

use rustbox::{Color, RustBox, Key};
use super::dom::{DomVariant, Dom};
use super::dom::node::Node;

pub fn display(dom: &Dom, ncurses: bool) {
    if ncurses {
        display_ncurses(dom);
    } else { 
        display_text(dom);
    }
}

fn display_ncurses(_dom: &Dom) {
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    rustbox.print(1, 1, rustbox::RB_BOLD, Color::White, Color::Default, "Hello, World!");
    rustbox.print(1, 3, rustbox::RB_BOLD, Color::White, Color::Default,
                  "Press q to quit.");
    rustbox.present();
    loop {
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Char('q') => {break;}
                    _ => {}
                }
            },
            Err(e) => panic!("{}", e),
            _ => { }
        }
    }
}

fn display_text(dom: &Dom) {
    //println!("{}", dom.to_json_pretty().expect("JSON Print failed"));
    if let Some(s) = get_title(dom) {
        println!("/====={}=====/", s);
    } else {
        println!("/=====No=Title=====/");
    }
    let mut text = "".to_string();
    text.push_str("Test");
    println!("{}", text);
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


