#![deny(warnings)]
#![warn(rust_2018_idioms)]

use pretty_env_logger;
use log::LevelFilter;
//#[macro_use] extern crate log;

use iron_oxide::cli;

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

