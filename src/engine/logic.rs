use std::sync::Arc;
use vulkano::command_buffer::allocator::StandardCommandBufferAllocator;
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferInheritanceInfo, CommandBufferLevel, CommandBufferUsage, PrimaryCommandBufferAbstract, RenderPassBeginInfo, SecondaryAutoCommandBuffer, SubpassBeginInfo, SubpassContents};
use vulkano::command_buffer::ResourceInCommand::DescriptorSet;
use vulkano::command_buffer::sys::CommandBufferBeginInfo;
use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;
use vulkano::descriptor_set::WriteDescriptorSet;
use vulkano::device::Queue;
use vulkano::format::Format;
use vulkano::image::sampler::{Filter, Sampler, SamplerAddressMode, SamplerCreateInfo, SamplerMipmapMode};
use vulkano::image::view::ImageView;
use vulkano::memory::allocator::StandardMemoryAllocator;
use vulkano::pipeline::{ComputePipeline, DynamicState, GraphicsPipeline, Pipeline, PipelineBindPoint, PipelineLayout, PipelineShaderStageCreateInfo};
use vulkano::pipeline::compute::ComputePipelineCreateInfo;
use vulkano::pipeline::graphics::color_blend::{ColorBlendAttachmentState, ColorBlendState};
use vulkano::pipeline::graphics::GraphicsPipelineCreateInfo;
use vulkano::pipeline::graphics::input_assembly::InputAssemblyState;
use vulkano::pipeline::graphics::multisample::MultisampleState;
use vulkano::pipeline::graphics::rasterization::RasterizationState;
use vulkano::pipeline::graphics::vertex_input::VertexInputState;
use vulkano::pipeline::graphics::viewport::{Viewport, ViewportState};
use vulkano::pipeline::layout::{PipelineDescriptorSetLayoutCreateInfo, PipelineLayoutCreateInfo};
use vulkano::render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass, Subpass};
use vulkano::sync::GpuFuture;

pub struct EnginePipeline {
    queue: Arc<Queue>,
    pipeline: Arc<ComputePipeline>,
    memory_allocator: Arc<StandardMemoryAllocator>,
    command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
    descriptor_set_allocator: Arc<StandardDescriptorSetAllocator>,
}

impl EnginePipeline {
    pub fn new(
        queue: Arc<Queue>,
        memory_allocator: Arc<StandardMemoryAllocator>,
        command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
        descriptor_set_allocator: Arc<StandardDescriptorSetAllocator>
    ) -> Self {
        let pipeline = {
            let device = queue.device();

            let cs = cs::load(device.clone())
                .unwrap()
                .entry_point("main")
                .unwrap();
            let stage = PipelineShaderStageCreateInfo::new(cs);
            let layout = PipelineLayout::new(
                device.clone(),
                PipelineDescriptorSetLayoutCreateInfo::from_stages([&stage])
                    .into_pipeline_layout_create_info(device.clone())
                    .unwrap(),
            ).unwrap();

            ComputePipeline::new(
                device.clone(),
                None,
                ComputePipelineCreateInfo::stage_layout(stage, layout)
            )
        }.unwrap();

        Self {queue, pipeline, memory_allocator, command_buffer_allocator, descriptor_set_allocator}
    }

    pub fn compute(&self, image_view: Arc<ImageView>) -> Box<dyn GpuFuture> {
        let image_extent = image_view.image().extent();

        let mut builder = AutoCommandBufferBuilder::primary(
            &self.command_buffer_allocator.clone(),
            self.queue.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit
        ).unwrap();

        builder
            .bind_pipeline_compute(self.pipeline.clone())
            .unwrap()
            .dispatch([image_extent[0] / 8, image_extent[1] / 8, 1])
            .unwrap();

        let command_buffer = builder.build().unwrap();

        let finished = command_buffer.execute(self.queue.clone()).unwrap();
        finished.then_signal_fence_and_flush().unwrap().boxed()
    }
}

pub struct PhysicsPipeline {

}

pub struct DrawingPipeline {
    gfx_queue: Arc<Queue>,
    pipeline: Arc<GraphicsPipeline>,
    subpass: Subpass,
    command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
    descriptor_set_allocator: Arc<StandardDescriptorSetAllocator>
}

impl DrawingPipeline {
    pub fn new(
        gfx_queue: Arc<Queue>,
        subpass: Subpass,
        command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
        descriptor_set_allocator: Arc<StandardDescriptorSetAllocator>
    ) -> Self {
        let device = gfx_queue.device();

        let vs = vs::load(device.clone())
            .expect("failed to create shader module")
            .entry_point("main")
            .expect("shader entry point not found");
        let fs = fs::load(device.clone())
            .expect("failed to create shader module")
            .entry_point("main")
            .expect("shader entry point not found");
        let stages = [
            PipelineShaderStageCreateInfo::new(vs),
            PipelineShaderStageCreateInfo::new(fs),
        ];
        let layout = PipelineLayout::new(
            device.clone(),
            PipelineDescriptorSetLayoutCreateInfo::from_stages(&stages)
                .into_pipeline_layout_create_info(device.clone())
                .unwrap(),
        )
            .unwrap();

        let pipeline = GraphicsPipeline::new(
            device.clone(),
            None,
            GraphicsPipelineCreateInfo {
                stages: stages.into_iter().collect(),
                vertex_input_state: Some(VertexInputState::default()),
                input_assembly_state: Some(InputAssemblyState::default()),
                viewport_state: Some(ViewportState::default()),
                rasterization_state: Some(RasterizationState::default()),
                multisample_state: Some(MultisampleState::default()),
                color_blend_state: Some(ColorBlendState::with_attachment_states(
                    subpass.num_color_attachments(),
                    ColorBlendAttachmentState::default(),
                )),
                dynamic_state: [DynamicState::Viewport].into_iter().collect(),
                subpass: Some(subpass.clone().into()),
                ..GraphicsPipelineCreateInfo::layout(layout)
            }
        ).unwrap();

        Self {
            gfx_queue,
            pipeline,
            subpass,
            command_buffer_allocator,
            descriptor_set_allocator
        }
    }

    /// Draws input `image` over a quad of size -1.0 to 1.0.
    pub fn draw(&self, viewport_dimensions: [u32; 2], image: Arc<ImageView>) -> Arc<SecondaryAutoCommandBuffer<Arc<StandardCommandBufferAllocator>>> {
        let inheritance_info = CommandBufferInheritanceInfo {
            render_pass: Some(self.subpass.clone().into()),
            ..Default::default()
        };

        let mut builder = AutoCommandBufferBuilder::secondary(
            &self.command_buffer_allocator.clone(),
            self.gfx_queue.queue_family_index(),
            CommandBufferUsage::MultipleSubmit,
            inheritance_info
        ).unwrap();

        builder
            .set_viewport(
                0,
                [Viewport {
                    offset: [0.0, 0.0],
                    extent: [viewport_dimensions[0] as f32, viewport_dimensions[1] as f32],
                    depth_range: 0.0..=1.0,
                }].into_iter().collect(),
            )
            .unwrap()
            .bind_pipeline_graphics(self.pipeline.clone())
            .unwrap();

        unsafe {
            builder.draw(6, 1, 0, 0).unwrap();
        }

        builder.build().unwrap()
    }
}

pub struct PlaceOverFrame {
    gfx_queue: Arc<Queue>,
    render_pass: Arc<RenderPass>,
    drawing_pipeline: DrawingPipeline,
    command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
}

struct RecordingCommandBuffer(Arc<StandardCommandBufferAllocator>, u32, CommandBufferLevel, CommandBufferBeginInfo);

impl PlaceOverFrame {
    pub fn new(
        gfx_queue: Arc<Queue>,
        output_format: Format,
        command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
        descriptor_set_allocator: Arc<StandardDescriptorSetAllocator>
    ) -> Self {
        let render_pass = vulkano::single_pass_renderpass!(
            gfx_queue.device().clone(),
            attachments: {
                color: {
                    format: output_format,
                    samples: 1,
                    load_op: Clear,
                    store_op: Store,
                },
            },
            pass: {
                color: [color],
                depth_stencil: {},
            },
        ).unwrap();

        let subpass = Subpass::from(render_pass.clone(), 0).unwrap();

        let drawing_pipeline = DrawingPipeline::new(
            gfx_queue.clone(),
            subpass,
            command_buffer_allocator.clone(),
            descriptor_set_allocator.clone()
        );

        Self {
            gfx_queue,
            render_pass,
            drawing_pipeline,
            command_buffer_allocator
        }
    }

    pub fn render<F>(
        &self,
        before_future: F,
        view: Arc<ImageView>,
        target: Arc<ImageView>,
    ) -> Box<dyn GpuFuture>
        where
            F: GpuFuture + 'static,
    {
        // Get dimensions.
        let img_dims: [u32; 2] = target.image().extent()[0..2].try_into().unwrap();

        // Create framebuffer;
        let framebuffer = Framebuffer::new(
            self.render_pass.clone(),
            FramebufferCreateInfo {
                attachments: vec![target],
                ..Default::default()
            },
        ).unwrap();

        // Create primary command buffer builder.

        let mut command_buffer_builder = AutoCommandBufferBuilder::primary(
            &self.command_buffer_allocator.clone(),
            self.gfx_queue.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit
        ).unwrap();

        // Begin render pass.
        command_buffer_builder
            .begin_render_pass(
                RenderPassBeginInfo {
                    clear_values: vec![Some([0.0; 4].into())],
                    ..RenderPassBeginInfo::framebuffer(framebuffer)
                },
                SubpassBeginInfo {
                    contents: SubpassContents::SecondaryCommandBuffers,
                    ..Default::default()
                },
            )
            .unwrap();

        // Create secondary command buffer from texture pipeline & send draw commands.
        let cb = self.drawing_pipeline.draw(img_dims, view);

        // Execute above commands (subpass).
        command_buffer_builder.execute_commands(cb).unwrap();

        // End render pass.
        command_buffer_builder
            .end_render_pass(Default::default())
            .unwrap();

        // Build command buffer.
        let command_buffer = command_buffer_builder.build().unwrap();

        // Execute primary command buffer.
        let after_future = before_future
            .then_execute(self.gfx_queue.clone(), command_buffer)
            .unwrap();

        after_future.boxed()
    }
}

mod vs {
    use vulkano_shaders::shader;
    shader! {
        ty: "vertex",
        src: r"
            #version 460

            void main() {

            }
        "
    }
}

mod fs {
    use vulkano_shaders::shader;
    shader! {
        ty: "fragment",
        src: r"
            #version 460

            void main() {

            }
        "
    }
}

mod cs {
    use vulkano_shaders::shader;
    shader! {
        ty: "compute",
        src: r"
            #version 460

            void main() {

            }
        ",
    }
}