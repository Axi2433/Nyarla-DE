pub struct ShellInterface {
    // Shell interface components
    pub time_display: String,
    pub floating_elements: Vec<FloatingElement>,
}

#[derive(Debug)]
pub struct FloatingElement {
    pub x: f64,
    pub y: f64,
    pub size: (f64, f64),
    pub animation_offset: f64,
}

impl ShellInterface {
    pub fn new() -> Self {
        Self {
            time_display: String::from("00:00"),
            floating_elements: Vec::new(),
        }
    }
    
    pub fn update_time(&mut self, time: &str) {
        self.time_display = time.to_string();
    }
    
    pub fn add_floating_element(&mut self, x: f64, y: f64, width: f64, height: f64) {
        let element = FloatingElement {
            x,
            y,
            size: (width, height),
            animation_offset: 0.0,
        };
        self.floating_elements.push(element);
    }
}