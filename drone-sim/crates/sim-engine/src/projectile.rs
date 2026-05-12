use crate::math;
use crate::general_objects;

pub struct ProjectileState{
    pub coordinates: math::Vec3,
    pub movement_state: math::MovementState,
}

pub enum ProjectileType{
    Missle
}
impl ProjectileType{
    pub fn to_string(&self)-> String{
        match self {
            ProjectileType::Missle => String::from("missle")
        }
    }
}

pub struct  Projectile {
    pub projectile_type: ProjectileType,
    pub id: u64,
    pub state: ProjectileState,
    pub length_feet: f32,
    pub width_feet: f32,
    pub height_feet: f32,
}

impl Projectile {
    pub fn new(projectile_type: ProjectileType, id: u64, state: ProjectileState, length_feet: f32, width_feet: f32, height_feet: f32)->Self{
        Self {
            projectile_type,
            id,
            state,
            length_feet,
            width_feet,
            height_feet
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
        changes
    }
}
