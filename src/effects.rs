use cgmath::{Vector2, Vector3};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Crystal {
    pub id: u32,
    pub position: Vector2<f64>,
    pub color: [f32; 4],
    pub size: f64,
    pub is_active: bool,
}

#[derive(Debug, Clone)]
pub struct LaserConnection {
    pub id: u32,
    pub start_crystal_id: u32,
    pub end_crystal_id: u32,
    pub points: Vec<Vector2<f64>>,
    pub color: [f32; 4],
    pub intensity: f32,
}

#[derive(Debug, Clone)]
pub struct CrystalFragment {
    pub id: u32,
    pub position: Vector2<f64>,
    pub velocity: Vector2<f64>,
    pub color: [f32; 4],
    pub size: f64,
    pub lifetime: f64,
}

pub struct EffectsState {
    pub crystals: HashMap<u32, Crystal>,
    pub laser_connections: HashMap<u32, LaserConnection>,
    pub fragments: Vec<CrystalFragment>,
    pub animation_time: f64,
}

impl EffectsState {
    pub fn new() -> Self {
        EffectsState {
            crystals: HashMap::new(),
            laser_connections: HashMap::new(),
            fragments: Vec::new(),
            animation_time: 0.0,
        }
    }

    pub fn add_crystal(&mut self, id: u32, position: Vector2<f64>, color: [f32; 4], size: f64) {
        let crystal = Crystal {
            id,
            position,
            color,
            size,
            is_active: true,
        };
        self.crystals.insert(id, crystal);
    }

    pub fn add_laser_connection(&mut self, id: u32, start_crystal_id: u32, end_crystal_id: u32, points: Vec<Vector2<f64>>, color: [f32; 4], intensity: f32) {
        let connection = LaserConnection {
            id,
            start_crystal_id,
            end_crystal_id,
            points,
            color,
            intensity,
        };
        self.laser_connections.insert(id, connection);
    }

    pub fn add_fragment(&mut self, fragment: CrystalFragment) {
        self.fragments.push(fragment);
    }

    pub fn update(&mut self, delta_time: f64) {
        self.animation_time += delta_time;
        
        // Update fragments
        self.fragments.retain(|fragment| {
            let new_position = Vector2::new(
                fragment.position.x + fragment.velocity.x * delta_time,
                fragment.position.y + fragment.velocity.y * delta_time,
            );
            
            let new_lifetime = fragment.lifetime - delta_time;
            
            // Update fragment position and lifetime
            // In a real implementation, we would also update velocity due to physics
            
            if new_lifetime > 0.0 {
                true
            } else {
                false
            }
        });
    }
}