mod compositor;
mod renderer;

use std::{sync::Arc, time::Duration};

use smithay::reexports::calloop::EventLoop;
use smithay::reexports::wayland_server::Display;
use smithay::wayland::socket::ListeningSocketSource;
use smithay::backend::renderer::{Frame, ImportAll, Renderer, Texture};
use smithay::utils::{Point, Size};

use compositor::manager::CompositorManager;
use compositor::menu_component::MenuComponent;

fn main() {
    let mut display: Display<CompositorManager> = Display::new().unwrap();
    let mut dh = display.handle();

    let mut state = CompositorManager::new(&dh);
    
    // Add a sample menu component
    let menu_component = MenuComponent::new(
        Point::from((100, 100)),
        Size::from((200, 50)),
        "Sample Menu".to_string(),
    );
    
    state.add_menu_component(menu_component);

    let mut event_loop: EventLoop<CompositorManager> = EventLoop::try_new().unwrap();

    let socket = ListeningSocketSource::new_auto().unwrap();
    let socket_name = socket.socket_name().to_os_string();

    event_loop
    .handle()
    .insert_source(socket, move |stream, _, _state| {
        dh.insert_client(stream, Arc::new(ClientState::default()))
        .unwrap();
    })
    .unwrap();

    println!("Starting Nyarla DE compositor on {:?}...", socket_name);

    loop {
        event_loop
        .dispatch(Some(Duration::from_millis(16)), &mut state)
        .unwrap();
        display.dispatch_clients(&mut state).unwrap();
        display.flush_clients().unwrap();
    }
}
