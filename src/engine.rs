use std::sync::Arc;
use std::time::Instant;

use vulkano::{instance::Instance, single_pass_renderpass, sync, Validated, VulkanError, VulkanLibrary};
use vulkano::command_buffer::allocator::StandardCommandBufferAllocator;
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, RenderPassBeginInfo, SubpassBeginInfo, SubpassContents, SubpassEndInfo};
use vulkano::device::{Device, DeviceCreateInfo, DeviceExtensions, QueueCreateInfo};
use vulkano::device::physical::PhysicalDevice;
use vulkano::instance::InstanceCreateInfo;
use vulkano::pipeline::graphics::viewport::Viewport;
use vulkano::swapchain::{acquire_next_image, Surface, Swapchain, SwapchainCreateInfo, SwapchainPresentInfo};
use vulkano::sync::GpuFuture;
use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

mod rendering;

mod world_system;
use world_system::World;
use crate::engine::rendering::window_size_dependent_setup;

// Engine Settings
pub struct Settings {
    title: &'static str,

    size: PhysicalSize<u32>,
    min_size: PhysicalSize<u32>,

    fps_limit: u64
}

impl Settings {
    pub fn new(title: &'static str, size: PhysicalSize<u32>, min_size: PhysicalSize<u32>, fps_limit: u64) -> Self {
        Self {title, size, min_size, fps_limit}
    }
}

impl Default for Settings {
    fn default() -> Self {
        let size = PhysicalSize::new(700, 500);
        let min_size = PhysicalSize::new(350, 250);

        Self {title: "DEngine", size, min_size, fps_limit: 120 }
    }
}

pub struct Engine {
    world: Arc<&'static mut World>,
    pub settings: Settings,

    instance: Arc<Instance>,
    event_loop: Option<EventLoop<()>>,
    window: Arc<Window>,
    surface: Arc<Surface>
}

impl Engine {
    pub fn new() -> &'static mut Self {
        let world = World::new("Empty world");
        let settings = Settings::default();

        let event_loop = EventLoop::new();

        print!("Creating Instance... ");
        let instance_init_time = Instant::now();
        let instance = {
            let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");
            let required_extensions = Surface::required_extensions(&event_loop);

            Instance::new(
                library,
                InstanceCreateInfo {
                    enabled_extensions: required_extensions,
                    ..Default::default()
                },
            ).expect("failed to create instance")
        };
        print!("Done! ({}s)\n", Instant::now().duration_since(instance_init_time).as_secs_f32());

        print!("Creating window... ");
        let window_init_time = Instant::now();
        let window = Arc::new(WindowBuilder::new().build(&event_loop).unwrap());
        print!("Done! ({}s)\n", Instant::now().duration_since(window_init_time).as_secs_f32());

        window.set_title(settings.title);
        window.set_inner_size(settings.size);
        window.set_min_inner_size(Some(settings.min_size));

        print!("Creating surface... ");
        let surface_init_time = Instant::now();
        let surface = Surface::from_window(instance.clone(), window.clone()).unwrap();
        print!("Done! ({}s)\n", Instant::now().duration_since(surface_init_time).as_secs_f32());

        Box::leak(Box::new(Self {
            world,
            settings,
            instance: instance.clone(),
            event_loop: Some(event_loop),
            window: window.clone(),
            surface: surface.clone()
        }))
    }

    pub fn set_world(&mut self, world: Arc<&'static mut World>) {
        self.world = world;
    }

    pub fn get_world(&self) -> &Arc<&mut World> {
        &self.world
    }

    pub fn draw_frame(&mut self) {

    }

    pub fn main_loop(&'static mut self) {
        println!("Run Main Loop...");

        let device_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::empty()
        };

        print!("Fetching physical device...");
        let phys_device_init_time = Instant::now();
        let (physical_device, queue_family_index) = rendering::select_physical_device(
            &self.instance.clone(),
            &self.surface.clone(),
            &device_extensions
        );
        print!("Done! ({}s)\n", Instant::now().duration_since(phys_device_init_time).as_secs_f32());

        println!(
            "Using device: {} (type: {:?})",
            physical_device.properties().device_name,
            physical_device.properties().device_type,
        );

        let (device, mut queues) = Device::new(
            physical_device.clone(),
            DeviceCreateInfo {
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index,
                    ..Default::default()
                }],
                enabled_extensions: device_extensions,
                ..Default::default()
            },
        ).expect("failed to create device");

        let queue = queues.next().unwrap();

        let (mut swapchain, images) = {
            let caps = device
                .physical_device()
                .surface_capabilities(&self.surface, Default::default())
                .unwrap();

            let usage = caps.supported_usage_flags;
            let alpha = caps.supported_composite_alpha
                .into_iter()
                .next()
                .unwrap();

            let image_format = Some(
                device
                    .physical_device()
                    .surface_formats(&self.surface, Default::default())
                    .unwrap()[0].0,
            );

            let image_extent: [u32; 2] = self.window.inner_size().into();

            Swapchain::new(
                device.clone(),
                self.surface.clone(),
                SwapchainCreateInfo {
                    min_image_count: caps.min_image_count,
                    image_format: image_format.unwrap(),
                    image_extent,
                    image_usage: usage,
                    composite_alpha: alpha,
                    ..Default::default()
                },
            ).unwrap()
        };

        let command_buffer_allocator =
            StandardCommandBufferAllocator::new(device.clone(), Default::default());

        let render_pass = single_pass_renderpass!(
            device.clone(),
            attachments: {
                color: {
                    format: swapchain.image_format(),
                    samples: 1,
                    load_op: Clear,
                    store_op: Store
                },
            },
            pass: {
                color: [color],
                depth_stencil: {}
            },
        ).unwrap();

        let mut viewport = Viewport {
            offset: [0.0, 0.0],
            extent: [0.0, 0.0],
            ..Viewport::default()
        };

        let mut framebuffers = window_size_dependent_setup(&images, render_pass.clone(), &mut viewport);

        let mut recreate_swapchain = false;
        let mut previous_frame_end = Some(sync::now(device.clone()).boxed());

        // Window events handler
        let handler = self.event_loop.take().unwrap();
        handler.run(move |event, _target, control_flow| {
            control_flow.set_poll();

            let start_time = Instant::now();

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    control_flow.set_exit();
                },
                Event::RedrawRequested(_) => {
                    // When need to draw frame

                    let image_extent: [u32; 2] = self.window.inner_size().into();
                    // If window size is zero: skip;
                    if image_extent.contains(&0) {
                        return;
                    }

                    let (image_index, suboptimal, acquire_future) =
                        match acquire_next_image(swapchain.clone(), None).map_err(Validated::unwrap) {
                            Ok(r) => r,
                            Err(VulkanError::OutOfDate) => {
                                recreate_swapchain = true;
                                return;
                            }
                            Err(e) => panic!("failed to acquire next image: {e}"),
                        };

                    if suboptimal {
                        recreate_swapchain = true;
                    }

                    let clear_values = vec![Some([0.0, 0.0, 0.0, 1.0].into())];

                    let mut builder = AutoCommandBufferBuilder::primary(
                        &command_buffer_allocator,
                        queue.queue_family_index(),
                        CommandBufferUsage::OneTimeSubmit,
                    ).unwrap();

                    builder
                        .begin_render_pass(
                            RenderPassBeginInfo {
                                clear_values,
                                ..RenderPassBeginInfo::framebuffer(
                                    framebuffers[image_index as usize].clone(),
                                )
                            },
                            SubpassBeginInfo::default(),
                        )
                        .unwrap()
                        .end_render_pass(SubpassEndInfo::default())
                        .unwrap();

                    let command_buffer = builder.build().unwrap();

                    let future = previous_frame_end
                        .take()
                        .unwrap()
                        .join(acquire_future)
                        .then_execute(queue.clone(), command_buffer)
                        .unwrap()
                        .then_swapchain_present(
                            queue.clone(),
                            SwapchainPresentInfo::swapchain_image_index(swapchain.clone(), image_index),
                        )
                        .then_signal_fence_and_flush();

                    match future.map_err(Validated::unwrap) {
                        Ok(future) => {
                            previous_frame_end = Some(future.boxed());
                        }
                        Err(VulkanError::OutOfDate) => {
                            recreate_swapchain = true;
                            previous_frame_end = Some(sync::now(device.clone()).boxed());
                        }
                        Err(e) => {
                            panic!("failed to flush future: {e}");
                            // previous_frame_end = Some(sync::now(device.clone()).boxed());
                        }
                    }

                    previous_frame_end.as_mut().unwrap().cleanup_finished();
                },
                Event::MainEventsCleared => {
                    let image_extent: [u32; 2] = self.window.inner_size().into();

                    if recreate_swapchain {
                        // Use the new dimensions of the window.

                        let (new_swapchain, new_images) = swapchain
                            .recreate(SwapchainCreateInfo {
                                image_extent,
                                ..swapchain.create_info()
                            })
                            .expect("failed to recreate swapchain");

                        swapchain = new_swapchain;

                        // Because framebuffers contains a reference to the old swapchain, we need to
                        // recreate framebuffers as well.
                        framebuffers = window_size_dependent_setup(
                            &new_images,
                            render_pass.clone(),
                            &mut viewport,
                        );

                        recreate_swapchain = false;
                    }
                },

                _ => ()
            }

            match *control_flow {
                ControlFlow::Exit => {
                    println!("Closing Application...");
                },
                ControlFlow::Poll => {
                    self.window.request_redraw();

                    let delta = {
                        let elapsed_time = Instant::now().duration_since(start_time).as_nanos() as u64;

                        //println!("E time: {}", Instant::now().duration_since(start_time).as_secs_f32());
                        match 1_000_000_000 / self.settings.fps_limit >= elapsed_time {
                            true => 1_000_000_000 / self.settings.fps_limit - elapsed_time,
                            false => 0
                        }
                    };

                    // TODO: Fps counter
                    //self.window.set_title(&*format!("DEngine fps: {}", fps));

                    let new_inst = start_time + std::time::Duration::from_nanos(delta);
                    *control_flow = ControlFlow::WaitUntil(new_inst); // Waiting in NS
                    //println!("delta: {}", delta);
                },
                _ => ()
            }
        });
    }
}