use smithay::{
    backend::renderer::{Frame, ImportAll, Renderer, Texture},
    utils::{Point, Size},
};

// Laser edge effect for menu components
#[derive(Debug, Clone)]
pub struct LaserEdge {
    pub start: Point<i32, smithay::utils::Logical>,
    pub end: Point<i32, smithay::utils::Logical>,
    pub width: u32,
    pub color: [f32; 4],
    pub opacity: f32,
    pub creation_time: std::time::Instant,
    pub duration: std::time::Duration,
    pub is_active: bool,
}

impl LaserEdge {
    pub fn new(
        start: Point<i32, smithay::utils::Logical>,
        end: Point<i32, smithay::utils::Logical>,
        width: u32,
    ) -> Self {
        LaserEdge {
            start,
            end,
            width,
            color: [1.0, 0.8, 0.2, 1.0], // Orange/yellow color
            opacity: 1.0,
            creation_time: std::time::Instant::now(),
            duration: std::time::Duration::from_millis(1000), // 1 second
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
        self.opacity = fade_factor as f32;
    }
    
    pub fn is_finished(&self) -> bool {
        !self.is_active || self.creation_time.elapsed() > self.duration
    }
}

pub struct LaserRenderer {
    pub laser_edges: Vec<LaserEdge>,
}

impl LaserRenderer {
    pub fn new() -> Self {
        LaserRenderer {
            laser_edges: Vec::new(),
        }
    }
    
    pub fn add_laser_edge(
        &mut self,
        start: Point<i32, smithay::utils::Logical>,
        end: Point<i32, smithay::utils::Logical>,
        width: u32,
    ) {
        self.laser_edges.push(LaserEdge::new(start, end, width));
    }
    
    pub fn update_lasers(&mut self, delta_time: std::time::Duration) {
        for edge in &mut self.laser_edges {
            edge.update(delta_time);
        }
        
        // Remove finished edges
        self.laser_edges.retain(|edge| !edge.is_finished());
    }
    
    pub fn render<R: Renderer + ImportAll>(
        &mut self,
        renderer: &mut R,
        frame: &mut Frame<'_, R::TextureId>,
        output_size: Size<i32, smithay::utils::Logical>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Render laser edges
        for edge in &self.laser_edges {
            if edge.opacity > 0.0 {
                let color = edge.color;
                
                // Create a line effect with gradient
                frame.render_rect(
                    edge.start,
                    Size::from((edge.width as i32, 1)), // Very thin rectangle to represent line
                    color,
                )?;
            }
        }
        
        Ok(())
    }
}