
use wasm_bindgen::prelude::*;

mod arena;
mod general_objects;
mod vehicle;
mod projectile;
mod math;

#[wasm_bindgen]
pub struct Simulation {
    arena: arena::Arena,
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
    pub fn add_vehicle(&mut self, name:String, x: f32, y: f32, z: f32, heading: f32, length_feet: f32, width_feet: f32, height_feet: f32){
        let state = vehicle::VehicleState {
            coordinates: math::Coordinates {
                x,
                y,
                z
            },
            heading,
            length_feet,
            width_feet,
            height_feet
        };
        self.objects.push(general_objects::MoveableObject::Vehicle(vehicle::Vehicle::new(name, state, Vec::new())));
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

    pub fn add_circle_target(&mut self, vehicle_identifier: &str, x: f32, y:f32, z:f32, circle_distance: f32, clockwise: bool){
        let location = math::Coordinates{x, y, z};
        for object in self.objects.iter_mut() {
            match object{
                general_objects::MoveableObject::Vehicle(v) => {
                    if !v.name.eq(vehicle_identifier){
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

}



