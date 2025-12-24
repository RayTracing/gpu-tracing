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

#[derive(Debug, Copy, Clone, Pod, Zeroable)]
#[repr(C)]
struct SphereBufferEntry {
    center: Vec3,
    radius: f32,
    material_index: u32,
    _pad: [u32; 3],
}

pub struct MaterialId(u32);

#[derive(Default)]
pub struct SceneBuilder {
    materials: Vec<Material>,
    spheres: Vec<SphereBufferEntry>,
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

    pub fn build(
        self,
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
    ) -> wgpu::BindGroup {
        let material_buffer =
            create_storage_buffer_with_data(device, &self.materials, Some("materials"));
        let spheres_buffer =
            create_storage_buffer_with_data(device, &self.spheres, Some("spheres"));

        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("scene resources"),
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &material_buffer,
                        offset: 0,
                        size: None,
                    }),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &spheres_buffer,
                        offset: 0,
                        size: None,
                    }),
                },
            ],
        })
    }
}

fn create_storage_buffer_with_data<T: Pod>(
    device: &wgpu::Device,
    data: &Vec<T>,
    label: Option<&str>,
) -> wgpu::Buffer {
    // Create the buffer already mapped and initialize data without an explicit transfer.
    let buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label,
        size: (std::mem::size_of::<T>() * data.len()) as u64,
        usage: wgpu::BufferUsages::STORAGE,
        mapped_at_creation: true,
    });
    // Copy the data into the buffer.
    {
        let mut view = buffer.slice(..).get_mapped_range_mut();
        view.as_mut()
            .copy_from_slice(bytemuck::cast_slice(data.as_slice()));
    }
    buffer.unmap();
    buffer
}
