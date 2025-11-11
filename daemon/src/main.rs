use std::process::ExitCode;

use log::{error, info};

use crate::daemon::Daemon;

mod daemon;

fn main() -> ExitCode {
    env_logger::init();

    info!("Starting waysted daemon...");

    match Daemon::new() {
        Ok(mut daemon) => {
            if let Err(err) = daemon.start() {
                error!("{err}");
                return ExitCode::FAILURE;
            }
        }
        Err(err) => {
            error!("Failed to initialize daemon: {err}");
            return ExitCode::FAILURE;
        }
    }

    info!("Stopping waysted daemon.");

    ExitCode::SUCCESS
}
