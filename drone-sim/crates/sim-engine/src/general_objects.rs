use wasm_bindgen::prelude::*;
use crate::vehicle;
use crate::projectile;
use crate::math;

#[wasm_bindgen]
pub struct RenderObject {
    name: String,
    x: f32,
    y: f32,
    z: f32,
    heading: f32,
    length_feet: f32,
    width_feet: f32,
    height_feet: f32,
}

#[wasm_bindgen]
impl RenderObject {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, x: f32, y:f32, z:f32, heading: f32, length_feet: f32, width_feet: f32, height_feet: f32)->Self{
        Self {
            name,
            x,
            y,
            z,
            heading,
            length_feet,
            width_feet,
            height_feet,
        }
    }
    #[wasm_bindgen(getter)]
    pub fn name(&self)->String{
        self.name.clone()
    }
     #[wasm_bindgen(getter)]
    pub fn x(&self)->f32{
        self.x
    }
     #[wasm_bindgen(getter)]
    pub fn y(&self)->f32{
        self.y
    }
     #[wasm_bindgen(getter)]
    pub fn z(&self)->f32{
        self.z
    }
     #[wasm_bindgen(getter)]
    pub fn heading(&self)->f32{
        self.heading
    }
     #[wasm_bindgen(getter)]
    pub fn length_feet(&self)->f32{
        self.length_feet
    }
     #[wasm_bindgen(getter)]
    pub fn width_feet(&self)->f32{
        self.width_feet
    }
     #[wasm_bindgen(getter)]
    pub fn height_feet(&self)->f32{
        self.height_feet
    }
}


pub enum MoveableObject {
    Vehicle(vehicle::Vehicle),
    Projectile(projectile::Projectile),
}

pub enum ChangeEnum {
    Destruction{radius: f32, coordinates: math::Coordinates},
    NewObject{object: MoveableObject},
}


impl MoveableObject {
    pub fn as_render_object(&self)->RenderObject{
        match self {
            MoveableObject::Vehicle(v) => RenderObject::new(v.name.clone(), v.state.coordinates.x, v.state.coordinates.y, v.state.coordinates.z, v.state.heading, v.state.length_feet, v.state.width_feet, v.state.height_feet ),
            MoveableObject::Projectile(p) => RenderObject::new(p.name.clone(), p.state.coordinates.x, p.state.coordinates.y, p.state.coordinates.z, p.state.heading, p.state.length_feet, p.state.width_feet, p.state.height_feet),
        }
    }
    pub fn tick(&mut self)-> Vec<ChangeEnum> {
        let mut changes: Vec<ChangeEnum> = Vec::new();
        match self {
            MoveableObject::Vehicle(v) => changes.extend(v.tick()),
            MoveableObject::Projectile(p) => changes.extend(p.tick()),
        }
        changes
    }
}
