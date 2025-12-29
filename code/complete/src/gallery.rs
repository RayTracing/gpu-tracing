// Originally written in 2025 by Arman Uguray <arman.uguray@gmail.com>
// SPDX-License-Identifier: CC-BY-4.0

use crate::{
    algebra::Vec3,
    camera::Camera,
    scene::{Material, SceneBuilder, Sphere},
};

pub struct Gallery {
    current_scene_index: usize,
    scenes: Vec<Scene>,
}

pub struct Scene {
    pub camera: Camera,
    pub resources: wgpu::BindGroup,
}

impl Gallery {
    pub fn new(device: &wgpu::Device, layout: &wgpu::BindGroupLayout) -> Self {
        let scenes = vec![scene_with_spheres(), another_scene_with_spheres()];
        Self {
            current_scene_index: scenes.len() - 1,
            scenes: scenes
                .into_iter()
                .map(|(camera, builder)| Scene {
                    camera,
                    resources: builder.build(device, layout),
                })
                .collect(),
        }
    }

    pub fn current_scene(&self) -> &Scene {
        &self.scenes[self.current_scene_index]
    }

    pub fn current_camera_mut(&mut self) -> &mut Camera {
        &mut self.scenes[self.current_scene_index].camera
    }

    pub fn select_next(&mut self) {
        self.current_scene_index += 1;
        self.current_scene_index %= self.scenes.len();
    }

    pub fn select_previous(&mut self) {
        if self.current_scene_index == 0 {
            self.current_scene_index = self.scenes.len() - 1;
        } else {
            self.current_scene_index -= 1;
        }
    }
}

#[rustfmt::skip]
fn scene_with_spheres() -> (Camera, SceneBuilder) {
    let mut builder = SceneBuilder::default();

    let glass = builder.add_material(Material::transparent_dielectric(Vec3::all(1.), 1.5));
    let diffuse = builder.add_material(Material::lambertian(Vec3::new(0.5, 0.5, 0.9)));
    let metal = builder.add_material(Material::metal(Vec3::new(0.7, 0.5, 0.5)));
    let cornflower = builder.add_material(Material::opaque(Vec3::new(0.3, 0.6, 0.9), 0.9));
    let turquoise = builder.add_material(Material::opaque(Vec3::new(0.3, 0.9, 0.7), 0.3));
    let magenta = builder.add_material(Material::opaque(Vec3::new(0.9, 0.3, 0.7), 0.001));
    let ground = builder.add_material(Material::lambertian(Vec3::new(0.7, 0.9, 0.2)));

    builder.add_sphere(Sphere { center: Vec3::new(0., 0.5, 1.6), radius: 0.5 }, glass);
    builder.add_sphere(Sphere { center: Vec3::new(0., 0.7, 0.), radius: 0.7 }, metal);
    builder.add_sphere(
        Sphere { center: Vec3::new(-1.522, 0.5, 0.494), radius: 0.5 },
        diffuse
    );
    builder.add_sphere(
        Sphere { center: Vec3::new(-0.941, 0.5, -1.294), radius: 0.5 },
        cornflower
    );
    builder.add_sphere(
        Sphere { center: Vec3::new(0.941, 0.5, -1.294), radius: 0.5 },
        turquoise
    );
    builder.add_sphere(
        Sphere { center: Vec3::new(1.522, 0.5, 0.494), radius: 0.5 },
        magenta
    );

    // Ground
    builder.add_sphere(Sphere { center: Vec3::new(0., -200.001, 0.), radius: 200. }, ground);

    let camera = Camera::look_at(
        Vec3::new(2.8577466, 5.002403, 3.9194741),
        Vec3::new(0., 0., 0.),
        Vec3::new(0., 1., 0.),
    )
    .with_fov(30_f32.to_radians());

    (camera, builder)
}

#[rustfmt::skip]
fn another_scene_with_spheres() -> (Camera, SceneBuilder) {
    let mut builder = SceneBuilder::default();

    let blue = builder.add_material(Material::opaque(Vec3::new(0., 0.2, 0.9), 0.001));
    let metal = builder.add_material(Material::metal(Vec3::new(0.8, 0.3, 0.3)));
    let glass = builder.add_material(Material::transparent_dielectric(Vec3::all(1.), 1.5));
    let ground = builder.add_material(Material::lambertian(Vec3::new(1., 0.8, 0.1)));

    builder.add_sphere(Sphere { center: Vec3::new(0., 0.5, 1.6), radius: 0.5 }, blue);
    builder.add_sphere(Sphere { center: Vec3::new(0., 1.1, 1.6), radius: 0.1 }, glass);
    builder.add_sphere(Sphere { center: Vec3::new(0., 0.2, 0.5), radius: 0.2 }, metal);

    builder.add_sphere(
        Sphere { center: Vec3::new(-1.522, 0.5, 0.494), radius: 0.5 },
        blue
    );
    builder.add_sphere(
        Sphere { center: Vec3::new(-1.522, 1.1, 0.494), radius: 0.1 },
        glass
    );
    builder.add_sphere(
        Sphere { center: Vec3::new(-0.476, 0.2, 0.154), radius: 0.2 },
        metal
    );

    builder.add_sphere(
        Sphere { center: Vec3::new(-0.941, 0.5, -1.294), radius: 0.5 },
        blue
    );
    builder.add_sphere(
        Sphere { center: Vec3::new(-0.941, 1.1, -1.294), radius: 0.1 },
        glass
    );
    builder.add_sphere(
        Sphere { center: Vec3::new(-0.294, 0.2, -0.404), radius: 0.2 },
        metal
    );

    builder.add_sphere(
        Sphere { center: Vec3::new(0.941, 0.5, -1.294), radius: 0.5 },
        blue
    );
    builder.add_sphere(
        Sphere { center: Vec3::new(0.941, 1.1, -1.294), radius: 0.1 },
        glass
    );
    builder.add_sphere(
        Sphere { center: Vec3::new(0.294, 0.2, -0.404), radius: 0.2 },
        metal
    );

    builder.add_sphere(
        Sphere { center: Vec3::new(1.522, 0.5, 0.494), radius: 0.5 },
        blue
    );
    builder.add_sphere(
        Sphere { center: Vec3::new(1.522, 1.1, 0.494), radius: 0.1 },
        glass
    );
    builder.add_sphere(
        Sphere { center: Vec3::new(0.476, 0.2, 0.154), radius: 0.2 },
        metal
    );

    // Ground
    builder.add_sphere(Sphere { center: Vec3::new(0., -200.001, 0.), radius: 200. }, ground);

    let camera = Camera::look_at(
        Vec3::new(3.3252046, 1.7924162, 4.541867),
        Vec3::new(-0.052276053, 0.32371068, -0.09043576),
        Vec3::new(0., 1., 0.),
    )
    .with_fov(30_f32.to_radians());

    (camera, builder)
}
