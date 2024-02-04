// If not debug and not a window - hide console;
#![cfg_attr(
    all(
        target_os = "windows",
        not(debug_assertions)
    ),
    windows_subsystem = "windows"
)]


use std::time::{Duration, Instant};
use vulkano::sync::GpuFuture;
use winit::event::{ElementState, Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

mod engine;
use engine::EngineApplication;

fn main() {
    println!("Delfi Engine - Simple Physics Engine; \nCurrent version: v{}; \nStarting Initialization...", engine::VERSION);
    let engine_init_time = Instant::now();

    print!("Creating event loop... ");
    let init_time = Instant::now();
    let event_loop = EventLoop::new();
    print!("Done! ({}s)\n", Instant::now().duration_since(init_time).as_secs_f32());

    println!("Creating Main Application...");
    let app = EngineApplication::new(&event_loop);

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
                        let keyboard = &mut app.get_context_mut().keyboard;

                        if input.virtual_keycode.is_none() {
                            return;
                        }

                        if input.state == ElementState::Pressed {
                            keyboard.pressed_keys().insert(input.virtual_keycode.unwrap());
                            println!("Key \"{:?}\" has been pressed", input.virtual_keycode.unwrap());
                        } else {
                            keyboard.pressed_keys().remove(&input.virtual_keycode.unwrap());
                            keyboard.just_released_keys().insert(input.virtual_keycode.unwrap());
                            println!("Key \"{:?}\" has been released", input.virtual_keycode.unwrap());
                        }
                    },

                    WindowEvent::Resized(..) | WindowEvent::ScaleFactorChanged { .. } => renderer.resize(),
                    _ => ()
                }
            },
            Event::RedrawRequested(_) => {
                let draw_start = Instant::now();
                app.compute_then_render();
                //println!("Draw time: {}", Instant::now().duration_since(draw_start).as_secs_f32());
            },
            Event::MainEventsCleared => {
                app.update_title();
            },

            _ => ()
        }

        match *control_flow {
            ControlFlow::Poll => {

                // Update World
                let phys_delta = 1.0 / app.settings.fps_limit as f64;
                app.update_world(phys_delta);

                let context = app.get_context();
                let time = &context.time;

                let delta = time.delta();
                //println!("{:?}", delta);
                let wait_time = time.get_time() + Duration::from_secs_f64(1.0 / app.settings.fps_limit as f64) - delta;

                *control_flow = ControlFlow::WaitUntil(wait_time);
                // Draw Frame!
                app.get_window().request_redraw();

                // match input
                app.match_input();
                //Update context
                app.get_context_mut().update();
            },
            _ => ()
        }
    })
}