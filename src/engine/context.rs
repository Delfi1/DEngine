use std::collections::HashSet;
use std::sync::Arc;
use std::time::{Duration, Instant};
use winit::event::{KeyboardInput, VirtualKeyCode};

pub struct TimeContext {
    init_time: Instant,
    frame_time: Instant,
    ticks: usize
}

impl TimeContext {
    pub fn new() -> Self {
        let init_time = Instant::now();
        let frame_time = Instant::now();

        Self {init_time, frame_time, ticks: 0}
    }

    pub fn get_time(&self) -> Instant {
        self.frame_time
    }

    pub fn delta(&self) -> Duration {
        Instant::now().duration_since(self.frame_time)
    }

    pub fn time_since_start(&self) -> Duration {
        Instant::now().duration_since(self.init_time)
    }

    pub fn ticks(&self) -> usize {
        self.ticks
    }
}

pub struct KeyboardContext {
    pressed_keys: HashSet<VirtualKeyCode>
}

impl KeyboardContext {
    pub fn new() -> Self {
        Self {pressed_keys: HashSet::new()}
    }

    pub fn is_key_pressed(&self, key: VirtualKeyCode) -> bool {
        self.pressed_keys.iter()
            .find(|x| x == &&key)
            .is_some()
    }

    pub fn pressed_keys(&mut self) -> &mut HashSet<VirtualKeyCode> {
        &mut self.pressed_keys
    }

    pub fn is_keys_pressed(&self, keys: HashSet<VirtualKeyCode>) -> bool {
        for key in keys {
            if self.pressed_keys.iter()
                .find(|x| x == &&key)
                .is_none() {
                return false;
            }
        }

        return true;
    }

    pub fn release_key(&mut self, key: Option<VirtualKeyCode>) {
        self.pressed_keys.remove(&key.unwrap());
    }
}

pub trait Object {
    fn new() -> &'static mut Self where Self: Sized + Send;

    fn on_update(&mut self, ctx: &EngineContext);
    fn on_draw(&self, ctx: &EngineContext);
}

pub struct WorldContext {
    name: &'static str,
    objects: Vec<&'static mut dyn Object>
}

impl WorldContext {
    pub fn new(_name: &str) -> Arc<&'static mut Self> {
        let name = _name.to_string().leak();

        let mut objects: Vec<&'static mut dyn Object> = Vec::new();
        //let cube = Cube::new()
        //objects.push(cube)
        Arc::new(Box::leak(Box::new(Self {name, objects})))
    }

    pub fn get_objects(&mut self) -> &mut Vec<&'static mut dyn Object> {
        &mut self.objects
    }
}

pub struct EngineContext {
    pub time: TimeContext,
    pub keyboard: KeyboardContext,
    world: Arc<&'static mut WorldContext>
}

impl EngineContext {
    pub fn new() -> &'static mut Self {
        let time = TimeContext::new();
        let keyboard = KeyboardContext::new();
        let world = WorldContext::new("Default World");

        Box::leak(Box::new(Self {time, keyboard, world}))
    }

    pub fn update(&mut self) {
        // Update time context
        self.time.frame_time = Instant::now();
        self.time.ticks += 1;

    }
}