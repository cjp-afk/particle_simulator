#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct Particle {
    // For demonstration: position, velocity, radius, color
    pub(crate) position: [f32; 2],
    pub(crate) velocity: [f32; 2],
    pub(crate) radius: f32,
    pub _pad0: [f32; 3],
    pub(crate) color: [f32; 4],
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

const SIZE: f32 = 0.1;

pub const QUAD_VERTICES: &[Vertex] = &[
    Vertex { position: [-SIZE, -SIZE], tex_coords: [0.0, 0.0] },
    Vertex { position: [-SIZE,  SIZE], tex_coords: [0.0, 1.0] },
    Vertex { position: [ SIZE,  SIZE], tex_coords: [1.0, 1.0] },
    Vertex { position: [ SIZE, -SIZE], tex_coords: [1.0, 0.0] },
];

pub const QUAD_INDICES: &[u16] = &[
    0, 1, 2,
    0, 2, 3,
];

impl Vertex {
    pub(crate) fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // position: @location(0)
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                // tex_coords: @location(1)
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct Instance {
    position: [f32; 2],
    radius: f32,
    color: [f32; 4],
}

// But let's assume our buffer is laid out exactly as Particle, with some offsets:
impl Instance {
    pub(crate) fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Particle>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                // position: @location(2)
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                },
                // radius: @location(3)
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 2]>() as wgpu::BufferAddress
                        + mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    // skip velocity as well => offset = 4 floats = 16 bytes
                    format: wgpu::VertexFormat::Float32,
                    shader_location: 3,
                },
                // color: @location(4)
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 2]>() as wgpu::BufferAddress
                        + mem::size_of::<[f32; 2]>() as wgpu::BufferAddress
                        + mem::size_of::<f32>() as wgpu::BufferAddress
                        + mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    format: wgpu::VertexFormat::Float32x4,
                    shader_location: 4,
                }
            ],
        }
    }
}