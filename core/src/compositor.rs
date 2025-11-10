use std::{env, io};

mod niri;

pub trait Compositor {
    fn get_focused_window(&self);

    fn watch_focused_window(&self, on_change: fn() -> ());
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

    match compositor_name.as_str() {
        "niri" => niri::Niri::new(),
        _ => todo!(),
    }
}
