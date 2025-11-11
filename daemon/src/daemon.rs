use std::{io, time::Instant};

use log::debug;
use waysted_core::compositor::{Compositor, WindowInfo, get_current_compositor};

pub struct Daemon {
    compositor: Box<dyn Compositor>,
    focused_window: Option<WindowInfo>,
    focus_start_time: Instant,
}

impl Daemon {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            compositor: Box::new(get_current_compositor()?),
            focused_window: None,
            focus_start_time: Instant::now(),
        })
    }

    pub fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.focus_start_time = Instant::now();
        self.focused_window = Some(self.compositor.get_focused_window()?);

        self.compositor.watch_focused_window(|window_info| {
            debug!("{}", window_info.app_name,);
        })?;

        Ok(())
    }
}
