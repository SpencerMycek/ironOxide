#![allow(dead_code)]
#![deny(warnings)]
#![warn(rust_2018_idioms)]

use clap::{Arg, App, ArgMatches};

pub fn get_args() -> ArgMatches {
    App::new("Iron Oxide")
        .arg(Arg::with_name("url").index(1).required(true))
        .arg(Arg::with_name("v")
             .short('v')
             .multiple(true)
             .about("Sets the level of verbosity"))
        .get_matches()
}

