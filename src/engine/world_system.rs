
use std::sync::Arc;
use cgmath::{Vector3, Zero};

pub trait Object {
    fn new(_name: &str) -> &'static mut Self where Self: Sized;

    /// If object was deleted;
    fn on_object_queued(&self) { /* Empty */ }

    /// Object Update request;
    fn on_update(&mut self, _delta: f64) { /* Empty */ }

    /// Object Draw request;
    fn on_draw(&self, camera: &Camera);
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

    fn on_draw(&self, _camera: &Camera) {
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

    fn on_update(&mut self, _delta: f64) {
        todo!()
    }

    fn on_draw(&self, _camera: &Camera) {
        todo!()
    }
}

pub struct Camera {
    position: Vector3<f64>,
    rotation: Vector3<f64>,

    velocity: Vector3<f64>,

    fov: f64
}

impl Camera {
    pub fn new(position: Vector3<f64>, rotation: Vector3<f64>, fov_degrees: f64) -> Self {
        let fov = fov_degrees.to_radians();

        let velocity = Vector3::new(0.0, 0.0, 0.0);

        Self {position, rotation, velocity, fov}
    }

    pub fn on_update(&mut self, _delta: f64) {
        self.position += self.velocity
    }
}

impl Default for Camera {
    fn default() -> Self {
        let position = Vector3::new(1.0, 1.0, 1.0);
        let fi = 45.0_f64.to_radians();
        let rotation = Vector3::new(fi, fi, fi);

        let fov = 70.0_f64.to_radians();

        let velocity = Vector3::new(0.0, 0.0,0.0);

        Self {position, rotation, velocity, fov}
    }
}

/// World struct;
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

    pub fn update_world(&mut self, delta: f64) {
        self.camera.on_update(delta);

        for object in &mut self.objects {
            object.on_update(delta);
        }
    }

    pub fn draw_world(&self) {
        for object in &self.objects {
            object.on_draw(&self.camera);
        }
    }

    pub fn add_object(&mut self, object: &'static mut dyn Object) {
        self.objects.push(object);
    }
}