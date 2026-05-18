use crate::math;
use crate::general_objects;


pub enum VehicleObjective {
    GoToLocation{location: math::Vec3},
    PatrolLocations{locations: Vec<math::Vec3>, return_to: math::Vec3},
    EngageTarget,
    HoldPosition{location: math::Vec3},
    CircleTarget{location: math::Vec3, distance: f32, clockwise: bool},
}

pub enum VehicleType {
    Drone
}
impl VehicleType {
    pub fn to_string(&self)-> String{
        match self {
            VehicleType::Drone => String::from("drone")
        }
    }
}

pub struct VehicleInstructions {
    pub objective: VehicleObjective,
}

pub struct VehicleState {
    pub coordinates: math::Vec3,
    pub movement_state: math::MovementState,
}

pub struct Vehicle {
    pub id: u64,
    pub vehicle_type: VehicleType,
    pub max_speed_fpt: f32,
    pub state: VehicleState,
    pub length_feet: f32,
    pub width_feet: f32,
    pub height_feet: f32,
    pub instructions: Vec<VehicleInstructions>,
}

impl Vehicle {
    pub fn new(id: u64, vehicle_type: VehicleType, max_speed_fpt: f32, state: VehicleState, length_feet: f32, width_feet: f32, height_feet: f32, instructions: Vec<VehicleInstructions>)->Self{
        Self {
            id,
            vehicle_type,
            max_speed_fpt,
            state,
            length_feet,
            height_feet,
            width_feet,
            instructions
        }
    }
    pub fn tick(&mut self)-> Vec<general_objects::ChangeEnum>{
        let changes: Vec<general_objects::ChangeEnum> = Vec::new();
        
        self.state.movement_state.momentum_vector = 
        self.state.movement_state.momentum_vector.midpoint_vector(
            &self.state.movement_state.orientation_speed_vector, 
            0.5
        );
        
       
        self.state.coordinates.x += math::feet_to_miles(self.state.movement_state.momentum_vector.x);
        self.state.coordinates.y += math::feet_to_miles(self.state.movement_state.momentum_vector.y);
        self.state.coordinates.z += math::feet_to_miles(self.state.movement_state.momentum_vector.z);

        
        

        for instruction in self.instructions.iter() {
            match instruction.objective {
                VehicleObjective::GoToLocation { location} => {
                    let location_vector = self.state.coordinates.to_location_vector(&location).normalize();
                    // web_sys::console::log_1(&format!("To location orientaiton :{}, {}, {}", 
                    //     location_vector.x,
                    //     location_vector.y,
                    //     location_vector.z
                    // ).into());
                    let orientation_vector = self.state.movement_state.orientation_speed_vector.normalize();
                    // web_sys::console::log_1(&format!("Normalized orientaiton :{}, {}, {}", 
                    //     orientation_vector.x,
                    //     orientation_vector.y,
                    //     orientation_vector.z
                    // ).into());
                    let new_orientation_vector = orientation_vector
                        .new_vec_with_max_change(&location_vector, 0.01)
                        .scale(self.max_speed_fpt);

                    // web_sys::console::log_1(&format!("New orientation :{}, {}, {}", 
                    //     new_orientation_vector.x,
                    //     new_orientation_vector.y,
                    //     new_orientation_vector.z
                    // ).into());
                    self.state.movement_state.orientation_speed_vector = new_orientation_vector;
                }
                _ => {}
            }
        }
        // web_sys::console::log_1(&format!("Post Instruciton orientaiton :{}, {}, {}", 
        //     self.state.movement_state.orientation_speed_vector.x,
        //     self.state.movement_state.orientation_speed_vector.y,
        //     self.state.movement_state.orientation_speed_vector.z
        // ).into());
        // web_sys::console::log_1(&format!("Post Instruciton :{}, {}, {}", 
        //     self.state.movement_state.orientation_speed_vector.x,
        //     self.state.movement_state.orientation_speed_vector.y,
        //     self.state.movement_state.orientation_speed_vector.z
        // ).into());

        // let projectile_state = projectile::ProjectileState {
        //     coordinates: self.state.coordinates,
        //     speed_fps: 1000.0,
        //     dspeed_fps: -0.1,
        //     heading: self.state.heading,
        //     length_feet: 6.0,
        //     width_feet: 1.0,
        //     height_feet: 1.0,
        // };
        // changes.push(
        //     general_objects::ChangeEnum::NewObject{
        //         object: general_objects::MoveableObject::Projectile(
        //             projectile::Projectile::new(
        //                 String::from("proj"), 
        //                 projectile_state)
        //         )
        //     }
        // );
        changes
    }

}
