mod compositor;

use smithay::{
    backend::renderer::{gles::GlesRenderer, Renderer},
    reexports::wayland_server::{Display, DisplayHandle},
    wayland::compositor::CompositorState,
    wayland::shell::xdg::XdgShellState,
};
use calloop::EventLoop;
use std::sync::Arc;

fn main() {
    // Initialize the Wayland display
    let display = Display::new().unwrap();
    
    // Create our compositor state
    let compositor_state = CompositorState::new();
    let xdg_shell_state = XdgShellState::new(&display.handle());
    
    // Initialize the event loop
    let event_loop = EventLoop::try_new().unwrap();
    
    // Start the compositor
    println!("Starting Nyarla DE compositor...");
    
    // Run the event loop (this will block)
    event_loop.run(None, &mut (), |_| {}).unwrap();
}