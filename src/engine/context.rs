use std::collections::HashSet;
use std::time::{Duration, Instant};
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode};

pub struct TimeContext {
    init_time: Instant,
    current_time: Instant,
    ticks: usize
}

impl TimeContext {
    pub fn new() -> Self {
        Self {init_time: Instant::now(), current_time: Instant::now(), ticks: 0}
    }

    pub fn frame_time(&self) -> Instant {
        self.current_time
    }

    pub fn delta(&self) -> Duration {
        Instant::now().duration_since(self.current_time)
    }

    pub fn time_since_start(&self) -> Duration {
        Instant::now().duration_since(self.init_time)
    }

    pub fn average_delta(&self) -> Duration {
        todo!()
    }

    pub fn fps(&self) -> f64 {
        todo!()
    }

    pub fn ticks(&self) -> usize {
        self.ticks
    }
}

pub struct KeyboardContext {
    pressed_keys: HashSet<KeyboardInput>
}

impl KeyboardContext {
    pub fn new() -> Self {
        Self {pressed_keys: HashSet::new()}
    }

    pub fn is_key_pressed(&self, key: VirtualKeyCode) -> bool {
        self.pressed_keys.iter()
            .find(|x| x.virtual_keycode == Some(key) &&
                x.state == ElementState::Pressed)
            .is_some()
    }

    pub fn pressed_keys(&mut self) -> &mut HashSet<KeyboardInput> {
        &mut self.pressed_keys
    }

    pub fn is_keys_pressed(&self, keys: HashSet<VirtualKeyCode>) -> bool {
        for key in keys {
            if self.pressed_keys.iter()
                .find(|x| x.virtual_keycode == Some(key) &&
                    x.state == ElementState::Pressed)
                .is_none() {
                return false;
            }
        }

        return true;
    }

    pub fn release_key(&mut self, key: Option<VirtualKeyCode>) {
        let value = *self.pressed_keys.iter()
            .find(|x| x.virtual_keycode == key)
            .take()
            .unwrap();

        self.pressed_keys.remove(&value);
    }
}

pub struct Context {
    pub time: TimeContext,
    pub keyboard: KeyboardContext
}

impl Context {
    pub fn new() -> &'static mut Self {
        let time = TimeContext::new();
        let keyboard = KeyboardContext::new();

        Box::leak(Box::new(Self {time, keyboard}))
    }

    pub fn update(&mut self) {
        // Update time context
        self.time.current_time = Instant::now();
        self.time.ticks += 1;

        // Update keyboard context

    }
}