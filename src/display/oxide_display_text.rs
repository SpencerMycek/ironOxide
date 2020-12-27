//! # Iron Oxide Text Display
//!
//! Displays the web-content, typucally an html webpage, by printing raw text

#![allow(dead_code)]
#![deny(warnings)]
#![warn(rust_2018_idioms)]

use super::super::dom::Dom;
use super::super::dom::node::Node;
use super::super::dom::element::{Element, HIDDEN_TAGS};
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
            },
            Node::Element(el) => {
                if !(HIDDEN_TAGS.iter().any(|&i| i == el.name.to_lowercase())) {
                    //get_text(buf, &el.children);
                    display_element_delegate(buf, el);
                }
            },
            Node::Comment(_) => {},
        }
    }
}

/// Delegates element rendering based on element name
fn display_element_delegate(buf: &mut String, element: &Element) {
    let el_name: &str = &element.name.to_lowercase();
    match &el_name[..] {
        "a" => display_element_anchor(buf, element),
        "article" => display_element_div(buf, element),
        "br" => buf.push_str("\n"),
        "button" => buf.push_str("{Button elements not yet supported}"),
        "center" => display_element_div(buf, element),
        "div" => display_element_div(buf, element),
        "em" => display_element_augment_text(buf, element, "*"),
        "figure" => buf.push_str("{Figure elements not yet supported}"),
        "footer" => display_element_div(buf, element),
        "form" => buf.push_str("{Form elements not yet supported}"),
        "h1" => display_element_heading(buf, &element.children[0], 1),
        "h2" => display_element_heading(buf, &element.children[0], 2),
        "h3" => display_element_heading(buf, &element.children[0], 3),
        "h4" => display_element_heading(buf, &element.children[0], 4),
        "h5" => display_element_heading(buf, &element.children[0], 5),
        "h6" => display_element_heading(buf, &element.children[0], 6),
        "header" => display_element_div(buf, element),
        "hr" => buf.push_str("\n----\n"),
        "iframe" => buf.push_str("{Iframes not yet supported}"),
        "img" => buf.push_str("{Img elements not yet supported}"),
        "input" => buf.push_str("{Input elements not yet supported}"),
        "main" => display_element_div(buf, element),
        "nav" => display_element_nav(buf, element),
        "noscript" => display_element_div(buf, element),
        "ol" => display_element_list_ordered(buf, element),
        "p" => display_element_paragraph(buf, element),
        "picture" => buf.push_str("{Picture elements not yet supported}"),
        "section" => display_element_div(buf, element),
        "select" => {},
        "span" => display_element_div(buf, element),
        "strong" => display_element_augment_text(buf, element, "**"),
        "sub" => get_text(buf, &element.children),
        "sup" => get_text(buf, &element.children),
        "svg" => {},
        "table" => buf.push_str("{Table elements not yet supported}"),
        "ul" => display_element_list_unordered(buf, element),
        "video" => buf.push_str("{Video elements not yet supported}"),
        _ => {
            buf.push_str(&("Unsupported Element: ".to_owned() + el_name + " "));
            get_text(buf, &element.children);
        },
    };

}

/// Adds text augmentation to buffer string
/// Emphasis or Strong - Italics or Bold
fn display_element_augment_text(buf: &mut String, element: &Element, augment: &str) {
    buf.push_str(augment);
    get_text(buf, &element.children);
    buf.push_str(augment);
}

/// Adds text representation of Div element to buffer string
/// Will continue rendering based on the amount of children elements
fn display_element_div(buf: &mut String, element: &Element) {
    get_text(buf, &element.children);
}

/// Adds text representation of Header element to buffer string
fn display_element_heading(buf: &mut String, text: &Node, num: usize) {
    if let Node::Text(t) = text {
        let header = "#".repeat(num)+" "+t+"\n";
        buf.push_str("\n");
        buf.push_str(&header);
    };
}

/// Adds text representation of Paragraph element to buffer string
/// Will continue rendering based on the amount of children elements
fn display_element_paragraph(buf: &mut String, element: &Element) {
    get_text(buf, &element.children);
    buf.push_str("\n");
}

/// Adds text representation of Anchor element to buffer string
fn display_element_anchor(buf: &mut String, element: &Element) {
    let anchor_text: &str;
    if (&element).children.len() != 0 {
        anchor_text = match &element.children[0] {
            Node::Text(t) => &t,
            _ => &"",
        };
    } else {
        anchor_text = match element.attributes.get("aria-label") {
            None => &"This link has no loadable text",
            Some(x) => match x {
                None => "N/A",
                Some(y) => &y,
            },
        };
    }
    let anchor = "[".to_owned()+anchor_text+"]("+ match element.attributes.get("href") {
        None => return,
        Some(x) => match x {
            None => "N/A",
            Some(y) => y,
        }
    } +")";
    buf.push_str(&anchor);
}

/// Adds text representation of an ordered list to buffer string
fn display_element_list_ordered(buf: &mut String, element: &Element) {
    buf.push_str("\n");
    let mut index = 1;
    for node in &element.children {
        match node {
            Node::Element(el) => {
                let el_name = el.name.to_lowercase();
                if !(HIDDEN_TAGS.iter().any(|&i| i == el.name.to_lowercase())) {
                    if el_name == "li" {
                        buf.push_str("\t");
                        buf.push_str(&index.to_string());
                        index += 1;
                        buf.push_str(". ");
                        get_text(buf, &el.children);
                        buf.push_str("\n");
                    } else {
                        get_text(buf, &el.children);
                    }
                }
            },
            _ => {}
        }
    }
    buf.push_str("\n");
}

/// Adds text representation of an unordered list to buffer string
fn display_element_list_unordered(buf: &mut String, element: &Element) {
    buf.push_str("\n");
    for node in &element.children {
        match node {
            Node::Element(el) => {
                let el_name = el.name.to_lowercase();
                if !(HIDDEN_TAGS.iter().any(|&i| i == el_name)) {
                    if el_name == "li" {
                        buf.push_str("+ ");
                        get_text(buf, &el.children);
                        buf.push_str("\n");
                    } else {
                        get_text(buf, &el.children);
                    }
                }
            },
            _ => {}
        }
    }
    buf.push_str("\n");
}

/// Adds text representation of a nav element to buffer string
fn display_element_nav(buf: &mut String, element: &Element) {
    buf.push_str("\nNavigation\n\n----\n");
    get_text(buf, &element.children);
    buf.push_str("\n----\n\nNavigation End\n");
}

