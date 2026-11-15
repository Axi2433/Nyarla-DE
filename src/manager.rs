use std::collections::HashMap;

use smithay::{
    delegate_compositor, delegate_seat, delegate_xdg_shell,
    input::{Seat, SeatHandler, SeatState},
    reexports::wayland_server::{
        backend::{ClientData, ClientId, DisconnectReason},
        protocol::{wl_seat, wl_surface::WlSurface},
        Client, DisplayHandle,
    },
    utils::Serial,
    wayland::{
        compositor::{CompositorClientState, CompositorHandler, CompositorState},
        shell::xdg::{
            PopupSurface, PositionerState, ToplevelSurface, XdgShellHandler, XdgShellState,
        },
    },
};

pub struct WindowManager {
    pub compositor_state: CompositorState,
    pub xdg_shell_state: XdgShellState,
    pub seat_state: SeatState<Self>,
    pub seat: Seat<Self>,
    pub windows: HashMap<u32, Window>,
    pub focused_window: Option<u32>,
}

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

#[derive(Default)]
pub struct ClientState {
    pub compositor_state: CompositorClientState,
}

impl ClientData for ClientState {
    fn initialized(&self, _client_id: ClientId) {}
    fn disconnected(&self, _client_id: ClientId, _reason: DisconnectReason) {}
}

impl WindowManager {
    pub fn new(dh: &DisplayHandle) -> Self {
        let mut seat_state = SeatState::new();
        let seat = seat_state.new_wl_seat(dh, "nyarla-seat");

        WindowManager {
            compositor_state: CompositorState::new::<Self>(dh),
            xdg_shell_state: XdgShellState::new::<Self>(dh),
            seat_state,
            seat,
            windows: HashMap::new(),
            focused_window: None,
        }
    }

    pub fn add_window(&mut self, id: u32, title: String, x: i32, y: i32, width: u32, height: u32) {
        self.windows.insert(
            id,
            Window {
                id,
                title,
                is_focused: false,
                is_minimized: false,
                is_maximized: false,
                x,
                y,
                width,
                height,
            },
        );
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

impl CompositorHandler for WindowManager {
    fn compositor_state(&mut self) -> &mut CompositorState {
        &mut self.compositor_state
    }

    fn client_compositor_state<'a>(&self, client: &'a Client) -> &'a CompositorClientState {
        &client.get_data::<ClientState>().unwrap().compositor_state
    }

    fn commit(&mut self, _surface: &WlSurface) {}
}

impl SeatHandler for WindowManager {
    type KeyboardFocus = WlSurface;
    type PointerFocus = WlSurface;
    type TouchFocus = WlSurface;

    fn seat_state(&mut self) -> &mut SeatState<Self> {
        &mut self.seat_state
    }
}

impl XdgShellHandler for WindowManager {
    fn xdg_shell_state(&mut self) -> &mut XdgShellState {
        &mut self.xdg_shell_state
    }

    fn new_toplevel(&mut self, surface: ToplevelSurface) {
        println!("New toplevel surface created: {:?}", surface);
        // TODO: utwórz Window i dodaj do self.windows
    }

    fn new_popup(&mut self, _surface: PopupSurface, _positioner: PositionerState) {}

    fn grab(&mut self, _surface: PopupSurface, _seat: wl_seat::WlSeat, _serial: Serial) {}

    fn reposition_request(
        &mut self,
        _surface: PopupSurface,
        _positioner: PositionerState,
        _token: u32,
    ) {
    }
}

delegate_compositor!(WindowManager);
delegate_seat!(WindowManager);
delegate_xdg_shell!(WindowManager);
