//! Window manager module for the Nyarla DE compositor

use smithay::{
    wayland::compositor::CompositorState,
    xdg_shell::XdgShellState,
    reexports::wayland_server::{Display, DisplayHandle},
};
use std::collections::HashMap;

/// Window manager state
pub struct WindowManager {
    pub compositor_state: CompositorState,
    pub xdg_shell_state: XdgShellState,
    pub windows: HashMap<u32, Window>,
    pub focused_window: Option<u32>,
}

/// Window representation
#[derive(Debug, Clone)]
pub struct Window {
    pub id: u32,
    pub title: String,
    pub is_focused: bool,
    pub is_minimized: bool,
    pub is_maximized: bool,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl WindowManager {
    pub fn new() -> Self {
        WindowManager {
            compositor_state: CompositorState::new(),
            xdg_shell_state: XdgShellState::new(),
            windows: HashMap::new(),
            focused_window: None,
        }
    }

    pub fn add_window(&mut self, id: u32, title: String, x: i32, y: i32, width: u32, height: u32) {
        let window = Window {
            id,
            title,
            is_focused: false,
            is_minimized: false,
            is_maximized: false,
            x,
            y,
            width,
            height,
        };
        self.windows.insert(id, window);
    }

    pub fn focus_window(&mut self, id: u32) {
        if let Some(window) = self.windows.get_mut(&id) {
            window.is_focused = true;
            self.focused_window = Some(id);
        }
    }

    pub fn minimize_window(&mut self, id: u32) {
        if let Some(window) = self.windows.get_mut(&id) {
            window.is_minimized = true;
        }
    }

    pub fn maximize_window(&mut self, id: u32) {
        if let Some(window) = self.windows.get_mut(&id) {
            window.is_maximized = true;
        }
    }

    pub fn update_window_geometry(&mut self, id: u32, x: i32, y: i32, width: u32, height: u32) {
        if let Some(window) = self.windows.get_mut(&id) {
            window.x = x;
            window.y = y;
            window.width = width;
            window.height = height;
        }
    }

    pub fn remove_window(&mut self, id: u32) {
        self.windows.remove(&id);
        if self.focused_window == Some(id) {
            self.focused_window = None;
        }
    }
}