//! Visualizer module for the Nyarla DE compositor
use super::CrystalRenderer;
use smithay::backend::renderer::{gles::GlesRenderer, Renderer};
use std::collections::HashMap;
use crate::renderer::crystal_renderer::CrystalRenderer;

/// Visualization configuration
#[derive(Debug, Clone)]
pub struct VisualizerConfig {
    pub width: u32,
    pub height: u32,
    pub fps: u32,
}

/// Visualizer state
pub struct VisualizerState {
    pub config: VisualizerConfig,
    pub renderers: HashMap<String, Box<dyn Renderer>>,
    pub is_active: bool,
}

impl VisualizerState {
    pub fn new(config: VisualizerConfig) -> Self {
        VisualizerState {
            config,
            renderers: HashMap::new(),
            is_active: false,
        }
    }

    pub fn enable(&mut self) {
        self.is_active = true;
    }

    pub fn disable(&mut self) {
        self.is_active = false;
    }

    pub fn update(&mut self) {
        if self.is_active {
            // Update visualization logic
        }
    }

    pub fn render<R: Renderer>(&self, renderer: &mut R) -> Result<(), Box<dyn std::error::Error>> {
        if self.is_active {
            // Render visualization using the provided renderer
        }
        Ok(())
    }
}