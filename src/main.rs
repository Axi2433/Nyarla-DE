mod compositor;
mod shell;
mod menu;
mod effects;
mod elements;

use compositor::State;
use smithay::{
    backend::renderer::{gles::GlesRenderer, Renderer},
    reexports::wayland_server::{Display, DisplayHandle},
    wayland::compositor::CompositorState,
    wayland::shell::xdg::XdgShellState,
};
use std::sync::Arc;

fn main() {
    // Initialize the Wayland display
    let display = Display::new().unwrap();
    
    // Create our compositor state
    let state = State::new(display);
    
    // Initialize the event loop
    let event_loop = EventLoop::try_new().unwrap();
    let handle = event_loop.handle();
    
    // Set up the compositor
    let mut state = state;
    state.event_loop = Some(handle);
    
    // Start the compositor
    println!("Starting Nyarla DE compositor...");
    
    // Run the event loop (this will block)
    event_loop.run(None, &mut state, |_| {}).unwrap();
}