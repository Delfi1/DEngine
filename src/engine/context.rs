use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use vulkano::command_buffer::allocator::{StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo};
use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;
use vulkano::device::Queue;
use vulkano::memory::allocator::StandardMemoryAllocator;
use vulkano_util::context::VulkanoContext;
use winit::event::{KeyboardInput, VirtualKeyCode};

pub struct GraphicsContext {
    pub queue: Arc<Queue>,
    pub memory_allocator: Arc<StandardMemoryAllocator>,
    pub command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
    pub descriptor_set_allocator: Arc<StandardDescriptorSetAllocator>,
    pub clear_color: [f32; 4],

    // Private
    window_context: &'static mut VulkanoContext,
    features: HashSet<Feature>
}

#[derive(Eq, Hash, PartialEq)]
pub enum Feature {
    Depth,
}

impl GraphicsContext {
    pub fn new(window_context: &'static mut VulkanoContext) -> Self {
        let queue = window_context.graphics_queue().clone();

        let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(
            queue.device().clone(),
        ));
        let command_buffer_allocator = Arc::new(StandardCommandBufferAllocator::new(
            queue.device().clone(),
            StandardCommandBufferAllocatorCreateInfo {
                secondary_buffer_count: 32,
                ..Default::default()
            },
        ));
        let descriptor_set_allocator = Arc::new(StandardDescriptorSetAllocator::new(
            queue.device().clone(),
            Default::default(),
        ));

        let features = HashSet::new();

        let clear_color = [0.0, 0.0, 0.0, 1.0];

        Self {queue, window_context, memory_allocator, clear_color, command_buffer_allocator, descriptor_set_allocator, features}
    }

    pub fn turn_feature(&mut self, feature: Feature) {
        if self.features.iter().find(|x| x == &&feature).is_none() {
            self.features.insert(feature);
        } else {
            self.features.remove(&feature);
        }
    }

    pub fn is_feature_enabled(&self, feature: Feature) -> bool {
        self.features.iter().find(|x| x == &&feature).is_some()
    }
}

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
    pressed_keys: HashSet<VirtualKeyCode>,
    just_released_keys: HashSet<VirtualKeyCode>
}

impl KeyboardContext {
    pub fn new() -> Self {
        Self {pressed_keys: HashSet::new(), just_released_keys: HashSet::new()}
    }

    pub fn is_key_pressed(&self, key: VirtualKeyCode) -> bool {
        self.pressed_keys.iter()
            .find(|x| x == &&key)
            .is_some()
    }

    pub fn is_key_just_released(&self, key: VirtualKeyCode) -> bool {
        self.just_released_keys.iter()
            .find(|x| x == &&key)
            .is_some()
    }

    pub fn pressed_keys(&mut self) -> &mut HashSet<VirtualKeyCode> {
        &mut self.pressed_keys
    }

    pub fn just_released_keys(&mut self) -> &mut HashSet<VirtualKeyCode> {
        &mut self.just_released_keys
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

pub struct EngineContext {
    pub time: TimeContext,
    pub keyboard: KeyboardContext,
    pub graphics: GraphicsContext
}

impl EngineContext {
    pub fn new(window_context: &'static mut VulkanoContext) -> &'static mut Self {
        let time = TimeContext::new();
        let keyboard = KeyboardContext::new();

        let graphics = GraphicsContext::new(window_context);

        Box::leak(Box::new(Self {time, keyboard, graphics}))
    }

    pub fn update(&mut self) {
        // Update time context
        self.time.frame_time = Instant::now();
        self.time.ticks += 1;

        // Update keyboard context
        self.keyboard.just_released_keys.clear();
    }
}