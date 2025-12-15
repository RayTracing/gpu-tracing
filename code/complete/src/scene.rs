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

// TODO:
pub struct MaterialId(u32);

#[derive(Default)]
pub struct SceneBuilder {
    materials: Vec<Material>,
}

impl SceneBuilder {
    pub fn add_material(&mut self, material: Material) {
        // TODO: -> MaterialId {
        self.materials.push(material);
        // TODO: MaterialId((self.materials.len() - 1).try_into().unwrap())
    }

    pub fn build(
        self,
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
    ) -> wgpu::BindGroup {
        // Create the buffer already mapped and initialize data without an explicit transfer.
        let material_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("materials"),
            size: (std::mem::size_of::<Material>() * self.materials.len()) as u64,
            usage: wgpu::BufferUsages::STORAGE,
            mapped_at_creation: true,
        });
        // Copy the data into the buffer.
        {
            let mut view = material_buffer.slice(..).get_mapped_range_mut();
            view.as_mut()
                .copy_from_slice(bytemuck::cast_slice(self.materials.as_slice()));
        }
        material_buffer.unmap();

        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("scene resources"),
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &material_buffer,
                    offset: 0,
                    size: None,
                }),
            }],
        })
    }
}
