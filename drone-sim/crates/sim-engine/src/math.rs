#[derive(Clone, Copy)]
pub struct Coordinates {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}


pub fn translate_heading_speed_to_miles(heading: f32, speed_fps: f32)->(f32, f32){
    let degrees = (heading + 90.0)%360.0;
    let miles = feet_to_miles(speed_fps);
    let dy = degrees.to_radians().sin() * miles;
    let dx = degrees.to_radians().cos() * miles;
    (dx, dy)
}

pub fn feet_to_miles(feet: f32)-> f32 {
    feet/5280.0
}