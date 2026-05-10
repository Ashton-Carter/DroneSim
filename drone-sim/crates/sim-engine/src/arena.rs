pub struct Arena {
    pub height: f32,
    pub width: f32,
}

impl Arena {
    pub fn new(height: f32, width:f32)->Self{
        Self {
            height,
            width,
        }
    }
}