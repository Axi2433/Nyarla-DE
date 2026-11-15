use rand::Rng;
use smithay::{
    backend::renderer::{Color32F, Frame, ImportAll, Renderer},
    utils::{Logical, Physical, Point, Rectangle, Size},
};

// Crystal effect for menu components
#[derive(Debug, Clone)]
pub struct CrystalEffect {
    pub position: Point<i32, Logical>,
    pub size: Size<i32, Logical>,
    pub color: [f32; 4],
    pub opacity: f32,
    pub scale: f32,
    pub rotation: f32,
    pub creation_time: std::time::Instant,
    pub duration: std::time::Duration,
    pub is_active: bool,
}

impl CrystalEffect {
    pub fn new(position: Point<i32, Logical>, max_size: Size<i32, Logical>) -> Self {
        let mut rng = rand::thread_rng();

        // gen_range panics na pustym zakresie, więc dolna granica to minimum 10
        let max_w = (max_size.w / 2).max(10);
        let max_h = (max_size.h / 2).max(10);

        CrystalEffect {
            position,
            size: Size::from((rng.gen_range(10..=max_w), rng.gen_range(10..=max_h))),
            color: [
                0.5 + rng.gen_range(0.0..=0.5), // R
                0.7 + rng.gen_range(0.0..=0.3), // G
                0.9 + rng.gen_range(0.0..=0.1), // B
                0.8 + rng.gen_range(0.0..=0.2), // A
            ],
            opacity: 1.0,
            scale: 0.5 + rng.gen_range(0.0..=1.0),
            rotation: rng.gen_range(0.0..=360.0),
            creation_time: std::time::Instant::now(),
            duration: std::time::Duration::from_millis(2000),
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
        self.opacity = fade_factor as f32 * 0.8;

        // Lekka animacja skali
        self.scale = (0.5 + (1.0 - fade_factor) * 0.5) as f32;
    }

    pub fn is_finished(&self) -> bool {
        !self.is_active || self.creation_time.elapsed() > self.duration
    }
}

#[derive(Debug)]
pub struct CrystalRenderer {
    pub crystal_effects: Vec<CrystalEffect>,
}

impl CrystalRenderer {
    pub fn new() -> Self {
        CrystalRenderer {
            crystal_effects: Vec::new(),
        }
    }

    pub fn add_crystal_effect(&mut self, position: Point<i32, Logical>, max_size: Size<i32, Logical>) {
        self.crystal_effects
            .push(CrystalEffect::new(position, max_size));
    }

    pub fn update_effects(&mut self, delta_time: std::time::Duration) {
        for effect in &mut self.crystal_effects {
            effect.update(delta_time);
        }

        // Usuń zakończone efekty
        self.crystal_effects.retain(|effect| !effect.is_finished());
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
        for effect in &self.crystal_effects {
            if effect.opacity <= 0.0 {
                continue;
            }

            let w = ((effect.size.w as f32) * effect.scale).max(1.0) as i32;
            let h = ((effect.size.h as f32) * effect.scale).max(1.0) as i32;

            let dst = Rectangle::<i32, Physical>::new(
                Point::from((effect.position.x, effect.position.y)),
                Size::from((w, h)),
            );

            // Smithay oczekuje koloru z premultiplikowaną alfą
            let [r, g, b, a] = effect.color;
            let alpha = (a * effect.opacity).clamp(0.0, 1.0);
            let color = Color32F::new(r * alpha, g * alpha, b * alpha, alpha);

            // Damage jest względem `dst`, czyli cały prostokąt
            frame.draw_solid(dst, &[Rectangle::from_size(dst.size)], color)?;
        }

        Ok(())
    }
}