use std::sync::Arc;
use cgmath::{Vector3, Zero};
use vulkano::shader::spirv::Instruction::Name;

#[path="./context.rs"]
mod context;
use context::Context;

pub trait Object {
    fn new(_name: &str) -> &'static mut Self where Self: Sized;
    fn on_update(&mut self, _ctx: &Context) {
        // Empty
    }

    fn on_draw(&self, _ctx: &Context) {
        // Empty
    }
}

pub struct Cube {
    name: &'static str,

    position: Vector3<f64>,
    size: Vector3<f64>,
}

impl Object for Cube {
    fn new(_name: &str) -> &'static mut Self where Self: Sized {
        let name = Box::leak(Box::from(_name));

        let position: Vector3<f64> = Vector3::zero();
        let size = Vector3::new(1.0, 1.0, 1.0);

        Box::leak(Box::new(Self {name, position, size}))
    }

}

pub struct World {
    name: &'static str,
    objects: Vec<&'static mut dyn Object>
}

impl World {
    pub fn new(_name: &str) -> Arc<&'static mut Self>{
        let name = Box::leak(Box::from(_name));

        Arc::new(Box::leak(Box::new(Self {name, objects: Vec::new()})))
    }

    fn default() -> Arc<&'static mut Self> {
        let name = String::from("World").leak();

        let mut objects: Vec<&'static mut dyn Object> = Vec::new();

        let cube = Cube::new("Cube");

        objects.push(cube);

        Arc::new(Box::leak(Box::new(Self {name, objects})))
    }
}
