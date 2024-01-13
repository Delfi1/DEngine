use std::sync::Arc;
use std::thread;
use std::time::Instant;

use vulkano::{instance::Instance, VulkanLibrary};
use vulkano::device::{Device, DeviceCreateInfo, DeviceExtensions, QueueCreateInfo};
use vulkano::device::physical::PhysicalDevice;
use vulkano::instance::InstanceCreateInfo;
use vulkano::swapchain::Surface;
use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

mod rendering;

mod world_system;
use world_system::World;

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

        print!("Initializing Vulkan library... ");
        let lib_init_time = Instant::now();
        let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");
        let required_extensions = Surface::required_extensions(&event_loop);
        print!("Done! ({}s)\n", Instant::now().duration_since(lib_init_time).as_secs_f32());

        print!("Creating Instance... ");
        let instance_init_time = Instant::now();
        let instance = Instance::new(
            library,
            InstanceCreateInfo {
                enabled_extensions: required_extensions,
                ..Default::default()
            },
        ).expect("failed to create instance");
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
        let mut world = self.world.clone();

        thread::spawn(move || {

        });
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

                _ => ()
            }

            match *control_flow {
                ControlFlow::Exit => {
                    println!("Closing Application...");
                },
                ControlFlow::Poll => {
                    let delta = {
                        let elapsed_time = Instant::now().duration_since(start_time).as_nanos() as u64;

                        //println!("E time: {}", Instant::now().duration_since(start_time).as_secs_f32());
                        match 1_000_000_000 / self.settings.fps_limit >= elapsed_time {
                            true => 1_000_000_000 / self.settings.fps_limit - elapsed_time,
                            false => 0
                        }
                    };

                    let new_inst = start_time + std::time::Duration::from_nanos(delta);
                    *control_flow = ControlFlow::WaitUntil(new_inst); // Waiting in NS
                    //println!("delta: {}", delta);
                },
                _ => ()
            }
        });
    }
}