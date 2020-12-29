//! # Iron Oxide Ncurses Display
//!
//! Displays the web-content, typically an html webpage, using Ncurses

#![allow(dead_code)]
#![deny(warnings)]
#![warn(rust_2018_idioms)]

use std::default::Default;
use rustbox::{Color, RustBox, Key};

use super::super::dom::Dom;
use super::super::dom::node::Node;
use super::super::dom::element::{Element, HIDDEN_TAGS};
use super::{get_title, get_visible_nodes};

static UPPER_LEFT: char = '\u{250C}';
static UPPER_RIGHT: char = '\u{2510}';
static TITLE_LOWER_RIGHT: char = '\u{2524}';
static TITLE_LOWER_LEFT: char = '\u{2534}';
static LOWER_LEFT: char = '\u{2514}';
static LOWER_RIGHT: char = '\u{2518}';
static HORIZONTAL: char = '\u{2500}';
static VERTICAL: char = '\u{2502}';
static BULLET: char = '\u{2022}';

/// Displays the provided DOM using Ncurses
pub fn display(dom: &Dom) {
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };
    let height: usize = rustbox.height() - 5;
    let width: usize = 84;
    let x = 1;
    let y = 3;


    let title = match get_title(&dom) {
        None => "".to_string(),
        Some(x) => x,
    };
    let mut content = "".to_string();
    let nodes = get_visible_nodes(&dom);
    get_content(&mut content, nodes);
    let processed_content: Vec<String> = process_content(content, width-4);
    let content_length = processed_content.len();
    let mut line: usize = 0;

    loop {
        rustbox.clear();
        draw_browser(&rustbox, x, y, width, height, &processed_content, &title, line);

        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Char('q') => {break;},
                    Key::Up => {
                        if line != 0 {
                            line -= 1;
                        }
                    },
                    Key::Down => {
                        if content_length >= height && line < content_length-height+2 {
                            line += 1;
                        }
                    },
                    _ => {}
                }
            },
            Err(e) => panic!("{}", e),
            _ => { }
        }
    }
}

/// Handles everything needed to build and present the ncurses display
fn draw_browser(rb: &RustBox, x: usize, y: usize, width: usize, height: usize, content: &Vec<String>, title: &String, line: usize) {
    draw_border(rb, x, y, width, height, Color::White, Color::White, Color::Default);
    draw_title(rb, x, y, width, Color::White, Color::Default, title);
    display_content(rb, x+2, y+1, height-2, content, line);
    rb.present();
    
}

/// Adds the lines of the content to the rustbox display, 
/// only provides lines that will fit
fn display_content(rb: &RustBox, x: usize, y: usize, height: usize, content: &Vec<String>, line: usize) {
    let length = content.len();
    for i in 0..height {
        if i+line >= length {
            break;
        }
        rb.print(x, y+i, rustbox::RB_NORMAL, Color::White, Color::Default,
             &content[i+line]);
    }
}

/// Splits a string into <width> character lines, stored in a Vec
fn process_content(content: String, width: usize) -> Vec<String> {
    let mut processed: Vec<String> = Vec::new();
    let mut word_buf = String::with_capacity(width);
    let mut line_buf = String::with_capacity(width);
    let mut chars = content.chars();
    loop {
        let curr: char = match chars.next() {
            None => {
                if word_buf.len() != 0 {
                    line_buf.push_str(&word_buf);
                }
                if line_buf.len() != 0 {
                    processed.push(line_buf.clone());    
                }
                break;
            },
            Some(x) => x,
        };
        if curr == ' ' {
            word_buf.push(curr);
            if word_buf.len() + line_buf.len() > width {
                processed.push(line_buf.clone());
                line_buf.clear();
            }
            line_buf.push_str(&word_buf);
            word_buf.clear();
        } else if curr == '\n' {
            if word_buf.len() + line_buf.len() > width {
                processed.push(line_buf.clone());
                processed.push(word_buf.clone());
            } else {
                line_buf.push_str(&word_buf);
                processed.push(line_buf.clone());
            }
            line_buf.clear();
            word_buf.clear();
        } else if curr == '\t' {
            word_buf.push_str(&" ".repeat(4));
        } else {
            word_buf.push(curr);
            if word_buf.len() + line_buf.len() > width {
                processed.push(line_buf.clone());
                line_buf.clear();
                line_buf.push_str(&word_buf);
                word_buf.clear();
            } else if word_buf.len() >= width {
                processed.push(line_buf.clone());
                line_buf.clear();
                processed.push(word_buf.clone());
                word_buf.clear();
            }
        }
    }
    return processed;
}

/// Fills a string buffer with content to display from the DOM
/// Uses indirect recursion to access all elements
fn get_content(buf: &mut String, nodes: &Vec<Node>) {
    for node in nodes {
        match node {
            Node::Text(s) => buf.push_str(&s),
            Node::Element(el) => {
                if !(HIDDEN_TAGS.iter().any(|&i| i == el.name.to_lowercase())) {
                    delegate_elements(buf, el);
                }
            },
            Node::Comment(_) => {},
        };
    }
}

/// Draws a horizontal line
fn draw_horizontal_line(rb: &RustBox, x: usize, y: usize, width: usize, fg: Color, bg: Color) {
    for i in 0..width+1 {
        rb.print_char(x+i, y, rustbox::RB_NORMAL, fg, bg, HORIZONTAL);
    }
}

/// Draws a vertical line
fn draw_vertical_line(rb: &RustBox, x: usize, y: usize, height: usize, fg: Color, bg: Color) {
    for i in 0..height+1 {
        rb.print_char(x, y+i, rustbox::RB_NORMAL, fg, bg, VERTICAL);
    }
}

/// Draws the Iron Oxide Ncurses border around the display area
fn draw_border(rb: &RustBox, x: usize, y: usize, width: usize, height: usize, _fill: Color, fg: Color, bg: Color) {
    draw_horizontal_line(rb, x, y, width, fg, bg);
    draw_horizontal_line(rb, x, y+height, width, fg, bg);
    draw_vertical_line(rb, x, y, height, fg, bg);
    draw_vertical_line(rb, x+width, y, height, fg, bg);
    rb.print_char(x, y, rustbox::RB_NORMAL, fg, bg, UPPER_LEFT);
    rb.print_char(x+width, y, rustbox::RB_NORMAL, fg, bg, UPPER_RIGHT);
    rb.print_char(x, y+height, rustbox::RB_NORMAL, fg, bg, LOWER_LEFT);
    rb.print_char(x+width, y+height, rustbox::RB_NORMAL, fg, bg, LOWER_RIGHT);
    rb.print(x+1, y, rustbox::RB_NORMAL, fg, bg, "Iron Oxide");
    rb.print(x+width-17, y+height, rustbox::RB_NORMAL, Color::White, 
                  Color::Default, "Press q to quit");
}

fn draw_title(rb: &RustBox, x:usize, y:usize, width: usize, fg: Color, bg: Color, title: &str) {
    let display_title: String;
    let length: usize;
    if title.len() >= 10 {
        length = 14;
        display_title = title[0..7].to_owned()+"...";
    } else {
        length = title.len()+4;
        display_title = title.to_owned();
    }
    draw_vertical_line(rb, x+width, y-2, 2, fg, bg);
    draw_vertical_line(rb, x+width-length, y-2, 2, fg, bg);
    draw_horizontal_line(rb, x+width-length, y-2, length, fg, bg);
    rb.print_char(x+width-length, y-2, rustbox::RB_NORMAL, fg, bg, UPPER_LEFT);
    rb.print_char(x+width, y-2, rustbox::RB_NORMAL, fg, bg, UPPER_RIGHT);
    rb.print_char(x+width-length, y, rustbox::RB_NORMAL, fg, bg, TITLE_LOWER_LEFT);
    rb.print_char(x+width, y, rustbox::RB_NORMAL, fg, bg, TITLE_LOWER_RIGHT);
    rb.print(x+width-length+2, y-1, rustbox::RB_NORMAL, fg, bg, &display_title);
}

fn delegate_elements(buf: &mut String, element: &Element) {
    let el_name: &str = &element.name.to_lowercase();
    match &el_name[..] {
        "a" => anchor(buf, element),
        "article" => div(buf, element),
        "br" => buf.push('\n'),
        "button" => buf.push_str("{Button elements not yet supported}"),
        "center" => div(buf, element),
        "div" => div(buf, element),
        "em" => augment_text(buf, element, "<em: "),
        "figure" => buf.push_str("{Figure elements not yet supported}"),
        "footer" => div(buf, element),
        "form" => buf.push_str("{Form elements not yet supported}"),
        "h1" => heading(buf, &element.children[0], 1),
        "h2" => heading(buf, &element.children[0], 2),
        "h3" => heading(buf, &element.children[0], 3),
        "h4" => heading(buf, &element.children[0], 4),
        "h5" => heading(buf, &element.children[0], 5),
        "h6" => heading(buf, &element.children[0], 6),
        "header" => div(buf, element),
        "hr" => buf.push_str("\n----\n"),
        "iframe" => buf.push_str("{Iframe elements not yet supported}"),
        "img" => buf.push_str("{Img elements not yet supported}"),
        "input" => buf.push_str("{Input elements not yet supported}"),
        "main" => div(buf, element),
        "nav" => nav(buf, element),
        "noscript" => div(buf, element),
        "ol" => list_ordered(buf, element),
        "p" => paragraph(buf, element),
        "picture" => buf.push_str("{Picture elements not yet supported}"),
        "section" => div(buf, element),
        "select" => {},
        "span" => span(buf, element),
        "strong" => augment_text(buf, element, "<strong: "),
        "sub" => get_content(buf, &element.children),
        "sup" => get_content(buf, &element.children),
        "svg" => {},
        "table" => buf.push_str("{Table elements not yet supported}"),
        "ul" => list_unordered(buf, element),
        "video" => buf.push_str("{Video elements not yet supported}"),
        _ => {
            buf.push_str(&("<".to_owned()+el_name+&">"));
            get_content(buf, &element.children)
        },
    };
}

fn paragraph(buf: &mut String, element: &Element) {
    get_content(buf, &element.children);
    buf.push('\n');
}

fn span(buf: &mut String, element: &Element) {
    get_content(buf, &element.children);
}

fn div(buf: &mut String, element: &Element) {
    get_content(buf, &element.children);
    buf.push('\n');
}

fn heading(buf: &mut String, text: &Node, num: usize) {
    if let Node::Text(t) = text {
        let header =  "#".repeat(num)+" "+t+"\n";
        buf.push_str("\n");
        buf.push_str(&header);
    }
}

fn augment_text(buf: &mut String, element: &Element, augment: &str) {
    buf.push_str(augment);
    get_content(buf, &element.children);
    buf.push_str(">");
}

fn nav(buf: &mut String, element: &Element) {
    buf.push_str("\n-Navigation-\n");
    get_content(buf, &element.children);
    buf.push_str("\n-Navigation-End-\n");
}

fn anchor(buf: &mut String, element: &Element) {
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
    let anchor = "[".to_owned()+anchor_text+"]("+match element.attributes.get("href") {
        None => return,
        Some(x) => match x {
            None => "N/A",
            Some(y) => y,
        }
    } + ")";
    buf.push_str(&anchor);
}

fn list_ordered(buf: &mut String, element: &Element) {
    buf.push('\n');
    let mut index = 1;
    for node in &element.children {
        match node {
            Node::Element(el) => {
                let el_name = el.name.to_lowercase();
                if !(HIDDEN_TAGS.iter().any(|&i| i == el_name)) {
                    if el_name == "li" {
                        buf.push('\t');
                        buf.push_str(&index.to_string());
                        index += 1;
                        buf.push_str(". ");
                        get_content(buf, &el.children);
                        buf.push('\n');
                    } else {
                        get_content(buf, &el.children);
                    }
                }
            },
            _ => {}
        }
    }
    buf.push('\n');
}

fn list_unordered(buf: &mut String, element: &Element) {
    buf.push('\n');
    for node in &element.children {
        match node {
            Node::Element(el) => {
                let el_name = el.name.to_lowercase();
                if !(HIDDEN_TAGS.iter().any(|&i| i == el_name)) {
                    if el_name == "li" {
                        buf.push('\t');
                        buf.push(BULLET);
                        buf.push(' ');
                        get_content(buf, &el.children);
                        buf.push('\n');
                    } else {
                        get_content(buf, &el.children);
                    }
                }
            },
            _ => {}
        }
    }
    buf.push('\n');
}

