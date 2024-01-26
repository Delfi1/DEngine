#![cfg_attr(
    all(
    target_os = "windows",
    not(debug_assertions)
    ),
    windows_subsystem = "windows"
)]

use std::time::Instant;
use vulkano::image::ImageUsage;
use vulkano::swapchain::{PresentMode};
use vulkano_util::context::{VulkanoConfig, VulkanoContext};
use vulkano_util::renderer::{DEFAULT_IMAGE_FORMAT};
use vulkano_util::window::{VulkanoWindows, WindowDescriptor};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use crate::engine::EngineApplication;

mod engine;

fn main() {
    println!("DEngine - Simple Physics Engine; \nCurrent version: v{}; \nStarting Initialization...", engine::VERSION);

    print!("Creating event loop... ");
    let init_time = Instant::now();
    let event_loop = EventLoop::new();
    print!("Done! ({}s)\n", Instant::now().duration_since(init_time).as_secs_f32());

    print!("Creating window context... ");
    let init_time = Instant::now();
    let context = VulkanoContext::new(VulkanoConfig::default());
    print!("Done! ({}s)\n", Instant::now().duration_since(init_time).as_secs_f32());

    print!("Creating main window... ");
    let windows = Box::leak(Box::new(VulkanoWindows::default()));

    windows.create_window(
        &event_loop,
        &context,
        &WindowDescriptor {
            title: "Engine".to_string(),
            present_mode: PresentMode::Fifo,
            ..Default::default()
        },

        |_| {}
    );

    print!("Done! ({}s)\n", Instant::now().duration_since(init_time).as_secs_f32());

    print!("Creating app... ");
    let render_target_id = 0;
    let primary_window_renderer = windows.get_primary_renderer_mut().unwrap();

    primary_window_renderer.add_additional_image_view(
        render_target_id,
        DEFAULT_IMAGE_FORMAT,
        ImageUsage::SAMPLED | ImageUsage::STORAGE | ImageUsage::TRANSFER_DST,
    );

    let gfx_queue = context.graphics_queue();

    let app = EngineApplication::new(gfx_queue, windows);
    print!("Done! ({}s)\n", Instant::now().duration_since(init_time).as_secs_f32());

    println!("Starting main loop...");
    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();
        app.update_time();

        let renderer = app.get_renderer();

        match event {
            Event::WindowEvent {
                event,
                ..
            } => {
                match event {
                    WindowEvent::CloseRequested => {
                        control_flow.set_exit();
                    },
                    WindowEvent::Resized(..) | WindowEvent::ScaleFactorChanged { .. } => renderer.resize(),
                    _ => ()
                }
            },
            Event::MainEventsCleared => {
                renderer.window().request_redraw();
                app.update_title();
            },
            Event::RedrawRequested(_) => 'redraw: {
                // Draw Frame
            }
            _ => ()
        }

        match *control_flow {
            ControlFlow::Poll => {
                let delta = app.get_delta_time();
                *control_flow = ControlFlow::WaitUntil(delta);
            },
            _ => ()
        }
    })
}
