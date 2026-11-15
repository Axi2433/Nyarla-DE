use smithay::{
    backend::renderer::{Frame, ImportAll, Renderer, Texture},
    utils::{Point, Size},
};
use std::collections::HashMap;

// Magic laser edge effect for menu components
#[derive(Debug, Clone)]
pub struct MagicLaserEdge {
    pub start: Point<i32, smithay::utils::Logical>,
    pub end: Point<i32, smithay::utils::Logical>,
    pub width: i32,
    pub color: [f32; 4],
    pub opacity: f32,
    pub pulse: f32,
    pub pulse_speed: f32,
    pub creation_time: std::time::Instant,
    pub duration: std::time::Duration,
    pub is_active: bool,
}

impl MagicLaserEdge {
    pub fn new(
        start: Point<i32, smithay::utils::Logical>,
        end: Point<i32, smithay::utils::Logical>,
        width: i32,
    ) -> Self {
        MagicLaserEdge {
            start,
            end,
            width,
            color: [1.0, 0.5, 0.5, 1.0], // Red magic theme
            opacity: 1.0,
            pulse: 0.0,
            pulse_speed: 2.0,
            creation_time: std::time::Instant::now(),
            duration: std::time::Duration::from_millis(1000),
            is_active: true,
        }
    }
    
    pub fn update(&mut self, delta_time: std::time::Duration) {
        if !self.is_active {
            return;
        }
        
        let elapsed = self.creation_time.elapsed();
        if elapsed > self.duration {
            self.is_active = false;
            return;
        }
        
        // Update pulse effect
        self.pulse += self.pulse_speed * delta_time.as_secs_f64() as f32;
        if self.pulse > 1.0 {
            self.pulse = 0.0;
        }
        
        // Fade out over time
        let fade_factor = 1.0 - (elapsed.as_secs_f64() / self.duration.as_secs_f64());
        self.opacity = fade_factor as f32 * 0.8;
    }
    
    pub fn is_finished(&self) -> bool {
        !self.is_active || self.creation_time.elapsed() > self.duration
    }
}

// Color filtering for red magical theme
pub struct ColorFilter {
    pub red_tint: f32,
    pub saturation: f32,
    pub brightness: f32,
}

impl ColorFilter {
    pub fn new() -> Self {
        ColorFilter {
            red_tint: 1.0,
            saturation: 1.0,
            brightness: 1.0,
        }
    }
    
    pub fn set_red_theme(&mut self) {
        self.red_tint = 1.5;
        self.saturation = 1.2;
        self.brightness = 1.1;
    }
    
    pub fn apply_filter(&self, color: [f32; 4]) -> [f32; 4] {
        let mut filtered = color;
        
        // Enhance red tones
        filtered[0] = (filtered[0] * self.red_tint).min(1.0);
        filtered[1] = filtered[1] * self.saturation;
        filtered[2] = filtered[2] * self.brightness;
        
        filtered
    }
}

pub struct Visualizer {
    pub laser_edges: Vec<MagicLaserEdge>,
    pub crystal_renderer: CrystalRenderer,
    pub color_filter: ColorFilter,
    pub enabled: bool,
}

impl Visualizer {
    pub fn new() -> Self {
        Visualizer {
            laser_edges: Vec::new(),
            crystal_renderer: CrystalRenderer::new(),
            color_filter: ColorFilter::new(),
            enabled: true,
        }
    }
    
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    
    pub fn add_laser_edge(
        &mut self,
        start: Point<i32, smithay::utils::Logical>,
        end: Point<i32, smithay::utils::Logical>,
        width: i32,
    ) {
        if self.enabled {
            self.laser_edges.push(MagicLaserEdge::new(start, end, width));
        }
    }
    
    pub fn add_crystal_effect(
        &mut self,
        position: Point<i32, smithay::utils::Logical>,
        max_size: Size<i32, smithay::utils::Logical>,
    ) {
        if self.enabled {
            self.crystal_renderer.add_crystal_effect(position, max_size);
        }
    }
    
    pub fn update(&mut self, delta_time: std::time::Duration) {
        // Update laser edges
        for edge in &mut self.laser_edges {
            edge.update(delta_time);
        }
        
        // Remove finished edges
        self.laser_edges.retain(|edge| !edge.is_finished());
        
        // Update crystal effects
        self.crystal_renderer.update_effects(delta_time);
    }
    
    pub fn render<R: Renderer + ImportAll>(
        &mut self,
        renderer: &mut R,
        frame: &mut R::Frame<'_, '_>,
        output_size: Size<i32, smithay::utils::Logical>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !self.enabled {
            return Ok(());
        }
        
        // Render magic laser edges
        for edge in &self.laser_edges {
            if edge.opacity > 0.0 {
                let color = self.color_filter.apply_filter(edge.color);
                
                // Create a gradient effect for the laser edge
                let length = ((edge.end.x as f64 - edge.start.x as f64).powi(2) + 
                             (edge.end.y as f64 - edge.start.y as f64).powi(2)).sqrt();
                
                // Render a line with glow effect
              frame.draw_solid(
                dst_rect,                          // Rectangle<i32, Physical>: gdzie rysujesz
                &[Rectangle::from_size(dst_rect.size)], // damage, względem dst
                Color32F::new(r, g, b, a),         // kolor
            )?;
            }
        }
        
        // Render crystal effects
        self.crystal_renderer.render(renderer, frame, output_size)?;
        
        Ok(())
    }
}