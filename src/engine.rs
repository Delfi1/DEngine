use std::collections::HashSet;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use vulkano::command_buffer::allocator::{StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo};

use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;
use vulkano::device::{Queue};
use vulkano::memory::allocator::StandardMemoryAllocator;
use vulkano_util::renderer::VulkanoWindowRenderer;
use vulkano_util::window::VulkanoWindows;
use winit::dpi::PhysicalSize;
use winit::event::VirtualKeyCode;
use winit::event_loop::ControlFlow;
use winit::window::{Fullscreen, Window};

mod rendering;

mod context;
use context::Context;

mod world_system;
use world_system::World;

// Engine Settings
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Settings {
    title: &'static str,

    size: PhysicalSize<u32>,
    min_size: PhysicalSize<u32>,

    pub fps_limit: u64
}

impl Settings {
    pub fn new(title: &'static str, size: PhysicalSize<u32>, min_size: PhysicalSize<u32>, fps_limit: u64) -> Self {
        Self {title, size, min_size, fps_limit}
    }
}

impl Default for Settings {
    fn default() -> Self {
        let size = PhysicalSize::new(1280, 720);
        let min_size = PhysicalSize::new(640, 360);

        Self {title: "Engine", size, min_size, fps_limit: 120}
    }
}


pub struct EngineApplication {
    pub world: Arc<&'static mut World>,
    pub settings: Settings,
    context: &'static mut Context,

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

        let context = Context::new();

        Box::leak(Box::new(Self {
            world,
            settings,
            context,

            windows
        }))
    }

    pub fn match_input(&self) {
        let inputs = &self.context.keyboard;

        if inputs.is_key_pressed(VirtualKeyCode::F11) {
            let window = self.windows.get_primary_window().unwrap();

            let is_full_screen = window.fullscreen().is_some();
            window.set_fullscreen(if !is_full_screen {
                Some(Fullscreen::Borderless(window.current_monitor()))
            } else {
                None
            });
        }

        let exit_keys = HashSet::from([VirtualKeyCode::Escape, VirtualKeyCode::LShift]);

        if inputs.is_keys_pressed(exit_keys) {
            self.exit()
        }

        //if inputs.is_key_pressed(VirtualKeyCode::Escape) {
        //    self.exit()
        //}
    }

    pub fn context(&mut self) -> &mut Context {
        self.context
    }

    pub fn exit(&self) {
        std::process::exit(0);
    }

    pub fn update_title(&mut self) {
        let renderer = self.windows.get_primary_renderer_mut().take().unwrap();
        let window = renderer.window();

        let size = window.inner_size();
        let display_size = format!("{}:{}", size.width, size.height);
        let aspect_ratio = renderer.aspect_ratio();

        let title = format!("{}; v{}; Size: {}; AR: {}", self.settings.title, VERSION, display_size, aspect_ratio).leak();
        window.set_title(title);
    }

    pub fn request_redraw(&mut self) {

    }

    pub fn get_renderer(&mut self) -> &mut VulkanoWindowRenderer {
        self.windows.get_primary_renderer_mut().unwrap()
    }

    pub fn get_window(&self) -> &Window {
        self.windows.get_primary_window().unwrap()
    }
}