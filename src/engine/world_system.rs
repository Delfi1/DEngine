use std::f64::consts::PI;
use std::sync::Arc;
use cgmath::{Rotation3, Vector3, Zero};

trait Object {
    fn new(_name: &str) -> &'static mut Self where Self: Sized;

    /// If object was deleted;
    fn on_object_queued(&self) {}

    /// Object Update request;
    fn on_update(&mut self);

    /// Object Draw request;
    fn on_draw(&self);
}

// Objects;
/// Cube object;
pub struct Cube {
    name: &'static str,

    position: Vector3<f64>,
    velocity: Vector3<f64>,

    //rotation: Rotation3,
    size: f64
}

impl Object for Cube {
    fn new(_name: &str) -> &'static mut Self where Self: Sized {
        let name = Box::leak(Box::from(_name));

        let position: Vector3<f64> = Vector3::zero();
        let velocity: Vector3<f64> = Vector3::zero();

        let size = 1.0;

        Box::leak(Box::new(Self {name, position, velocity, size}))
    }

    fn on_update(&mut self) {
        todo!()
    }

    fn on_draw(&self) {
        todo!()
    }
}

pub struct Sphere {
    name: &'static str,

    position: Vector3<f64>,
    velocity: Vector3<f64>,

    radius: f64
}

impl Object for Sphere {
    fn new(_name: &str) -> &'static mut Self where Self: Sized {
        let name = Box::leak(Box::from(_name));

        let position: Vector3<f64> = Vector3::zero();
        let velocity: Vector3<f64> = Vector3::zero();

        let radius = 0.5;

        Box::leak(Box::new(Self {name, position, velocity, radius}))
    }

    fn on_update(&mut self) {
        todo!()
    }

    fn on_draw(&self) {
        todo!()
    }
}

struct Camera {
    position: Vector3<f64>,
    rotation: Vector3<f64>,

    fov: f64
}

impl Camera {
    pub fn new(position: Vector3<f64>, rotation: Vector3<f64>, fov_degrees: f64) -> Self {
        let fov = fov_degrees.to_radians();

        Self {position, rotation, fov}
    }
}

impl Default for Camera {
    fn default() -> Self {
        let position = Vector3::new(1.0, 1.0, 1.0);
        let fi = 45.0_f64.to_radians();
        let rotation = Vector3::new(fi, fi, fi);

        let fov = 70.0_f64.to_radians();

        Self {position, rotation, fov}
    }
}

/// World;
pub struct World {
    name: &'static str,
    camera: Camera,

    objects: Vec<&'static mut dyn Object>
}

impl World {
    pub fn new(_name: &str) -> Arc<&'static mut Self> {
        let name = Box::leak(Box::from(_name));

        let camera = Camera::default();

        let cube = Cube::new("Cube");

        let mut objects: Vec<&'static mut dyn Object> = Vec::new();
        objects.push(cube);

        Arc::new(Box::leak(Box::new(Self {name, camera, objects})))
    }

    pub fn get_objects(&mut self) -> &mut Vec<&'static mut dyn Object> {
        &mut self.objects
    }

    pub fn update_world(&mut self) {
        //self.camera

        for object in &mut self.objects {
            object.on_update();
        }
    }

    pub fn add_object(&mut self, object: &'static mut dyn Object) {
        self.objects.push(object);
    }
}