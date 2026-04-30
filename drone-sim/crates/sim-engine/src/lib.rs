
use wasm_bindgen::prelude::*;


pub struct Arena {
    height: f32,
    width: f32,
}

pub struct Vehicle {
    name: String,
    x: f32,
    y: f32,
    heading: f32,
    length_feet: f32,
    width_feet: f32,
    height_feet: f32,
}

pub struct  Projectile {
    name: String,
    x: f32,
    y: f32,
    speed_fps: f32,
    dspeed_fps: f32,
    heading: f32,
    length_feet: f32,
    width_feet: f32,
    height_feet: f32,
}



#[wasm_bindgen]
pub struct Simulation {
    arena: Arena,
    objects: Vec<MoveableObject>,
    tick_count: u32,
}

#[wasm_bindgen]
pub struct RenderObject {
    name: String,
    x: f32,
    y: f32,
    heading: f32,
    length_feet: f32,
    width_feet: f32,
    height_feet: f32,
}

#[wasm_bindgen]
impl RenderObject {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, x: f32, y:f32, heading: f32, length_feet: f32, width_feet: f32, height_feet: f32)->Self{
        Self {
            name,
            x,
            y,
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
    Vehicle(Vehicle),
    Projectile(Projectile),
}

impl Vehicle {
    pub fn new(name: String, x: f32, y: f32, heading: f32, length_feet: f32, width_feet: f32, height_feet: f32)->Self{
        Self {
            name,
            x,
            y,
            heading,
            length_feet,
            width_feet,
            height_feet,
        }
    }
}

impl Projectile {
    pub fn new(name: String, x: f32, y: f32, speed_fps: f32, dspeed_fps: f32, heading: f32, length_feet: f32, width_feet: f32, height_feet: f32)->Self{
        Self {
            name,
            x,
            y,
            speed_fps,
            dspeed_fps,
            heading,
            length_feet,
            width_feet,
            height_feet,
        }
    }
}

impl Arena {
    pub fn new(height: f32, width:f32)->Self{
        Self {
            height,
            width,
        }
    }
}

impl MoveableObject {
    fn as_render_object(&self)->RenderObject{
        match self {
            MoveableObject::Vehicle(v) => RenderObject::new(v.name.clone(), v.x, v.y, v.heading, v.length_feet, v.width_feet, v.height_feet ),
            MoveableObject::Projectile(p) => RenderObject::new(p.name.clone(), p.x, p.y, p.heading, p.length_feet, p.width_feet, p.height_feet),
        }
    }
}


#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new(length: f32, width:f32)->Self{
        Self {
            arena: Arena::new(length, width),
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
    pub fn add_vehicle(&mut self, name:String, x: f32, y: f32, heading: f32, length_feet: f32, width_feet: f32, height_feet: f32){
        self.objects.push(MoveableObject::Vehicle(Vehicle::new(name, x, y, heading, length_feet, width_feet, height_feet)));
    }
    fn return_objects_as_renderables(&self)->Vec<RenderObject>{
        self.objects.iter().map(MoveableObject::as_render_object).collect()
    }
    #[wasm_bindgen]
    pub fn tick(&mut self)->Vec<RenderObject>{
        let mut to_add: Vec<MoveableObject> = Vec::new();
        for object in self.objects.iter_mut(){
            match object{
                MoveableObject::Vehicle(v) => {
                    // v.heading+=0.1;
                    if(self.tick_count % 15 == 0){
                        to_add.push(MoveableObject::Projectile(Projectile::new(String::from("proj"), v.x, v.y, 1000.0, -0.1, v.heading, 6.0, 1.0, 1.0)));
                    }
                    
                },
                MoveableObject::Projectile(p)=> {
                    let (dx, dy) = translate_heading_speed_to_miles(p.heading, p.speed_fps);
                    p.x += dx;
                    p.y += dy;
                    p.speed_fps += p.dspeed_fps;
                },
            }
        }
        self.objects.retain(|object| {
            match object{
                MoveableObject::Vehicle(v) => {
                    true
                },
                MoveableObject::Projectile(p)=> {
                    !(p.x > self.arena.width || p.x < 0.0 || p.y > self.arena.height || p.y < 0.)
                },
            }
        });
        self.objects.extend(to_add);
        self.tick_count += 1;
        self.return_objects_as_renderables()
    }


}

fn translate_heading_speed_to_miles(heading: f32, speed_fps: f32)->(f32, f32){
    let degrees = (heading + 90.0)%360.0;
    let miles = feet_to_miles(speed_fps);
    let dy = degrees.to_radians().sin() * miles;
    let dx = degrees.to_radians().cos() * miles;
    (dx, dy)
}

fn feet_to_miles(feet: f32)-> f32 {
    feet/5280.0
}


