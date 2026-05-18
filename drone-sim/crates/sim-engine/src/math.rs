#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn length(&self)->f32{
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }
    pub fn midpoint_vector(&self, vec2: &Vec3, original_weighting: f32)->Vec3{
        let other_weighting = 1.0 - original_weighting;
        Vec3{
            x: (original_weighting*self.x + other_weighting*vec2.x),
            y: (original_weighting*self.y + other_weighting*vec2.y),
            z: (original_weighting*self.z + other_weighting*vec2.z)
        }
    }
    pub fn to_location_vector(&self, desired_location: &Vec3)-> Vec3{
        Vec3 { 
            x: desired_location.x-self.x, 
            y: desired_location.y-self.y, 
            z: desired_location.z-self.z 
        }
    }
    pub fn normalize(&self)->Vec3{
        if self.length()==0.0{
            return Vec3{
                x:0.0,y:0.0,z:0.0
            };
        }
        Vec3{
            x: self.x/self.length(),
            y: self.y/self.length(),
            z: self.z/self.length()
        }
    }
   pub fn scale(&self, scaler: f32)->Vec3{
        Vec3{
            x: self.x*scaler,
            y: self.y*scaler,
            z: self.z*scaler
        }
    }
    pub fn normalize_and_scale(&self, scaler: f32)->Vec3{
        self.normalize().scale(scaler)
    }
    pub fn new_vec_with_max_change(&self, vec2: &Vec3, max_change: f32)->Vec3{
        
        let mut x = vec2.x - self.x;
        let mut y = vec2.y - self.y;
        let mut z = vec2.z - self.z;
        
        if x.abs() > max_change{
            x = max_change * (x.abs()/x);
        }
        if y.abs() > max_change{
            y = max_change * (y.abs()/y);
        }
        if z.abs() > max_change{
            z = max_change * (z.abs()/z);
        }
        
        let returned_vec = Vec3 { x: self.x+x, y: self.y+y, z: self.z+z };
        
        returned_vec
    }
}



pub struct MovementState {
    pub momentum_vector: Vec3,
    pub orientation_speed_vector: Vec3,
}


pub fn feet_to_miles(feet: f32)-> f32 {
    feet/5280.0
}

pub fn heading(x: f32, y: f32)->f32{
    let mut heading = y.atan2(x).to_degrees();
    if heading < 0.0{
        heading = heading + 360.0;
    }
    heading
}

pub fn heading_to_x_y(heading: f32)->(f32, f32){
    (heading.to_radians().cos(), heading.to_radians().sin())
}