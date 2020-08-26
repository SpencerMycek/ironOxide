#![allow(dead_code)]
#![deny(warnings)]
#![warn(rust_2018_idioms)]

use clap::Clap;

/// A rust-based mini web-browser
#[derive(Clap)]
#[clap(version="0.9", author="Spencer M. <Spence.Mycek@gmail.com>")]
pub struct Opts {
    /// A level of verbosity, can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    pub verbose: i32,
    /// Request URL
    pub url: String,
    /// Use ncurses for webpage display
    #[clap(short, long)]
    pub ncurses: bool,
}

/*
 * Function: get_args
 * -------------------
 * Parses command line arguments with clap-rs,
 *     cli args are build with clap derive in Opts
 *
 * return:  An Opts struct of parsed command line args
 *              Or exits the program
 */
pub fn get_args() -> Opts {
    Opts::parse()
}

