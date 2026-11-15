use smithay::{
    backend::renderer::{Frame, ImportAll, Renderer},
    utils::{Point, Size},
};
use crate::renderer::{CrystalRenderer, LaserRenderer};

pub struct Visualizer {
    pub crystal_renderer: CrystalRenderer,
    pub laser_renderer: LaserRenderer,
}

impl Visualizer {
    pub fn new() -> Self {
        Visualizer {
            crystal_renderer: CrystalRenderer::new(),
            laser_renderer: LaserRenderer::new(),
        }
    }

    pub fn update(&mut self, delta_time: std::time::Duration) {
        self.crystal_renderer.update_effects(delta_time);
        self.laser_renderer.update_lasers(delta_time);
    }

    pub fn render<R: Renderer + ImportAll>(
        &mut self,
        renderer: &mut R,
        frame: &mut R::Frame<'_, '_>,
        output_size: Size<i32, smithay::utils::Logical>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Render visual effects
        self.crystal_renderer.render(renderer, frame, output_size)?;
        self.laser_renderer.render(renderer, frame, output_size)?;
        
        Ok(())
    }

    pub fn add_laser_edge(
        &mut self,
        start: Point<i32, smithay::utils::Logical>,
        end: Point<i32, smithay::utils::Logical>,
        width: i32,
    ) {
        self.laser_renderer.add_laser_edge(start, end, width as u32);
    }

    pub fn add_crystal_effect(
        &mut self,
        position: Point<i32, smithay::utils::Logical>,
        max_size: Size<i32, smithay::utils::Logical>,
    ) {
        self.crystal_renderer.add_crystal_effect(position, max_size);
    }
}