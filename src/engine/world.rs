use std::borrow::BorrowMut;
use std::rc::Rc;
use std::sync::Arc;
use cgmath::{InnerSpace, Rotation, Rotation3, Vector3, Zero};

#[path="./context.rs"]
pub mod context;
use context::EngineContext;

pub(crate) type ObjectType = Box<dyn Object + Sync + Send>;

pub struct Transform {
    position: Vector3<f64>,
    direction: Vector3<f64>,
    scale: Vector3<f64>
}

impl Transform {
    pub fn new(position: Vector3<f64>) -> Self {
        let direction = Vector3::zero();
        let scale = Vector3::new(1.0, 1.0, 1.0);

        Self {position, direction, scale}
    }

    pub fn zero() -> Self {
        let position = Vector3::zero();
        let direction = Vector3::zero();
        let scale = Vector3::new(1.0, 1.0, 1.0);

        Self {position, direction, scale}
    }
}

pub trait Object {
    fn new(_name: &str, transform: Transform) -> ObjectType where Self: Sized;

    fn on_update(&mut self, _ctx: &EngineContext) { /* Empty */ }
    fn on_draw(&self, _ctx: &EngineContext);
}

pub struct Rectangle {
    name: &'static str,
    transform: Transform
}

impl Object for Rectangle {
    fn new(_name: &str, transform: Transform) -> ObjectType where Self: Sized {
        let name = _name.to_string().leak();

        Box::new(Self {name, transform})
    }

    fn on_draw(&self, _ctx: &EngineContext) {
        // pass
    }
}

pub struct Camera {
    pub transform: Transform,
    pub velocity: Vector3<f64>,
    pub fov: f64,

    max_speed: Vector3<f64>,
    speed: Vector3<f64>
}

impl Camera {
    pub fn new(transform: Transform, fov: f64) -> Self {
        let velocity = Vector3::zero();

        let max_speed = Vector3::new(10.0, 5.0, 10.0);
        let speed = Vector3::new(0.2, 0.1, 0.2);

        Self {transform, max_speed, speed, fov, velocity}
    }

    pub fn update(&mut self, delta: f64) {
        self.transform.position += self.velocity * delta;
        self.velocity -= 0.01 * self.max_speed;
        self.velocity.x = r_float(self.velocity.x, 3).clamp(-self.max_speed.x, self.max_speed.x);
        self.velocity.y = r_float(self.velocity.y, 3).clamp(-self.max_speed.y, self.max_speed.y);
        self.velocity.z = r_float(self.velocity.z, 3).clamp(-self.max_speed.z, self.max_speed.z);
    }
}

pub struct World {
    name: &'static str,
    camera: Camera,
    objects: &'static mut Vec<ObjectType>
}

fn r_float(x: f64, a: u32) -> f64 {
    (x * (10_i32.pow(a) as f64)).round() / (10_i32.pow(a) as f64)
}

impl World {
    pub fn new(_name: &str) -> &'static mut Self {
        let name = _name.to_string().leak();

        let mut objects: &'static mut Vec<ObjectType> = Box::leak(Box::new(Vec::new()));

        let cube = Rectangle::new("Cube", Transform::zero());
        objects.push(cube);

        let camera_transform = Transform::new([2.0, 2.0, 2.0].into());

        let camera = Camera::new(camera_transform, 70.0);

        Box::leak(Box::new(Self {name, camera, objects}))
    }

    pub fn update(&mut self, _ctx: &EngineContext, delta: f64) {
        self.camera.update(delta);

        for object in self.objects.into_iter() {
            object.on_update(_ctx);
        }
    }

    pub fn add_object(&mut self, object: ObjectType) {
        self.objects.push(object);
    }

    pub fn get_objects(&mut self) -> &mut Vec<ObjectType> {
        self.objects
    }

    pub fn get_camera(&mut self) -> &mut Camera {
        &mut self.camera
    }
}