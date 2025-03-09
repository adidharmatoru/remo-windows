use remo_core::{prelude::*, RemoCore, tokio};

#[remo_core::tokio::main]
async fn main() -> Result<()> {
    // Load configuration
    let config = Config::load()?;

    // Create and start RemoCore
    let mut core = RemoCore::new(config).await?;
    core.start().await?;

    // Wait for Ctrl+C
    remo_core::tokio::signal::ctrl_c().await?;
    println!("Shutting down Remo...");

    // Stop RemoCore
    core.stop().await?;

    println!("Remo terminated.");

    Ok(())
}
