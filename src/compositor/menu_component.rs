use smithay::{
    backend::renderer::{Frame, ImportAll, Renderer, Texture},
    utils::{Point, Size},
};
use crate::renderer::{CrystalRenderer, LaserRenderer};

// Menu component with visual effects
#[derive(Debug)]
pub struct MenuComponent {
    pub position: Point<i32, smithay::utils::Logical>,
    pub size: Size<i32, smithay::utils::Logical>,
    pub label: String,
    pub is_active: bool,
    pub crystal_renderer: CrystalRenderer,
    pub laser_renderer: LaserRenderer,
}

impl MenuComponent {
    pub fn new(
        position: Point<i32, smithay::utils::Logical>,
        size: Size<i32, smithay::utils::Logical>,
        label: String,
    ) -> Self {
        MenuComponent {
            position,
            size,
            label,
            is_active: false,
            crystal_renderer: CrystalRenderer::new(),
            laser_renderer: LaserRenderer::new(),
        }
    }
    
    pub fn activate(&mut self) {
        self.is_active = true;
        // Add visual effects when activated
        self.crystal_renderer.add_crystal_effect(
            self.position.clone(),
            self.size.clone()
        );
        
        // Add laser edges around the component
        self.laser_renderer.add_laser_edge(
            self.position.clone(),
            Point::from((self.position.x + self.size.w, self.position.y)),
            2,
        );
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
        // Render the menu component
        let color = if self.is_active {
            [0.2, 0.4, 0.8, 0.9] // Blue when active
        } else {
            [0.1, 0.2, 0.4, 0.7] // Darker blue when inactive
        };
        
        frame.render_rect(
            self.position,
            self.size,
            color,
        )?;
        
        // Render visual effects
        self.crystal_renderer.render(renderer, frame, output_size)?;
        self.laser_renderer.render(renderer, frame, output_size)?;
        
        Ok(())
    }
}