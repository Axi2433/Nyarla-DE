use smithay::{
    backend::renderer::{gles::GlesRenderer, Renderer},
    reexports::wayland_server::{Display, DisplayHandle},
    wayland::compositor::CompositorState,
    xdg_shell::XdgShellState,
};
use calloop::{EventLoop, LoopHandle};
use std::sync::Arc;

pub struct State {
    // Core Smithay components
    pub display: Display<State>,
    pub event_loop: Option<LoopHandle<'static, State>>,
    
    // Custom components
    pub compositor_state: CompositorState,
    pub xdg_shell_state: XdgShellState,
}

impl State {
    pub fn new(display: Display<State>) -> Self {
        let compositor_state = CompositorState::new();
        let xdg_shell_state = XdgShellState::new();
        
        Self {
            display,
            event_loop: None,
            compositor_state,
            xdg_shell_state,
        }
    }
}