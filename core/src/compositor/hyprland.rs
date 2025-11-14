use std::io;

use crate::compositor::Compositor;

pub struct Hyprland {}

impl Hyprland {
    pub fn new() -> io::Result<Self> {
        Ok(Self {})
    }
}

impl Compositor for Hyprland {
    fn get_focused_window(&mut self) -> Result<super::WindowInfo, String> {
        todo!()
    }

    fn watch_focused_window(
        &mut self,
        sender: std::sync::mpsc::Sender<super::WindowInfo>,
    ) -> std::io::Result<()> {
        todo!()
    }
}
