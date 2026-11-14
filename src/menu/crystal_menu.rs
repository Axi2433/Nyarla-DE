use smithay::reexports::wayland_server::DisplayHandle;

#[derive(Debug)]
pub struct CrystalMenu {
    pub is_open: bool,
    pub items: Vec<MenuItem>,
    pub animation_progress: f32,
}

#[derive(Debug)]
pub struct MenuItem {
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub size: (f64, f64),
    pub is_selected: bool,
}

impl CrystalMenu {
    pub fn new() -> Self {
        Self {
            is_open: false,
            items: Vec::new(),
            animation_progress: 0.0,
        }
    }
    
    pub fn open(&mut self) {
        self.is_open = true;
        self.animation_progress = 0.0;
    }
    
    pub fn close(&mut self) {
        self.is_open = false;
        self.animation_progress = 1.0;
    }
    
    pub fn add_item(&mut self, name: &str, x: f64, y: f64, width: f64, height: f64) {
        let item = MenuItem {
            name: name.to_string(),
            x,
            y,
            size: (width, height),
            is_selected: false,
        };
        self.items.push(item);
    }
    
    pub fn update_animation(&mut self) {
        if self.is_open && self.animation_progress < 1.0 {
            self.animation_progress += 0.05; // Adjust speed as needed
        } else if !self.is_open && self.animation_progress > 0.0 {
            self.animation_progress -= 0.05;
        }
    }
}