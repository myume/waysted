use std::io;

use crate::compositor::Compositor;
use hyprland::data::Client;
use hyprland::event_listener::EventListener;
use hyprland::shared::HyprDataActiveOptional;
use log::error;

pub struct Hyprland {}

impl Hyprland {
    pub fn new() -> io::Result<Self> {
        Ok(Self {})
    }
}

impl Compositor for Hyprland {
    fn get_focused_window(&mut self) -> Result<super::WindowInfo, String> {
        match Client::get_active() {
            Ok(Some(client)) => Ok(super::WindowInfo {
                title: client.title,
                app_name: client.class.to_string(),
            }),
            Ok(None) => Err(format!("No window has focus in Hyprland")),
            Err(err) => Err(format!("Failed to get focused window from Hyprland: {err}")),
        }
    }

    fn watch_focused_window(
        &mut self,
        sender: std::sync::mpsc::Sender<super::WindowInfo>,
    ) -> std::io::Result<()> {
        let mut event_listener = EventListener::new();
        event_listener.add_active_window_changed_handler(move |data| match data {
            Some(window_event) => {
                if let Err(err) = sender.send(super::WindowInfo {
                    title: window_event.title,
                    app_name: window_event.class.to_string(),
                }) {
                    error!("Failed to send window info: {err}");
                };
            }
            None => {
                error!("New focused window is missing.");
            }
        });

        event_listener.start_listener().unwrap();
        Ok(())
    }
}
