use crate::math;
use crate::general_objects;
use crate::projectile;

enum VehicleObjective {
    GoToLocation,
    PatrolLocation,
    EngageTarget,
    HoldPosition,
    CircleTarget,
}

pub struct VehicleInstructions {
    objective: VehicleObjective,
}

pub struct VehicleState {
    pub coordinates: math::Coordinates,
    pub heading: f32,
    pub length_feet: f32,
    pub width_feet: f32,
    pub height_feet: f32,
}

pub struct Vehicle {
    pub name: String,
    pub state: VehicleState,
    instructions: Vec<VehicleInstructions>,
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
