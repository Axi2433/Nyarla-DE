use smithay::{
    backend::renderer::{Frame, ImportAll, Renderer, Texture},
    delegate_compositor, delegate_seat, delegate_xdg_shell,
    input::{Seat, SeatHandler, SeatState},
    reexports::wayland_server::{
        backend::{ClientData, ClientId, DisconnectReason},
        protocol::{wl_seat, wl_surface::WlSurface},
        Client, DisplayHandle,
    },
    utils::{Point, Size},
    wayland::{
        compositor::{CompositorClientState, CompositorHandler, CompositorState},
        shell::xdg::{
            PopupSurface, PositionerState, ToplevelSurface, XdgShellHandler, XdgShellState,
        },
    },
};
use crate::renderer::visualizer::Visualizer;

pub struct CompositorManager {
    pub compositor_state: CompositorState,
    pub xdg_shell_state: XdgShellState,
    pub seat_state: SeatState<Self>,
    pub seat: Seat<Self>,
    pub visualizer: Visualizer,
    pub output_size: Size<i32, smithay::utils::Logical>,
}

#[derive(Default)]
pub struct ClientState {
    pub compositor_state: CompositorClientState,
}

impl ClientData for ClientState {
    fn initialized(&self, _client_id: ClientId) {}
    fn disconnected(&self, _client_id: ClientId, _reason: DisconnectReason) {}
}

impl CompositorManager {
    pub fn new(dh: &DisplayHandle) -> Self {
        let mut seat_state = SeatState::new();
        let seat = seat_state.new_wl_seat(dh, "nyarla-seat");

        CompositorManager {
            compositor_state: CompositorState::new::<Self>(dh),
            xdg_shell_state: XdgShellState::new::<Self>(dh),
            seat_state,
            seat,
            visualizer: Visualizer::new(),
            output_size: Size::from((1920, 1080)), // Default size
        }
    }
    
    pub fn set_output_size(&mut self, size: Size<i32, smithay::utils::Logical>) {
        self.output_size = size;
    }
    
    pub fn update(&mut self, delta_time: std::time::Duration) {
        self.visualizer.update(delta_time);
    }
    
    pub fn render<R: Renderer + ImportAll>(
        &mut self,
        renderer: &mut R,
        frame: &mut Frame<'_, R::TextureId>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Clear the frame
        frame.clear(
            [0.0, 0.0, 0.0, 1.0], // Black background
            &[],
        )?;
        
        // Render visual effects
        self.visualizer.render(renderer, frame, self.output_size)?;
        
        Ok(())
    }
    
    pub fn add_laser_edge(
        &mut self,
        start: Point<i32, smithay::utils::Logical>,
        end: Point<i32, smithay::utils::Logical>,
        width: i32,
    ) {
        self.visualizer.add_laser_edge(start, end, width);
    }
    
    pub fn add_crystal_effect(
        &mut self,
        position: Point<i32, smithay::utils::Logical>,
        max_size: Size<i32, smithay::utils::Logical>,
    ) {
        self.visualizer.add_crystal_effect(position, max_size);
    }
}

impl CompositorHandler for CompositorManager {
    fn compositor_state(&mut self) -> &mut CompositorState {
        &mut self.compositor_state
    }

    fn client_compositor_state<'a>(&self, client: &'a Client) -> &'a CompositorClientState {
        &client.get_data::<ClientState>().unwrap().compositor_state
    }

    fn commit(&mut self, _surface: &WlSurface) {}
}

impl SeatHandler for CompositorManager {
    type KeyboardFocus = WlSurface;
    type PointerFocus = WlSurface;
    type TouchFocus = WlSurface;

    fn seat_state(&mut self) -> &mut SeatState<Self> {
        &mut self.seat_state
    }
}

impl XdgShellHandler for CompositorManager {
    fn xdg_shell_state(&mut self) -> &mut XdgShellState {
        &mut self.xdg_shell_state
    }

    fn new_toplevel(&mut self, surface: ToplevelSurface) {
        println!("New toplevel surface created: {:?}", surface);
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

delegate_compositor!(CompositorManager);
delegate_seat!(CompositorManager);
delegate_xdg_shell!(CompositorManager);
