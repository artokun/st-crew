pub mod app;
pub mod children;
pub mod components;
pub mod utils;

use color_eyre::eyre::Result;
use components::home::Home;

use crate::{
    app::run_app,
    utils::{initialize_logging, initialize_panic_handler},
};

async fn tokio_main() -> Result<()> {
    initialize_logging()?;

    initialize_panic_handler()?;

    run_app(Home::new).await
}

#[tokio::main]
async fn main() -> Result<()> {
    if let Err(e) = tokio_main().await {
        eprintln!("{} error: Something went wrong", env!("CARGO_PKG_NAME"));
        Err(e)
    } else {
        Ok(())
    }
}
