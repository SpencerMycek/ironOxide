//! # Iron Oxide
//!
//! `Iron Oxide` is a lite web browser written entirely in rust.
//!
//! Due to using [`tokio`] the main function must be run as an async function.
//!
//! [`tokio`]: https://docs.rs/tokio/0.3.1/tokio/

#![deny(warnings)]
#![warn(rust_2018_idioms)]

use pretty_env_logger;
use log::LevelFilter;

use iron_oxide::cli;

/// Prepares the environment with a logger and gathers command line arguments
#[tokio::main]
async fn main() -> iron_oxide::Result<()> {
    let args = cli::get_args();

    let mut builder = pretty_env_logger::formatted_builder();

    let filter = match args.verbose {
        0 => LevelFilter::Error,
        1 => LevelFilter::Warn,
        2 => LevelFilter::Info,
        3 | _ => LevelFilter::Trace,
    };

    builder.filter(None, filter).init();

    let running = iron_oxide::run(args);
    running.await
}

