// Hide console window
#![windows_subsystem = "windows"]

use remo_core::{
    prelude::*,
    tokio::{self, signal::ctrl_c},
    RemoCore,
};

#[remo_core::tokio::main]
async fn main() -> Result<()> {
    // Load configuration
    let config = Config::load()?;

    // Create and start RemoCore
    let mut core = RemoCore::new(config).await?;
    core.start().await?;

    // Keep the application running
    ctrl_c().await?;

    // Stop RemoCore
    core.stop().await?;

    Ok(())
}
