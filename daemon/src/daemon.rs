use std::io;

use waysted_core::compositor::{Compositor, get_current_compositor};

pub struct Daemon {
    compositor: Box<dyn Compositor>,
}

impl Daemon {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            compositor: Box::new(get_current_compositor()?),
        })
    }

    pub fn start(&mut self) -> Result<(), String> {
        self.compositor.get_focused_window()?;
        Ok(())
    }
}
