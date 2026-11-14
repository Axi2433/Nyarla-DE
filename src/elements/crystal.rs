use smithay::reexports::wayland_server::DisplayHandle;
use cgmath::Point2;

#[derive(Debug)]
pub struct CrystalElement {
    pub position: Point2<f64>,
    pub size: (f64, f64),
    pub is_active: bool,
    pub animation_progress: f32,
}

impl CrystalElement {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            position: Point2::new(x, y),
            size: (width, height),
            is_active: false,
            animation_progress: 0.0,
        }
    }
    
    pub fn activate(&mut self) {
        self.is_active = true;
        self.animation_progress = 0.0;
    }
    
    pub fn update_animation(&mut self) {
        if self.is_active && self.animation_progress < 1.0 {
            self.animation_progress += 0.02; // Adjust speed as needed
        }
    }
    
    pub fn is_ready(&self) -> bool {
        self.animation_progress >= 1.0
    }
}