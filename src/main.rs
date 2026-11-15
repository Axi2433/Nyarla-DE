mod compositor;

use std::{sync::Arc, time::Duration};

use smithay::reexports::calloop::EventLoop;
use smithay::reexports::wayland_server::Display;
use smithay::wayland::socket::ListeningSocketSource;

use compositor::manager::{ClientState, WindowManager};

fn main() {
    let mut display: Display<WindowManager> = Display::new().unwrap();
    let mut dh = display.handle();

    let mut state = WindowManager::new(&dh);

    let mut event_loop: EventLoop<WindowManager> = EventLoop::try_new().unwrap();

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
