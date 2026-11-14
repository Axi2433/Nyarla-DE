use cgmath::Point2;

#[derive(Debug)]
pub struct LaserConnection {
    pub start: Point2<f64>,
    pub end: Point2<f64>,
    pub is_active: bool,
    pub opacity: f32,
    pub animation_progress: f32,
}

impl LaserConnection {
    pub fn new(start_x: f64, start_y: f64, end_x: f64, end_y: f64) -> Self {
        Self {
            start: Point2::new(start_x, start_y),
            end: Point2::new(end_x, end_y),
            is_active: false,
            opacity: 0.0,
            animation_progress: 0.0,
        }
    }
    
    pub fn activate(&mut self) {
        self.is_active = true;
        self.opacity = 0.0;
        self.animation_progress = 0.0;
    }
    
    pub fn update_animation(&mut self) {
        if self.is_active && self.animation_progress < 1.0 {
            self.animation_progress += 0.03; // Adjust speed as needed
            self.opacity = self.animation_progress.min(1.0);
        } else if !self.is_active && self.animation_progress > 0.0 {
            self.animation_progress -= 0.03;
            self.opacity = self.animation_progress.max(0.0);
        }
    }
}