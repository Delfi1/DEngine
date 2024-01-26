use std::sync::Arc;
use std::time::Instant;

use vulkano::command_buffer::allocator::{StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo};

use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;
use vulkano::device::{Queue};
use vulkano::memory::allocator::StandardMemoryAllocator;
use vulkano_util::renderer::VulkanoWindowRenderer;
use vulkano_util::window::VulkanoWindows;
use winit::dpi::PhysicalSize;
mod rendering;

mod world_system;
use world_system::World;

// Engine Settings
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Settings {
    title: &'static str,

    size: PhysicalSize<u32>,
    min_size: PhysicalSize<u32>,

    fps_limit: u64,
    compute_time: Instant
}

impl Settings {
    pub fn new(title: &'static str, size: PhysicalSize<u32>, min_size: PhysicalSize<u32>, fps_limit: u64) -> Self {
        let compute_time = Instant::now();

        Self {title, size, min_size, fps_limit, compute_time}
    }
}

impl Default for Settings {
    fn default() -> Self {
        let compute_time = Instant::now();

        let size = PhysicalSize::new(1280, 720);
        let min_size = PhysicalSize::new(640, 360);

        Self {title: "Engine", size, min_size, fps_limit: 120, compute_time}
    }
}

pub struct EngineApplication {
    world: Arc<&'static mut World>,
    pub settings: Settings,

    windows: &'static mut VulkanoWindows
}

impl EngineApplication {
    pub fn new(gfx_queue: &Arc<Queue>, windows: &'static mut VulkanoWindows) -> &'static mut Self {
        let settings = Settings::default();
        let main_window = windows.get_primary_renderer_mut().unwrap().window();
        main_window.set_title(settings.title);
        main_window.set_inner_size(settings.size);
        main_window.set_min_inner_size(Some(settings.min_size));

        let world = World::new("Default World");

        let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(
            gfx_queue.device().clone(),
        ));
        let command_buffer_allocator = Arc::new(StandardCommandBufferAllocator::new(
            gfx_queue.device().clone(),
            StandardCommandBufferAllocatorCreateInfo {
                secondary_buffer_count: 32,
                ..Default::default()
            },
        ));
        let descriptor_set_allocator = Arc::new(StandardDescriptorSetAllocator::new(
            gfx_queue.device().clone(),
            Default::default(),
        ));

        Box::leak(Box::new(Self {
            world,
            settings,
            windows
        }))
    }

    pub fn update_time(&mut self) {
        self.settings.compute_time = Instant::now()
    }

    pub fn get_time(&self) -> Instant {
        self.settings.compute_time
    }

    pub fn get_delta_time(&self) -> Instant {
        let elapsed_time = Instant::now().duration_since(self.settings.compute_time).as_nanos() as u64;

        let delta = {
            match 1_000_000_000 / self.settings.fps_limit >= elapsed_time {
                true => 1_000_000_000 / self.settings.fps_limit - elapsed_time,
                false => 0
            }
        };

        self.settings.compute_time + std::time::Duration::from_nanos(delta)
    }

    pub fn update_title(&mut self) {
        let window = self.windows.get_primary_renderer_mut().take().unwrap().window();

        let size = window.inner_size();
        let display_size = format!("{}:{}", size.width, size.height);
        let title = format!("{}; v{}; Size: {}; ", self.settings.title, VERSION, display_size).leak();
        window.set_title(title);
    }

    pub fn get_renderer(&mut self) -> &mut VulkanoWindowRenderer {
        self.windows.get_primary_renderer_mut().unwrap()
    }
}