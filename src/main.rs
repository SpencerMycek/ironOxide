#![deny(warnings)]
#![warn(rust_2018_idioms)]

use pretty_env_logger;
use log::LevelFilter;
#[macro_use] extern crate log;

#[tokio::main]
async fn main() -> iron_oxide::Result<()> {
    let mut builder = pretty_env_logger::formatted_builder();
    builder.filter(None, LevelFilter::Info).init();

    trace!("a trace example");
    //debug!("deboogging");
    //info!("such information");
    //warn!("o_O");
    //error!("boom");

    let running = iron_oxide::run();
    running.await
}

