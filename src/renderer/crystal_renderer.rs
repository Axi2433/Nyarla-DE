use smithay::{
    backend::renderer::{Frame, ImportAll, Renderer, Texture},
    utils::{Point, Size},
};
use std::collections::HashMap;
use rand::Rng;

// Crystal effect for menu components
#[derive(Debug, Clone)]
pub struct CrystalEffect {
    pub position: Point<i32, smithay::utils::Logical>,
    pub size: Size<i32, smithay::utils::Logical>,
    pub color: [f32; 4],
    pub opacity: f32,
    pub scale: f32,
    pub rotation: f32,
    pub creation_time: std::time::Instant,
    pub duration: std::time::Duration,
    pub is_active: bool,
}

impl CrystalEffect {
    pub fn new(
        position: Point<i32, smithay::utils::Logical>,
        max_size: Size<i32, smithay::utils::Logical>,
    ) -> Self {
        let mut rng = rand::thread_rng();
        
        CrystalEffect {
            position,
            size: Size::from((
                rng.gen_range(10..=max_size.w / 2),
                rng.gen_range(10..=max_size.h / 2)
            )),
            color: [
                0.5 + rng.gen_range(0.0..=0.5), // Red component (more red than white)
                0.7 + rng.gen_range(0.0..=0.3), // Green component 
                0.9 + rng.gen_range(0.0..=0.1), // Blue component (more blue)
                0.8 + rng.gen_range(0.0..=0.2), // Alpha
            ],
            opacity: 1.0,
            scale: 0.5 + rng.gen_range(0.0..=1.0),
            rotation: rng.gen_range(0.0..=360.0),
            creation_time: std::time::Instant::now(),
            duration: std::time::Duration::from_millis(2000), // 2 seconds
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
        
        // Fade out over time
        let fade_factor = 1.0 - (elapsed.as_secs_f64() / self.duration.as_secs_f64());
        self.opacity = fade_factor as f32 * 0.8;
        
        // Slight scaling animation
        self.scale = (0.5 + (1.0 - fade_factor) * 0.5) as f32;
    }
    
    pub fn is_finished(&self) -> bool {
        !self.is_active || self.creation_time.elapsed() > self.duration
    }
}

pub struct CrystalRenderer {
    pub crystal_effects: Vec<CrystalEffect>,
}

impl CrystalRenderer {
    pub fn new() -> Self {
        CrystalRenderer {
            crystal_effects: Vec::new(),
        }
    }
    
    pub fn add_crystal_effect(
        &mut self,
        position: Point<i32, smithay::utils::Logical>,
        max_size: Size<i32, smithay::utils::Logical>,
    ) {
        self.crystal_effects.push(CrystalEffect::new(position, max_size));
    }
    
    pub fn update_effects(&mut self, delta_time: std::time::Duration) {
        for effect in &mut self.crystal_effects {
            effect.update(delta_time);
        }
        
        // Remove finished effects
        self.crystal_effects.retain(|effect| !effect.is_finished());
    }
    
    pub fn render<R: Renderer + ImportAll>(
        &mut self,
        renderer: &mut R,
        frame: &mut R::Frame<'_, '_>,
        output_size: Size<i32, smithay::utils::Logical>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Render crystal effects
        for effect in &self.crystal_effects {
            if effect.opacity > 0.0 {
                let color = effect.color;
                
                // Create a diamond-like shape with gradient effect
                frame.draw_solid(
                    dst_rect,                          // Rectangle<i32, Physical>: gdzie rysujesz
                    &[Rectangle::from_size(dst_rect.size)], // damage, względem dst
                    Color32F::new(r, g, b, a),         // kolor
                )?;
            }
        }
        
        Ok(())
    }
}