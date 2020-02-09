#![deny(warnings)]
#![warn(rust_2018_idioms)]


#[tokio::main]
async fn main() -> iron_oxide::Result<()> {
    pretty_env_logger::init();

    let _running = iron_oxide::run();
    //running.await?;
    
    Ok(())

}

