use std::sync::Arc;
use vulkano::command_buffer::allocator::StandardCommandBufferAllocator;
use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;
use vulkano::device::Queue;
use vulkano::memory::allocator::StandardMemoryAllocator;
use vulkano::pipeline::compute::ComputePipelineCreateInfo;
use vulkano::pipeline::{ComputePipeline, PipelineShaderStageCreateInfo};

#[path="./world_system.rs"]
mod world_system;
use world_system::World;

//Rendering...