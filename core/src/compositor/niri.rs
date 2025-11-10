use std::io;

use super::Compositor;
use niri_ipc::socket::Socket;

pub struct Niri {
    socket: Socket,
}

impl Niri {
    pub fn new() -> io::Result<Self> {
        let socket = Socket::connect()?;
        Ok(Self { socket })
    }
}

impl Compositor for Niri {
    fn get_focused_window(&self) {
        todo!()
    }

    fn watch_focused_window(&self, on_focus_change: fn() -> ()) {
        todo!()
    }
}
