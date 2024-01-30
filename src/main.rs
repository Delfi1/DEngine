// If not debug and not a window - hide console;
#![cfg_attr(
    all(
        target_os = "windows",
        not(debug_assertions)
    ),
    windows_subsystem = "windows"
)]

use std::time::{Duration, Instant};
use vulkano::image::ImageUsage;
use vulkano::swapchain::{PresentMode};
use vulkano_util::context::{VulkanoConfig, VulkanoContext};
use vulkano_util::renderer::{DEFAULT_IMAGE_FORMAT};
use vulkano_util::window::{VulkanoWindows, WindowDescriptor};
use winit::event::{ElementState, Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use crate::engine::EngineApplication;

mod engine;

fn main() {
    println!("DEngine - Simple Physics Engine; \nCurrent version: v{}; \nStarting Initialization...", engine::VERSION);
    let engine_init_time = Instant::now();

    print!("Creating event loop... ");
    let mut init_time = Instant::now();
    let event_loop = EventLoop::new();
    print!("Done! ({}s)\n", Instant::now().duration_since(init_time).as_secs_f32());

    print!("Creating window context... ");
    init_time = Instant::now();
    let context = VulkanoContext::new(VulkanoConfig::default());
    print!("Done! ({}s)\n", Instant::now().duration_since(init_time).as_secs_f32());

    print!("Creating main window... ");
    init_time = Instant::now();
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
    init_time = Instant::now();
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

    println!("Initialization completed in {}s", Instant::now().duration_since(engine_init_time).as_secs_f32());

    println!("Starting main loop...");
    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();

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
                    WindowEvent::KeyboardInput {input, ..} => {
                        if input.state == ElementState::Pressed {
                            println!("Key \"{:?}\" has been pressed", input.virtual_keycode.unwrap());
                            app.context().keyboard.pressed_keys().insert(input);
                        } else {
                            app.context().keyboard.release_key(input.virtual_keycode)
                        }
                    }
                    WindowEvent::Resized(..) | WindowEvent::ScaleFactorChanged { .. } => renderer.resize(),
                    _ => ()
                }
            },
            Event::MainEventsCleared => {
                app.update_title();
            },

            _ => ()
        }

        match *control_flow {
            ControlFlow::Poll => {
                let delta = app.context().time.delta();
                //println!("{:?}", delta.as_secs_f32());
                //print!("{:?}", app.context().keyboard.pressed_keys());

                let wait_time = app.context().time.frame_time() + Duration::from_secs_f64(1.0 / app.settings.fps_limit as f64) - delta;

                *control_flow = ControlFlow::WaitUntil(wait_time);
                app.request_redraw();

                // match input
                app.match_input();
                //Update context
                app.context().update()
            },
            _ => ()
        }
    })
}
