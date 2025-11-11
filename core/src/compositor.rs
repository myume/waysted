use std::{env, io, sync::mpsc::Sender};

use log::info;

mod niri;

#[derive(Debug)]
pub struct WindowInfo {
    pub title: String,
    pub app_name: String,
}

pub trait Compositor {
    /// Retrieve the currently focused window.
    fn get_focused_window(&mut self) -> Result<WindowInfo, String>;

    /// Watch for changes in the focused window. Sends the window info over the channel.
    /// This method will block the main thread and only return if the compositor IPC socket is closed.
    fn watch_focused_window(&mut self, sender: Sender<WindowInfo>) -> io::Result<()>;
}

const CURRENT_DESKTOP_ENV: &str = "XDG_CURRENT_DESKTOP";

pub fn get_current_compositor() -> io::Result<impl Compositor> {
    let compositor_name = env::var(CURRENT_DESKTOP_ENV).map_err(|err| match err {
        env::VarError::NotPresent => io::Error::new(
            io::ErrorKind::NotFound,
            format!("No compositor found, {CURRENT_DESKTOP_ENV} is not set."),
        ),
        env::VarError::NotUnicode(_) => io::Error::new(
            io::ErrorKind::InvalidData,
            format!("{CURRENT_DESKTOP_ENV} is not not valid unicode"),
        ),
    })?;

    info!("{compositor_name} compositor found.");

    match compositor_name.as_str() {
        "niri" => niri::Niri::new(),
        unsupported => Err(io::Error::new(
            io::ErrorKind::Unsupported,
            format!(
                "The {unsupported} compositor is currently unsupported, please file an issue or open a PR."
            ),
        )),
    }
}
