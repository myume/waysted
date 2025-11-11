use log::info;

use crate::daemon::Daemon;

mod daemon;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    info!("Starting waysted daemon...");

    let mut daemon = Daemon::new()?;
    daemon.start()?;

    info!("Stopping waysted daemon.");
    Ok(())
}
