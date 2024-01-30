use std::collections::HashSet;
use std::time::Instant;
use vulkano::swapchain::PresentMode;
use vulkano_util::context::{VulkanoConfig, VulkanoContext};
use vulkano_util::renderer::VulkanoWindowRenderer;
use vulkano_util::window::{VulkanoWindows, WindowDescriptor};
use winit::dpi::PhysicalSize;
use winit::event::VirtualKeyCode;
use winit::event_loop::EventLoop;
use winit::window::{Fullscreen, Window};
use crate::engine::context::EngineContext;

mod context;
mod rendering;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub struct EngineSettings {
    window_title: &'static str,

    size: PhysicalSize<f32>,
    min_size: Option<PhysicalSize<f32>>,

    pub fps_limit: u32
}

impl EngineSettings {
    fn new(_title: &str) -> Self {
        let window_title = _title.to_string().leak();
        let size = PhysicalSize::new(1280.0, 720.0);
        let min_size = Some(PhysicalSize::new(640.0, 360.0));
        let fps_limit = 120;

        Self {window_title, size, min_size, fps_limit}
    }
}

pub struct EngineApplication {
    pub settings: EngineSettings,
    context: &'static mut EngineContext,
    windows: &'static mut VulkanoWindows
}

impl EngineApplication {
    pub fn new(event_loop: &EventLoop<()>) -> &'static mut Self {
        let settings = EngineSettings::new("Engine");

        print!("Creating window context... ");
        let mut init_time = Instant::now();
        let win_context = VulkanoContext::new(VulkanoConfig::default());
        print!("Done! ({}s)\n", Instant::now().duration_since(init_time).as_secs_f32());

        print!("Creating main window... ");
        init_time = Instant::now();
        let windows = Box::leak(Box::new(VulkanoWindows::default()));

        windows.create_window(
            event_loop,
            &win_context,
            &WindowDescriptor {
                title: "Engine".to_string(),
                present_mode: PresentMode::Fifo,
                ..Default::default()
            },

            |_| {}
        );

        print!("Done! ({}s)\n", Instant::now().duration_since(init_time).as_secs_f32());

        let main_window = windows.get_primary_window().unwrap();
        main_window.set_title(settings.window_title);
        main_window.set_inner_size(settings.size);
        main_window.set_min_inner_size(settings.min_size);

        let context = EngineContext::new();

        Box::leak(Box::new(Self {
            settings,
            context,
            windows
        }))
    }

    pub fn request_redraw(&mut self) {
        //
    }

    pub fn match_input(&mut self) {
        let inputs = &mut self.context.keyboard;

        if inputs.is_key_pressed(VirtualKeyCode::F11) {
            let window = self.windows.get_primary_window().unwrap();

            let is_full_screen = window.fullscreen().is_some();
            window.set_fullscreen(if !is_full_screen {
                Some(Fullscreen::Borderless(window.current_monitor()))
            } else {
                None
            });
            inputs.release_key(Some(VirtualKeyCode::F11));
        }

        if inputs.is_key_pressed(VirtualKeyCode::M) {
            let window = self.windows.get_primary_window().unwrap();
            window.set_maximized(!window.is_maximized());
            inputs.release_key(Some(VirtualKeyCode::M));
        }

        let exit_keys = HashSet::from([VirtualKeyCode::Escape, VirtualKeyCode::LShift]);

        if inputs.is_keys_pressed(exit_keys) {
            self.exit()
        }
    }

    pub fn update_title(&mut self) {
        let renderer = self.windows.get_primary_renderer_mut().take().unwrap();
        let window = renderer.window();

        let size = window.inner_size();
        let display_size = format!("{}:{}", size.width, size.height);
        let aspect_ratio = renderer.aspect_ratio();

        let title = format!("{}; v{}; Size: {}; AR: {}", self.settings.window_title, VERSION, display_size, aspect_ratio).leak();
        window.set_title(title);
    }

    pub fn get_renderer(&mut self) -> &mut VulkanoWindowRenderer {
        self.windows.get_primary_renderer_mut().take().unwrap()
    }

    pub fn get_context(&self) -> &EngineContext {
        self.context
    }

    pub fn get_context_mut(&mut self) -> &mut EngineContext {
        self.context
    }

    pub fn get_window(&self) -> &Window {
        self.windows.get_primary_window().take().unwrap()
    }

    pub fn exit(&self) {
        std::process::exit(0);
    }
}