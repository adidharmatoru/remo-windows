use remo_core::{
    prelude::*,
    tokio::{
        self, select,
        signal::windows::{ctrl_break, ctrl_c},
    },
    RemoCore,
};

#[remo_core::tokio::main]
async fn main() -> Result<()> {
    // Load configuration
    let config = Config::load()?;

    // Create and start RemoCore
    let mut core = RemoCore::new(config).await?;
    core.start().await?;

    // Create signal handlers
    let mut ctrl_c = ctrl_c()?;
    let mut ctrl_break = ctrl_break()?;

    // Wait for either Ctrl+C (restart) or Ctrl+Break (exit)
    loop {
        select! {
            _ = ctrl_c.recv() => {
                println!("Received Ctrl+C, restarting...");
                if let Err(e) = core.restart().await {
                    println!("Error during restart: {}", e);
                    break;
                }
                println!("Restart completed. Press Ctrl+Break to exit or Ctrl+C to restart again.");
                continue;
            }
            _ = ctrl_break.recv() => {
                println!("Received Ctrl+Break, shutting down...");
                break;
            }
        }
    }

    // Stop RemoCore
    core.stop().await?;

    println!("Remo terminated.");

    Ok(())
}
