use cgmath::{Vector2, Vector3};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FloatingElement {
    pub id: u32,
    pub position: Vector2<f64>,
    pub size: Vector2<f64>,
    pub is_visible: bool,
    pub z_index: i32,
}

#[derive(Debug, Clone)]
pub struct FloatingWindow {
    pub id: u32,
    pub title: String,
    pub position: Vector2<f64>,
    pub size: Vector2<f64>,
    pub is_focused: bool,
    pub is_minimized: bool,
    pub is_maximized: bool,
}

#[derive(Debug, Clone)]
pub struct WindowFrame {
    pub id: u32,
    pub window_id: u32,
    pub position: Vector2<f64>,
    pub size: Vector2<f64>,
    pub title_bar_height: f64,
}

pub struct ElementsState {
    pub floating_elements: HashMap<u32, FloatingElement>,
    pub windows: HashMap<u32, FloatingWindow>,
    pub window_frames: HashMap<u32, WindowFrame>,
    pub active_window_id: Option<u32>,
}

impl ElementsState {
    pub fn new() -> Self {
        ElementsState {
            floating_elements: HashMap::new(),
            windows: HashMap::new(),
            window_frames: HashMap::new(),
            active_window_id: None,
        }
    }

    pub fn add_floating_element(&mut self, id: u32, position: Vector2<f64>, size: Vector2<f64>) {
        let element = FloatingElement {
            id,
            position,
            size,
            is_visible: true,
            z_index: 0,
        };
        self.floating_elements.insert(id, element);
    }

    pub fn add_window(&mut self, id: u32, title: String, position: Vector2<f64>, size: Vector2<f64>) {
        let window = FloatingWindow {
            id,
            title,
            position,
            size,
            is_focused: false,
            is_minimized: false,
            is_maximized: false,
        };
        self.windows.insert(id, window);
    }

    pub fn add_window_frame(&mut self, id: u32, window_id: u32, position: Vector2<f64>, size: Vector2<f64>, title_bar_height: f64) {
        let frame = WindowFrame {
            id,
            window_id,
            position,
            size,
            title_bar_height,
        };
        self.window_frames.insert(id, frame);
    }

    pub fn focus_window(&mut self, window_id: u32) {
        // First, unfocus any currently focused windows
        for window in self.windows.values_mut() {
            if window.is_focused {
                window.is_focused = false;
            }
        }
        
        // Then focus the new window
        if let Some(window) = self.windows.get_mut(&window_id) {
            window.is_focused = true;
            self.active_window_id = Some(window_id);
        }
    }

    pub fn minimize_window(&mut self, window_id: u32) {
        if let Some(window) = self.windows.get_mut(&window_id) {
            window.is_minimized = true;
        }
    }

    pub fn maximize_window(&mut self, window_id: u32) {
        if let Some(window) = self.windows.get_mut(&window_id) {
            window.is_maximized = true;
        }
    }
}