use std::borrow::BorrowMut;
use std::rc::Rc;
use std::sync::Arc;
use cgmath::{Vector3, Zero};

#[path="./context.rs"]
pub mod context;
use context::EngineContext;

pub(crate) type ObjectType = Box<dyn Object + Sync + Send>;

pub trait Object {
    fn new(_name: &str) -> ObjectType where Self: Sized;

    fn on_update(&mut self, _ctx: &EngineContext) { /* Empty */ }
    fn on_draw(&self, _ctx: &EngineContext);
}

pub struct Rectangle {
    name: &'static str,

    position: Vector3<f64>,
    scale: Vector3<f64>
}

impl Object for Rectangle {
    fn new(_name: &str) -> ObjectType where Self: Sized {
        let name = _name.to_string().leak();
        let position = Vector3::zero();
        let scale = Vector3::new(1.0, 1.0, 1.0);

        Box::new(Self {name, position, scale})
    }

    fn on_draw(&self, _ctx: &EngineContext) {
        // pass
    }
}

pub struct World {
    name: &'static str,
    objects: &'static mut Vec<ObjectType>
}

impl World {
    pub fn new(_name: &str) -> &'static mut Self {
        let name = _name.to_string().leak();

        let mut objects: &'static mut Vec<ObjectType> = Box::leak(Box::new(Vec::new()));

        let cube = Rectangle::new("Cube");
        objects.push(cube);

        Box::leak(Box::new(Self {name, objects}))
    }

    pub fn add_object(&mut self, object: ObjectType) {
        self.objects.push(object);
    }

    pub fn get_objects(&mut self) -> &mut Vec<ObjectType> {
        self.objects
    }
}