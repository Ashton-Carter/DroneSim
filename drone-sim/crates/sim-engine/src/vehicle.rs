use crate::math;
use crate::general_objects;
use crate::math::Coordinates;
use crate::projectile;


pub enum VehicleObjective {
    GoToLocation{location: math::Coordinates},
    PatrolLocations{locations: Vec<math::Coordinates>, return_to: math::Coordinates},
    EngageTarget,
    HoldPosition{location: math::Coordinates},
    CircleTarget{location: math::Coordinates, distance: f32, clockwise: bool},
}

pub struct VehicleInstructions {
    pub objective: VehicleObjective,
}

pub struct VehicleState {
    pub coordinates: math::Coordinates,
    pub heading: f32,
    pub dx: f32,
    pub dy: f32,
    pub dz: f32,
    pub ddx: f32,
    pub ddy: f32,
    pub ddz: f32,
    pub length_feet: f32,
    pub width_feet: f32,
    pub height_feet: f32,
}

pub struct Vehicle {
    pub name: String,
    pub state: VehicleState,
    pub instructions: Vec<VehicleInstructions>,
}

impl Vehicle {
    pub fn new(name: String, state: VehicleState, instructions: Vec<VehicleInstructions>)->Self{
        Self {
            name,
            state,
            instructions
        }
    }
    pub fn tick(&mut self)-> Vec<general_objects::ChangeEnum>{
        let mut changes: Vec<general_objects::ChangeEnum> = Vec::new();

        for instruction in self.instructions.iter() {
            match instruction.objective {
                VehicleObjective::CircleTarget { location, distance, clockwise} => {
                    if self.state.coordinates.x < location.x {
                        
                    }
                }
                _ => {}
            }
        }

        let projectile_state = projectile::ProjectileState {
            coordinates: self.state.coordinates,
            speed_fps: 1000.0,
            dspeed_fps: -0.1,
            heading: self.state.heading,
            length_feet: 6.0,
            width_feet: 1.0,
            height_feet: 1.0,
        };
        changes.push(
            general_objects::ChangeEnum::NewObject{
                object: general_objects::MoveableObject::Projectile(
                    projectile::Projectile::new(
                        String::from("proj"), 
                        projectile_state)
                )
            }
        );
        changes
    }

}
