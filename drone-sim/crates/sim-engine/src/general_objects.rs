use wasm_bindgen::prelude::*;
use crate::vehicle;
use crate::projectile;
use crate::math;


#[wasm_bindgen]
pub struct RenderObject {
    object_type: String,
    id: u64,
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
    pub fn new(object_type: String, id: u64, x: f32, y:f32, z:f32, heading: f32, length_feet: f32, width_feet: f32, height_feet: f32)->Self{
        Self {
            object_type,
            id,
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
    pub fn object_type(&self)->String{
        self.object_type.clone()
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
    Destruction{radius: f32, coordinates: math::Vec3},
    NewObject{object: MoveableObject},
}


impl MoveableObject {
    pub fn as_render_object(&self)->RenderObject{
        match self {
            MoveableObject::Vehicle(v) => {
                let heading = math::heading(
                    v.state.movement_state.orientation_speed_vector.x, 
                    v.state.movement_state.orientation_speed_vector.y
                );
            
                RenderObject::new(
                    v.vehicle_type.to_string(), 
                    v.id, 
                    v.state.coordinates.x, 
                    v.state.coordinates.y, 
                    v.state.coordinates.z, 
                    heading, 
                    v.length_feet, 
                    v.width_feet, 
                    v.height_feet
                )
            },
            MoveableObject::Projectile(p) => 
            {
                let heading = math::heading(
                    p.state.movement_state.orientation_speed_vector.x, 
                    p.state.movement_state.orientation_speed_vector.y
                );
                RenderObject::new(
                    p.projectile_type.to_string(), 
                    p.id, 
                    p.state.coordinates.x, 
                    p.state.coordinates.y, 
                    p.state.coordinates.z, 
                    heading, 
                    p.length_feet, 
                    p.width_feet, 
                    p.height_feet
                )               
            }
            ,
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
