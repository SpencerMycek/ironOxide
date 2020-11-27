//! # Iron Oxide Ncurses Display
//!
//! Displays the web-content, typically an html webpage, using Ncurses

#![allow(dead_code)]
#![deny(warnings)]
#![warn(rust_2018_idioms)]

use std::default::Default;
use rustbox::{Color, RustBox, Key};

use super::super::dom::Dom;
use super::get_title;

static UPPER_LEFT: char = '\u{250C}';
static UPPER_RIGHT: char = '\u{2510}';
static TITLE_LOWER_RIGHT: char = '\u{2524}';
static TITLE_LOWER_LEFT: char = '\u{2534}';
static LOWER_LEFT: char = '\u{2514}';
static LOWER_RIGHT: char = '\u{2518}';
static HORIZONTAL: char = '\u{2500}';
static VERTICAL: char = '\u{2502}';

/// Displays the provided DOM using Ncurses
pub fn display(dom: &Dom) {
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };
    let height: usize = rustbox.height() - 5;
    let width: usize = 84;
    let left_margin = 2;
    let top_margin = 4;
    let x = 1;
    let y = 3;

    draw_border(&rustbox, x, y, width, height, Color::White, Color::White, Color::Default);
    draw_title(&rustbox, x, y, width, Color::White, Color::Default, dom);
    rustbox.print(left_margin, top_margin, rustbox::RB_BOLD, Color::White, 
                  Color::Default, "Hello, World!");
    rustbox.print(x+width-17, y+height, rustbox::RB_BOLD, Color::White, 
                  Color::Default, "Press q to quit");
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
}

fn draw_title(rb: &RustBox, x:usize, y:usize, width: usize, fg: Color, bg: Color, dom: &Dom) {
    let mut title = match get_title(&dom) {
        None => return,
        Some(x) => x,
    };
    let length: usize;
    if title.len() >= 10 {
        length = 14;
        title = (&title[0..7]).to_owned()+"...";
    } else {
        length = title.len()+4;
    }
    rb.print(x+5, y+5, rustbox::RB_NORMAL, fg, bg, &title);
    rb.print(x+5, y+6, rustbox::RB_NORMAL, fg, bg, &length.to_string());

    draw_vertical_line(rb, x+width, y-2, 2, fg, bg);
    draw_vertical_line(rb, x+width-length, y-2, 2, fg, bg);
    draw_horizontal_line(rb, x+width-length, y-2, length, fg, bg);
    rb.print_char(x+width-length, y-2, rustbox::RB_NORMAL, fg, bg, UPPER_LEFT);
    rb.print_char(x+width, y-2, rustbox::RB_NORMAL, fg, bg, UPPER_RIGHT);
    rb.print_char(x+width-length, y, rustbox::RB_NORMAL, fg, bg, TITLE_LOWER_LEFT);
    rb.print_char(x+width, y, rustbox::RB_NORMAL, fg, bg, TITLE_LOWER_RIGHT);
    rb.print(x+width-length+2, y-1, rustbox::RB_NORMAL, fg, bg, &title);
}

