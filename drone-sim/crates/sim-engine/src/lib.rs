
use wasm_bindgen::prelude::*;

use crate::math::Vec3;

mod arena;
mod general_objects;
mod vehicle;
mod projectile;
mod math;

#[wasm_bindgen]
pub struct Simulation {
    arena: arena::Arena,
    current_object_id: u64,
    objects: Vec<general_objects::MoveableObject>,
    tick_count: u32,
}


#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new(length: f32, width:f32)->Self{
        Self {
            arena: arena::Arena::new(length, width),
            objects: Vec::new(),
            tick_count: 0,
            current_object_id: 1,
        }
    }
    #[wasm_bindgen(getter)]
    pub fn height(&self)->f32{
        self.arena.height
    }
    #[wasm_bindgen(getter)]
    pub fn width(&self)->f32{
        self.arena.width
    }
    #[wasm_bindgen]
    pub fn add_vehicle(&mut self, vehicle_type: String, max_speed_fpt: f32, x: f32, y: f32, z: f32, heading: f32, length_feet: f32, width_feet: f32, height_feet: f32) -> u64{
        let state = vehicle::VehicleState {
            coordinates: math::Vec3 {
                x,
                y,
                z
            },
            movement_state: math::MovementState { 
                momentum_vector: Vec3{x:0.0, y:0.0, z:0.0},
                orientation_speed_vector: Vec3 { x:0.0, y:0.0, z:0.0 }
            }
        };
        let v_typ: vehicle::VehicleType;
        if vehicle_type.eq("drone"){
            v_typ = vehicle::VehicleType::Drone;
        } else {
            return 0;
        }
        self.objects.push(general_objects::MoveableObject::Vehicle(vehicle::Vehicle::new(self.current_object_id, v_typ, max_speed_fpt, state, length_feet, width_feet, height_feet, Vec::new())));
        self.current_object_id += 1;
        self.current_object_id - 1
    }
    fn return_objects_as_renderables(&self)->Vec<general_objects::RenderObject>{
        self.objects.iter().map(general_objects::MoveableObject::as_render_object).collect()
    }
    #[wasm_bindgen]
    pub fn tick(&mut self)->Vec<general_objects::RenderObject>{
        let mut changes: Vec<general_objects::ChangeEnum> = Vec::new();
        for object in self.objects.iter_mut(){
            changes.extend(object.tick());
        }
        self.objects.retain(|object| {
            match object{
                general_objects::MoveableObject::Vehicle(v) => {
                    true
                },
                general_objects::MoveableObject::Projectile(p)=> {
                    !(p.state.coordinates.x > self.arena.width || p.state.coordinates.x < 0.0 || p.state.coordinates.y > self.arena.height || p.state.coordinates.y < 0.)
                },
            }
        });
    
        for change in changes {
            match change {
                general_objects::ChangeEnum::NewObject{object} => self.objects.push(object),
                general_objects::ChangeEnum::Destruction{radius, coordinates} => {}
            }
        }

        self.tick_count += 1;
        self.return_objects_as_renderables()
    }

    pub fn add_circle_target(&mut self, object_identifier: u64, x: f32, y:f32, z:f32, circle_distance: f32, clockwise: bool){
        let location = math::Vec3{x, y, z};
        for object in self.objects.iter_mut() {
            match object{
                general_objects::MoveableObject::Vehicle(v) => {
                    if v.id != object_identifier{
                        continue;
                    }
                    v.instructions.push(
                        vehicle::VehicleInstructions{
                            objective:vehicle::VehicleObjective::CircleTarget { location, distance: circle_distance, clockwise}
                        }
                    );
                },
                _ => {}
            }
        }
    }

    pub fn add_go_to_location(&mut self, object_identifier: u64, x: f32, y:f32, z:f32){
        let location = math::Vec3{x, y, z};
        for object in self.objects.iter_mut() {
            match object{
                general_objects::MoveableObject::Vehicle(v) => {
                    if v.id != object_identifier{
                        continue;
                    }
                    v.instructions.push(
                        vehicle::VehicleInstructions{
                            objective:vehicle::VehicleObjective::GoToLocation { location }
                        }
                    );
                },
                _ => {}
            }
        }
    }

}



