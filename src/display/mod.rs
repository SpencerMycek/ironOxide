#![allow(dead_code)]
#![deny(warnings)]
#![warn(rust_2018_idioms)]

use std::default::Default;

use rustbox::{Color, RustBox, Key};

pub fn draw() {
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    rustbox.print(1, 1, rustbox::RB_BOLD, Color::White, Color::Black, "Hello, World!");
    rustbox.print(1, 3, rustbox::RB_BOLD, Color::White, Color::Black,
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

