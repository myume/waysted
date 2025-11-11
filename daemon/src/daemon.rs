use std::{io, sync::mpsc::channel, thread::spawn, time::Instant};

use chrono::Utc;
use log::{debug, info};
use waysted_core::{
    compositor::{Compositor, WindowInfo, get_current_compositor},
    database::Database,
};

pub struct Daemon {
    compositor: Box<dyn Compositor>,
}

impl Daemon {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            compositor: Box::new(get_current_compositor()?),
        })
    }

    pub fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let (sender, receiver) = channel();

        let db = Database::new()?;
        let handle = spawn(move || {
            let mut start_timestamp = Utc::now();
            let mut start_time = Instant::now();
            let mut focused_window: Option<WindowInfo> = None;
            while let Ok(currently_focused_window) = receiver.recv() {
                if let Some(previously_focused_window) = focused_window {
                    let duration = start_time.elapsed();
                    let end_timestamp = Utc::now();
                    debug!(
                        "{} focused for {}ms",
                        previously_focused_window.app_name,
                        duration.as_millis()
                    );

                    db.log_focus_duration(
                        previously_focused_window,
                        duration,
                        start_timestamp,
                        end_timestamp,
                    );
                }

                start_time = Instant::now();
                start_timestamp = Utc::now();
                focused_window = Some(currently_focused_window);
            }
        });

        info!("Watching for changes in the focused window");
        self.compositor.watch_focused_window(sender)?;

        handle.join().unwrap();

        Ok(())
    }
}
