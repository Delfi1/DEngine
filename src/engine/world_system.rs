use cgmath::{Rotation3, Vector3, Zero};

trait Object {
    fn new(_name: &str) -> &'static mut Self where Self: Sized;

    /// If object was spawned;
    fn on_start(&mut self) {}

    /// If object was deleted;
    fn on_object_queued(&self) {}

    /// Object Update request;
    fn on_update(&mut self);

    /// Object Draw request;
    fn on_draw(&self);
}

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

/// World;
pub struct World {
    name: &'static str,
    objects: Vec<&'static mut dyn Object>
}

impl World {
    pub fn new(_name: &str) -> &'static mut Self {
        let name = Box::leak(Box::from(_name));

        let cube = Cube::new("Cube");

        let mut objects: Vec<&'static mut dyn Object> = Vec::new();
        objects.push(cube);

        Box::leak(Box::new(Self {name, objects}))
    }

    pub fn add_object(&mut self, object: &'static mut dyn Object) {
        self.objects.push(object);
    }

    pub fn start_world(&mut self) {
        // Callback
        for obj in &mut self.objects {
            obj.on_start();
        }
    }

    pub fn update_world(&mut self) {
        for obj in &mut self.objects {
            obj.on_update();
        }
    }

    pub fn draw_world(&self) {
        for obj in &self.objects {
            obj.on_draw(); // TODO: ADD draw data
        }
    }
}