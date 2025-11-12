use std::{collections::HashMap, io, sync::mpsc::Sender};

use super::{Compositor, WindowInfo};
use log::{debug, error, warn};
use niri_ipc::{Event, Request, Response, Window, socket::Socket};

pub struct Niri {
    socket: Socket,
    windows: HashMap<u64, Window>,
    focused_window_id: Option<u64>,
}

impl Niri {
    pub fn new() -> io::Result<Self> {
        let socket = Socket::connect()?;
        Ok(Self {
            socket,
            windows: HashMap::new(),
            focused_window_id: None,
        })
    }

    /// Clears the internal windows state and populates it with the [`new_windows`]
    fn populate_windows(&mut self, new_windows: Vec<Window>) {
        self.windows.clear();
        for window in new_windows {
            if window.is_focused {
                self.focused_window_id = Some(window.id);
            }

            self.windows.insert(window.id, window);
        }
    }

    fn get_windows(&mut self) -> Result<Vec<Window>, String> {
        match self.socket.send(Request::Windows) {
            Ok(Ok(Response::Windows(windows))) => Ok(windows),
            Ok(Ok(response)) => {
                debug!("Unexpected reply {:?}", response);
                Err("Unexpected reply from niri IPC socket".to_string())
            }
            Ok(Err(message)) => Err(format!("Error message returned from niri: {message}")),
            Err(err) => Err(format!("Failure to communicate with niri, {err}")),
        }
    }

    fn handle_event(&mut self, event: Event, sender: &Sender<WindowInfo>) {
        match event {
            niri_ipc::Event::WindowsChanged { windows } => {
                self.populate_windows(windows);
            }
            niri_ipc::Event::WindowOpenedOrChanged { window } => {
                if window.is_focused
                    && let Some(existing_window_info) = self.windows.get(&window.id)
                    && existing_window_info.title != window.title
                {
                    let window_info = WindowInfo {
                        title: window.title.clone().unwrap_or_default(),
                        app_name: window.app_id.clone().unwrap_or_default(),
                    };
                    if let Err(err) = sender.send(window_info) {
                        error!("Failed to send window info: {err}");
                    };
                }
                self.windows.insert(window.id, window);
            }
            niri_ipc::Event::WindowClosed { id } => {
                self.windows.remove(&id);
            }
            niri_ipc::Event::WindowFocusChanged { id } => {
                if let Some(focused_id) = self.focused_window_id {
                    if let Some(focused_window) = self.windows.get_mut(&focused_id) {
                        focused_window.is_focused = false;
                    } else {
                        warn!("Current focused window is missing: {focused_id}.");
                    }
                }

                self.focused_window_id = id;
                if let Some(id) = id {
                    if let Some(window) = self.windows.get_mut(&id) {
                        window.is_focused = true;

                        let window_info = WindowInfo {
                            title: window.title.clone().unwrap_or_default(),
                            app_name: window.app_id.clone().unwrap_or_default(),
                        };

                        if let Err(err) = sender.send(window_info) {
                            error!("Failed to send window info: {err}");
                        };
                    } else {
                        error!("New focused window is missing: {id}.");
                    }
                };
            }
            niri_ipc::Event::WindowUrgencyChanged { id, urgent } => {
                if let Some(window) = self.windows.get_mut(&id) {
                    window.is_urgent = urgent;
                } else {
                    error!("Window could not be found: {id}");
                }
            }
            niri_ipc::Event::WindowLayoutsChanged { changes } => {
                for (id, layout) in changes {
                    if let Some(window) = self.windows.get_mut(&id) {
                        window.layout = layout;
                    } else {
                        error!("Window could not be found: {id}");
                    };
                }
            }
            _ => {
                // ignore other events because they don't affect the window state
            }
        }
    }
}

impl Compositor for Niri {
    fn get_focused_window(&mut self) -> Result<WindowInfo, String> {
        match self.socket.send(Request::FocusedWindow) {
            Ok(Ok(Response::FocusedWindow(Some(window)))) => {
                self.focused_window_id = Some(window.id);
                Ok(WindowInfo {
                    title: window.title.unwrap_or_default(),
                    app_name: window.app_id.unwrap_or_default(),
                })
            }
            // Unexpected reply
            Ok(Ok(response)) => {
                debug!("Unexpected reply {:?}", response);
                Err("Unexpected reply from niri IPC socket".to_string())
            }
            // Niri returned an error
            Ok(Err(message)) => Err(format!("Error message returned from niri: {message}")),
            // Failed to communicate with niri
            Err(err) => Err(format!("Failure to communicate with niri, {err}")),
        }
    }

    /// Start watching the Niri events stream and call [`notify_focus_change`] on the WindowFocusChanged
    fn watch_focused_window(&mut self, sender: Sender<WindowInfo>) -> io::Result<()> {
        let windows = self.get_windows().map_err(io::Error::other)?;
        self.populate_windows(windows);

        let mut socket = Socket::connect()?;

        let reply = socket.send(Request::EventStream)?;
        if matches!(reply, Ok(Response::Handled)) {
            let mut read_event = socket.read_events();
            while let Ok(event) = read_event() {
                self.handle_event(event, &sender);
            }
        }

        Ok(())
    }
}
