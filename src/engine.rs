use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;
use cgmath::{Vector3, Zero};
use vulkano::device::DeviceOwned;
use vulkano::image::ImageUsage;
use vulkano::image::view::ImageView;
use vulkano::swapchain::PresentMode;
use vulkano::sync::GpuFuture;
use vulkano_util::context::{VulkanoConfig, VulkanoContext};
use vulkano_util::renderer::{DEFAULT_IMAGE_FORMAT, VulkanoWindowRenderer};
use vulkano_util::window::{VulkanoWindows, WindowDescriptor};
use winit::dpi::PhysicalSize;
use winit::event::VirtualKeyCode;
use winit::event_loop::EventLoop;
use winit::window::{Fullscreen, Window};

pub mod world;

mod logic;
use logic::*;

use world::World;
use world::context::EngineContext;
use world::context::Feature;

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
    world: &'static mut World,
    pub settings: EngineSettings,
    context: &'static mut EngineContext,
    windows: &'static mut VulkanoWindows,

    engine_pipeline: EnginePipeline,
    place_over_frame: PlaceOverFrame,

    render_target_id: usize
}

impl EngineApplication {
    pub fn new(event_loop: &EventLoop<()>) -> &'static mut Self {
        let settings = EngineSettings::new("Engine");
        let world = World::new("Default World");

        print!("Creating window context... ");
        let mut init_time = Instant::now();
        let win_context = Box::leak(Box::new(VulkanoContext::new(VulkanoConfig::default())));
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

        let renderer = windows.get_primary_renderer_mut().unwrap();

        let main_window = renderer.window();
        main_window.set_title(settings.window_title);
        main_window.set_inner_size(settings.size);
        main_window.set_min_inner_size(settings.min_size);

        let render_target_id = 0;
        renderer.add_additional_image_view(
            render_target_id,
            DEFAULT_IMAGE_FORMAT,
            ImageUsage::SAMPLED | ImageUsage::STORAGE | ImageUsage::TRANSFER_DST,
        );

        print!("Done! ({}s)\n", Instant::now().duration_since(init_time).as_secs_f32());

        let context = EngineContext::new(win_context);

        let graphics = &context.graphics;
        let engine_pipeline = EnginePipeline::new(
            renderer.compute_queue(),
            graphics.memory_allocator.clone(),
            graphics.command_buffer_allocator.clone(),
            graphics.descriptor_set_allocator.clone()
        );

        let place_over_frame = PlaceOverFrame::new(
            renderer.graphics_queue(),
            renderer.swapchain_format(),
            graphics.command_buffer_allocator.clone(),
            graphics.descriptor_set_allocator.clone()
        );

        Box::leak(Box::new(Self {
            world,
            settings,

            engine_pipeline,
            place_over_frame,
            context,
            windows,
            render_target_id
        }))
    }

    pub fn match_input(&mut self) {
        let inputs = &mut self.context.keyboard;

        if inputs.is_key_just_released(VirtualKeyCode::F11) {
            let window = self.windows.get_primary_window().unwrap();

            let is_full_screen = window.fullscreen().is_some();
            window.set_fullscreen(if !is_full_screen {
                Some(Fullscreen::Borderless(window.current_monitor()))
            } else {
                None
            });
        }

        if inputs.is_key_just_released(VirtualKeyCode::M) {
            let window = self.windows.get_primary_window().unwrap();
            window.set_maximized(!window.is_maximized());
        }

        let graphics = &mut self.context.graphics;
        // Enable depth
        if inputs.is_key_pressed(VirtualKeyCode::Numpad1) {
            graphics.turn_feature(Feature::Depth)
        }

        if inputs.is_key_pressed(VirtualKeyCode::Space) {
            let camera = self.world.get_camera();

            camera.velocity += Vector3::new(0.0, 0.10, 0.0)
        }
        if inputs.is_key_pressed(VirtualKeyCode::LShift) {
            let camera = self.world.get_camera();

            camera.velocity -= Vector3::new(0.0, 0.10, 0.0)
        }

        if inputs.is_key_pressed(VirtualKeyCode::W) {
            let camera = self.world.get_camera();
            //camera.velocity += 0.1 * camera.rotation;
            //println!("{:?}", camera.velocity);
        }

        let exit_keys = HashSet::from([VirtualKeyCode::Escape, VirtualKeyCode::LShift]);

        if inputs.is_keys_pressed(exit_keys) {
            self.exit()
        }
    }

    pub fn update_world(&mut self, delta: f64) {
        self.world.update(&self.context, delta);
    }

    pub fn get_world(&mut self) -> &mut World {
        self.world
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

    pub fn compute(&self, image_target: Arc<ImageView>) -> Box<dyn GpuFuture> {
        self.engine_pipeline.compute(
            image_target
        )
    }

    pub fn compute_then_render(&mut self) {
        // Start the frame.
        //println!("Starting rendering frame!");

        let before_pipeline_future = match self.windows.get_primary_renderer_mut().unwrap().acquire() {
            Err(e) => {
                println!("{e}");
                return;
            }
            Ok(future) => future,
        };

        // Retrieve the target image.
        let image = self.windows.get_primary_renderer_mut().unwrap().get_additional_image_view(self.render_target_id);

        let after_compute = self.compute(image.clone()).join(before_pipeline_future);

        let clear_color = self.context.graphics.clear_color.clone();

        // Render the image over the swapchain image, inputting the previous future.
        let after_renderpass_future =
            self.place_over_frame
                .render(after_compute, clear_color, image, self.windows.get_primary_renderer_mut().unwrap().swapchain_image_view());

        self.windows.get_primary_renderer_mut().unwrap().present(after_renderpass_future, true);
    }

    pub fn exit(&self) {
        std::process::exit(0);
    }
}