use cgmath::{Rotation3, Vector3};

pub trait Object {
    fn new(_name: &str) -> &'static mut Self where Self: Sized;

    // Main object functions
    fn draw(&self);
    fn update(&mut self, delta: f32);
}

pub struct Cuboid {
    name: &'static str,

    position: Vector3<f32>,
    velocity: Vector3<f32>,
    //rotation: Epsilon,
    scale: Vector3<f32>,
}

impl Object for Cuboid {
    fn new(_name: &str) -> &'static mut Self where Self: Sized {
        let name = Box::leak(Box::from(_name));

        let position = Vector3::new(0.0, 0.0, 0.0f32);
        let scale = Vector3::new(1.0, 1.0, 1.0f32);

        let velocity = Vector3::new(0.0, 0.0, 0.0f32);

        Box::leak(Box::new(Self {name, position, velocity, scale}))
    }

    fn update(&mut self, delta: f32) {
        self.position += self.velocity * delta;
    }

    fn draw(&self) {
        todo!()
    }
}

pub struct World {
    name: &'static str,

    objects: Vec<&'static mut dyn Object>
}

impl World {
    pub fn new(_name: &str) -> &'static mut Self {
        let name = Box::leak(Box::from(_name));

        Box::leak(Box::new(Self {name, objects: Vec::new()}))
    }

    pub fn save_world(&self) {
        //let raw_data = json!(self);
        //let data = serde_json::to_string(&raw_data);

    }

    pub fn update_world(&mut self, delta: f32) {
        for object in &mut self.objects {
            object.update(delta);
        }
    }

    pub fn update_camera(&mut self) {
        todo!()
    }

    pub fn draw_world(&self) {
        for object in &self.objects {
            object.draw();
        }
    }
}