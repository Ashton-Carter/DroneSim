use crate::math;
use crate::general_objects;

pub struct ProjectileState{
    pub coordinates: math::Coordinates,
    pub speed_fps: f32,
    pub dspeed_fps: f32,
    pub heading: f32,
    pub length_feet: f32,
    pub width_feet: f32,
    pub height_feet: f32,
}

pub struct  Projectile {
    pub name: String,
    pub state: ProjectileState,
    
}

impl Projectile {
    pub fn new(name: String, state: ProjectileState)->Self{
        Self {
            name,
            state
        }
    }
    pub fn tick(&mut self)-> Vec<general_objects::ChangeEnum>{
        let changes: Vec<general_objects::ChangeEnum> = Vec::new();
        let (dx, dy) = math::translate_heading_speed_to_miles(self.state.heading, self.state.speed_fps);
        self.state.coordinates.x += dx;
        self.state.coordinates.y += dy;
        self.state.speed_fps += self.state.dspeed_fps;
        changes
    }
}
