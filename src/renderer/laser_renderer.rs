use smithay::{
    backend::renderer::{Color32F, Frame, ImportAll, Renderer},
    utils::{Logical, Physical, Point, Rectangle, Size},
};

// Laser edge effect for menu components
#[derive(Debug, Clone)]
pub struct LaserEdge {
    pub start: Point<i32, Logical>,
    pub end: Point<i32, Logical>,
    pub width: u32,
    pub color: [f32; 4],
    pub opacity: f32,
    pub creation_time: std::time::Instant,
    pub duration: std::time::Duration,
    pub is_active: bool,
}

impl LaserEdge {
    pub fn new(start: Point<i32, Logical>, end: Point<i32, Logical>, width: u32) -> Self {
        LaserEdge {
            start,
            end,
            width,
            color: [1.0, 0.8, 0.2, 1.0], // pomarańczowo-żółty
            opacity: 1.0,
            creation_time: std::time::Instant::now(),
            duration: std::time::Duration::from_millis(1000),
            is_active: true,
        }
    }

    pub fn update(&mut self, _delta_time: std::time::Duration) {
        if !self.is_active {
            return;
        }

        let elapsed = self.creation_time.elapsed();
        if elapsed > self.duration {
            self.is_active = false;
            return;
        }

        // Zanikanie w czasie
        let fade_factor = 1.0 - (elapsed.as_secs_f64() / self.duration.as_secs_f64());
        self.opacity = fade_factor as f32;
    }

    pub fn is_finished(&self) -> bool {
        !self.is_active || self.creation_time.elapsed() > self.duration
    }
}

#[derive(Debug)]
pub struct LaserRenderer {
    pub laser_edges: Vec<LaserEdge>,
}

impl LaserRenderer {
    pub fn new() -> Self {
        LaserRenderer {
            laser_edges: Vec::new(),
        }
    }

    pub fn add_laser_edge(&mut self, start: Point<i32, Logical>, end: Point<i32, Logical>, width: u32) {
        self.laser_edges.push(LaserEdge::new(start, end, width));
    }

    pub fn update_lasers(&mut self, delta_time: std::time::Duration) {
        for edge in &mut self.laser_edges {
            edge.update(delta_time);
        }

        // Usuń zakończone krawędzie
        self.laser_edges.retain(|edge| !edge.is_finished());
    }

    pub fn render<R: Renderer + ImportAll>(
        &mut self,
        _renderer: &mut R,
        frame: &mut R::Frame<'_, '_>,
        _output_size: Size<i32, Logical>,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        R::Error: 'static,
    {
        for edge in &self.laser_edges {
            if edge.opacity <= 0.0 {
                continue;
            }

            let dx = (edge.end.x - edge.start.x).abs();
            let dy = (edge.end.y - edge.start.y).abs();
            let thickness = (edge.width as i32).max(1);

            // Linie osiowe: poziome lub pionowe (ukośne wymagałyby shadera)
            let (x, y, w, h) = if dx >= dy {
                (edge.start.x.min(edge.end.x), edge.start.y, dx.max(1), thickness)
            } else {
                (edge.start.x, edge.start.y.min(edge.end.y), thickness, dy.max(1))
            };

            let dst = Rectangle::<i32, Physical>::new(Point::from((x, y)), Size::from((w, h)));

            // Premultiplikowana alfa
            let [r, g, b, a] = edge.color;
            let alpha = (a * edge.opacity).clamp(0.0, 1.0);
            let color = Color32F::new(r * alpha, g * alpha, b * alpha, alpha);

            frame.draw_solid(dst, &[Rectangle::from_size(dst.size)], color)?;
        }

        Ok(())
    }
}