// Originally written in 2025 by Arman Uguray <arman.uguray@gmail.com>
// SPDX-License-Identifier: CC-BY-4.0

use bytemuck::{Pod, Zeroable};

use crate::algebra::Vec3;

#[derive(Debug, Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct Material {
    color: Vec3,
    metallic_or_ior: f32,
}

impl Material {
    pub fn lambertian(color: Vec3) -> Self {
        Self {
            color,
            metallic_or_ior: 0.,
        }
    }

    pub fn opaque(color: Vec3, metallic: f32) -> Self {
        Self {
            color,
            metallic_or_ior: metallic.max(0.00001),
        }
    }

    pub fn transparent_dielectric(color: Vec3, ior: f32) -> Self {
        Self {
            color,
            metallic_or_ior: -ior.abs(),
        }
    }

    pub fn metal(color: Vec3) -> Self {
        Self::opaque(color, 1.)
    }
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

pub struct Disc {
    pub center: Vec3,
    pub radius: f32,
    pub normal: Vec3,
}

pub struct Box {
    pub origin: Vec3,
    pub extent: Vec3,
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,
}

#[derive(Debug, Copy, Clone, Pod, Zeroable)]
#[repr(C)]
struct SceneUniforms {
    sphere_count: u32,
    disc_count: u32,
    box_count: u32,
}

#[derive(Debug, Copy, Clone, Pod, Zeroable)]
#[repr(C)]
struct SphereBufferEntry {
    center: Vec3,
    radius: f32,
    material_index: u32,
    _pad: [u32; 3],
}

#[derive(Debug, Copy, Clone, Pod, Zeroable)]
#[repr(C)]
struct DiscBufferEntry {
    center: Vec3,
    radius: f32,
    normal: Vec3,
    material_index: u32,
}

#[derive(Debug, Copy, Clone, Pod, Zeroable)]
#[repr(C)]
struct BoxBufferEntry {
    origin: Vec3,
    _pad0: u32,
    extent: Vec3,
    _pad1: u32,
    v0: Vec3,
    _pad2: u32,
    v1: Vec3,
    _pad3: u32,
    v2: Vec3,
    material_index: u32,
}

#[derive(Copy, Clone)]
pub struct MaterialId(u32);

#[derive(Default)]
pub struct SceneBuilder {
    materials: Vec<Material>,
    spheres: Vec<SphereBufferEntry>,
    discs: Vec<DiscBufferEntry>,
    boxes: Vec<BoxBufferEntry>,
}

impl SceneBuilder {
    pub fn add_material(&mut self, material: Material) -> MaterialId {
        self.materials.push(material);
        MaterialId((self.materials.len() - 1).try_into().unwrap())
    }

    pub fn add_sphere(&mut self, sphere: Sphere, material: MaterialId) {
        let entry = SphereBufferEntry {
            center: sphere.center,
            radius: sphere.radius,
            material_index: material.0,
            _pad: [0; 3],
        };
        self.spheres.push(entry)
    }

    pub fn add_disc(&mut self, disc: Disc, material: MaterialId) {
        let entry = DiscBufferEntry {
            center: disc.center,
            radius: disc.radius,
            normal: disc.normal,
            material_index: material.0,
        };
        self.discs.push(entry)
    }

    pub fn add_box(&mut self, box_: Box, material: MaterialId) {
        let entry = BoxBufferEntry {
            origin: box_.origin,
            _pad0: 0,
            extent: box_.extent,
            _pad1: 0,
            v0: box_.v0,
            _pad2: 0,
            v1: box_.v1,
            _pad3: 0,
            v2: box_.v2,
            material_index: material.0,
        };
        self.boxes.push(entry)
    }

    pub fn build(
        self,
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
    ) -> wgpu::BindGroup {
        let uniform_buffer = create_uniform_buffer_with_data(
            device,
            SceneUniforms {
                sphere_count: self.spheres.len() as u32,
                disc_count: self.discs.len() as u32,
                box_count: self.boxes.len() as u32,
            },
            Some("scene uniforms")
        );
        let material_buffer =
            create_storage_buffer_with_data(device, &self.materials, Some("materials"));
        let spheres_buffer =
            create_storage_buffer_with_data(device, &self.spheres, Some("spheres"));
        let discs_buffer =
            create_storage_buffer_with_data(device, &self.discs, Some("discs"));
        let boxes_buffer =
            create_storage_buffer_with_data(device, &self.boxes, Some("boxes"));

        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("scene resources"),
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &uniform_buffer,
                        offset: 0,
                        size: None,
                    }),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &material_buffer,
                        offset: 0,
                        size: None,
                    }),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &spheres_buffer,
                        offset: 0,
                        size: None,
                    }),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &discs_buffer,
                        offset: 0,
                        size: None,
                    }),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &boxes_buffer,
                        offset: 0,
                        size: None,
                    }),
                },
            ],
        })
    }
}

fn create_uniform_buffer_with_data<T: Pod>(
    device: &wgpu::Device,
    data: T,
    label: Option<&str>,
) -> wgpu::Buffer {
    // Create the buffer already mapped and initialize data without an explicit transfer.
    let buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label,
        size: std::mem::size_of::<T>() as u64,
        usage: wgpu::BufferUsages::UNIFORM,
        mapped_at_creation: true,
    });
    // Copy the data into the buffer.
    {
        let mut view = buffer.slice(..).get_mapped_range_mut();
        view.as_mut().copy_from_slice(bytemuck::bytes_of(&data));
    }
    buffer.unmap();
    buffer
}

fn create_storage_buffer_with_data<T: Pod>(
    device: &wgpu::Device,
    data: &Vec<T>,
    label: Option<&str>,
) -> wgpu::Buffer {
    // Create the buffer already mapped and initialize data without an explicit transfer.
    let buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label,
        size: (std::mem::size_of::<T>() * data.len().max(1)) as u64,
        usage: wgpu::BufferUsages::STORAGE,
        mapped_at_creation: true,
    });
    // Copy the data into the buffer.
    if data.len() > 0 {
        let mut view = buffer.slice(..).get_mapped_range_mut();
        view.as_mut()
            .copy_from_slice(bytemuck::cast_slice(data.as_slice()));
    }
    buffer.unmap();
    buffer
}
