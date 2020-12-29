//! # Iron Oxide
//!
//! This is the main running process of the `Iron Oxide` browser and
//! orchestrates all of the moving pieces


mod display;
mod dom;
mod error;
mod grammar;
mod http;
pub mod cli;

use grammar::Rule;
pub use anyhow::Result;
pub use crate::cli::Opts;
pub use crate::dom::Dom;
pub use crate::error::Error;

// Type alias so as to DRY
//pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Executes as the main process for `Iron Oxide`
pub async fn run(args: cli::Opts) -> Result<()> {
    let url = args.url;

    let body = http::get(&url).await?;
    let dom = Dom::parse(&body)?;

    display::display(&dom, args.ncurses);

    Ok(())
}

