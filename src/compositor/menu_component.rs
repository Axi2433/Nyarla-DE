use crate::renderer::{CrystalRenderer, LaserRenderer};
use smithay::{
    backend::renderer::{Color32F, Frame, ImportAll, Renderer},
    utils::{Logical, Physical, Point, Rectangle, Size},
};

// Menu component with visual effects
#[derive(Debug)]
pub struct MenuComponent {
    pub position: Point<i32, Logical>,
    pub size: Size<i32, Logical>,
    pub label: String,
    pub is_active: bool,
    pub crystal_renderer: CrystalRenderer,
    pub laser_renderer: LaserRenderer, // wymaga #[derive(Debug)] w laser_renderer.rs
}

impl MenuComponent {
    pub fn new(position: Point<i32, Logical>, size: Size<i32, Logical>, label: String) -> Self {
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

        // Efekt kryształów po aktywacji
        self.crystal_renderer
            .add_crystal_effect(self.position, self.size);

        // Laserowa krawędź u góry komponentu
        self.laser_renderer.add_laser_edge(
            self.position,
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
        output_size: Size<i32, Logical>,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        R::Error: 'static,
    {
        let [r, g, b, a] = if self.is_active {
            [0.2, 0.4, 0.8, 0.9] // niebieski, aktywny
        } else {
            [0.1, 0.2, 0.4, 0.7] // ciemniejszy, nieaktywny
        };

        let dst = Rectangle::<i32, Physical>::new(
            Point::from((self.position.x, self.position.y)),
            Size::from((self.size.w, self.size.h)),
        );

        // Premultiplikowana alfa
        let color = Color32F::new(r * a, g * a, b * a, a);
        frame.draw_solid(dst, &[Rectangle::from_size(dst.size)], color)?;

        // Efekty wizualne
        self.crystal_renderer.render(renderer, frame, output_size)?;
        self.laser_renderer.render(renderer, frame, output_size)?;

        Ok(())
    }
}