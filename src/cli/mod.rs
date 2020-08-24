#![allow(dead_code)]
#![deny(warnings)]
#![warn(rust_2018_idioms)]

use clap::Clap;

/// Test Doc String help message
#[derive(Clap)]
#[clap(version="0.9", author="Spencer M. <Spence.Mycek@gmail.com>")]
pub struct Opts {
    /// A level of verbosity, can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    pub verbose: i32,
    /// URL to get
    pub url: String,
}

pub fn get_args() -> Opts {
    Opts::parse()
}

