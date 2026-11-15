use super::CrystalRenderer;
use smithay::{
    backend::renderer::{Color32F, Frame, ImportAll, Renderer},
    utils::{Logical, Physical, Point, Rectangle, Size},
};

// Magic laser edge effect for menu components
#[derive(Debug, Clone)]
pub struct MagicLaserEdge {
    pub start: Point<i32, Logical>,
    pub end: Point<i32, Logical>,
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
    pub fn new(start: Point<i32, Logical>, end: Point<i32, Logical>, width: i32) -> Self {
        MagicLaserEdge {
            start,
            end,
            width,
            color: [1.0, 0.5, 0.5, 1.0], // czerwony motyw magiczny
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

        // Pulsowanie
        self.pulse += self.pulse_speed * delta_time.as_secs_f32();
        if self.pulse > 1.0 {
            self.pulse = 0.0;
        }

        // Zanikanie w czasie
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

        // Wzmocnienie czerwieni; kanały obcięte do 1.0
        filtered[0] = (filtered[0] * self.red_tint).min(1.0);
        filtered[1] = (filtered[1] * self.saturation).min(1.0);
        filtered[2] = (filtered[2] * self.brightness).min(1.0);

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

    pub fn add_laser_edge(&mut self, start: Point<i32, Logical>, end: Point<i32, Logical>, width: i32) {
        if self.enabled {
            self.laser_edges.push(MagicLaserEdge::new(start, end, width));
        }
    }

    pub fn add_crystal_effect(&mut self, position: Point<i32, Logical>, max_size: Size<i32, Logical>) {
        if self.enabled {
            self.crystal_renderer.add_crystal_effect(position, max_size);
        }
    }

    pub fn update(&mut self, delta_time: std::time::Duration) {
        for edge in &mut self.laser_edges {
            edge.update(delta_time);
        }

        // Usuń zakończone krawędzie
        self.laser_edges.retain(|edge| !edge.is_finished());

        self.crystal_renderer.update_effects(delta_time);
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
        if !self.enabled {
            return Ok(());
        }

        for edge in &self.laser_edges {
            if edge.opacity <= 0.0 {
                continue;
            }

            let [r, g, b, a] = self.color_filter.apply_filter(edge.color);

            let dx = (edge.end.x - edge.start.x) as f64;
            let dy = (edge.end.y - edge.start.y) as f64;
            let length = (dx * dx + dy * dy).sqrt() as i32;

            // Uwaga: rysujemy prostokąt poziomy od punktu start. Linie ukośne
            // wymagałyby shadera lub rotacji, więc tu działa tylko poziomo.
            let dst = Rectangle::<i32, Physical>::new(
                Point::from((edge.start.x, edge.start.y)),
                Size::from((length.max(1), edge.width.max(1))),
            );

            // Premultiplikowana alfa
            let alpha = (a * edge.opacity).clamp(0.0, 1.0);
            let color = Color32F::new(r * alpha, g * alpha, b * alpha, alpha);

            frame.draw_solid(dst, &[Rectangle::from_size(dst.size)], color)?;
        }

        self.crystal_renderer.render(renderer, frame, output_size)?;

        Ok(())
    }
}