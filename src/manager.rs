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
    // add_window, focus_window... zostają bez zmian
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

    fn new_toplevel(&mut self, _surface: ToplevelSurface) {
        // TODO: tu utwórz Window i dodaj do self.windows
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
