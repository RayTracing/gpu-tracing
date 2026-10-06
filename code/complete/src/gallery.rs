// Originally written in 2025 by Arman Uguray <arman.uguray@gmail.com>
// SPDX-License-Identifier: CC-BY-4.0

use crate::{
    algebra::Vec3,
    camera::Camera,
    scene::{Box, Disc, Material, SceneBuilder, Sphere},
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
        let scenes = vec![
            scene_with_spheres(),
            another_scene_with_spheres(),
            scene_with_discs(),
            scene_with_boxes(),
        ];
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

#[rustfmt::skip]
fn scene_with_discs() -> (Camera, SceneBuilder) {
    let mut builder = SceneBuilder::default();

    let ground = builder.add_material(Material::lambertian(Vec3::new(1., 0.8, 0.1)));
    let sphere = builder.add_material(Material::opaque(Vec3::new(0.9, 0.2, 0.4), 0.001));
    let mirror = builder.add_material(Material::metal(Vec3::all(0.8)));

    let sphere_position = Vec3::new(0., 0.5, 0.);
    builder.add_sphere(Sphere { center: sphere_position, radius: 0.5 }, sphere);
    builder.add_disc(
        Disc {
            center: Vec3::all(0.),
            radius: 1000.,
            normal: Vec3::new(0., 1., 0.)
        },
        ground
    );
    builder.add_disc(
        Disc {
            center: Vec3::new(-0.8, 1., -0.8),
            radius: 1.,
            normal: Vec3::new(1., -0.3, 1.).normalized()
        },
        mirror
    );
    builder.add_disc(
        Disc {
            center: Vec3::new(0.8, 1., -0.8),
            radius: 1.,
            normal: Vec3::new(-1., -0.3, 1.).normalized()
        },
        mirror
    );

    let camera = Camera::look_at(
        Vec3::new(0., 1.2, 2.5),
        Vec3::new(0., 0.5, 0.),
        Vec3::new(0., 1., 0.)
    ).with_fov(60_f32.to_radians());

    (camera, builder)
}

#[rustfmt::skip]
fn scene_with_boxes() -> (Camera, SceneBuilder) {
    let mut builder = SceneBuilder::default();

    let ground = builder.add_material(Material::lambertian(Vec3::new(1., 0.8, 0.1)));
    let magenta = builder.add_material(Material::opaque(Vec3::new(0.9, 0.2, 0.4), 0.001));
    let red = builder.add_material(Material::metal(Vec3::new(0.8, 0.1, 0.1)));
    let matte = builder.add_material(Material::lambertian(Vec3::all(0.8)));
    let glass = builder.add_material(
        Material::transparent_dielectric(Vec3::new(0.8, 1., 0.9), 1.5)
    );

    let crimson_frame = builder.add_material(
        Material::lambertian(Vec3::new(0.6, 0.16, 0.16))
    );
    let blue_frame = builder.add_material(Material::lambertian(Vec3::new(0.16, 0.16, 0.6)));
    let mirror = builder.add_material(Material::metal(Vec3::all(0.95)));
    let green = builder.add_material(Material::opaque(Vec3::new(0.2, 0.9, 0.4), 0.2));

    builder.add_disc(
        Disc {
            center: Vec3::all(0.),
            radius: 1000.,
            normal: Vec3::new(0., 1., 0.)
        },
        ground
    );
    builder.add_box(
        Box {
            origin: Vec3::new(-1.5, 0., -1.5),
            extent: Vec3::new(3., 0.25, 3.),
            v0: Vec3::new(1., 0., 0.),
            v1: Vec3::new(0., 1., 0.),
            v2: Vec3::new(0., 0., 1.),
        },
        magenta
    );
    builder.add_box(
        Box {
            origin: Vec3::new(-1., 0.25, -1.),
            extent: Vec3::new(0.75, 1., 0.75),
            v0: Vec3::new(1., 0., 0.),
            v1: Vec3::new(0., 1., 0.),
            v2: Vec3::new(0., 0., 1.),
        },
        matte
    );
    builder.add_box(
        Box {
            origin: Vec3::new(0.25, 0.25, -1.),
            extent: Vec3::new(0.75, 1., 0.75),
            v0: Vec3::new(1., 0., 0.),
            v1: Vec3::new(0., 1., 0.),
            v2: Vec3::new(0., 0., 1.),
        },
        matte
    );
    builder.add_box(
        Box {
            origin: Vec3::new(-1., 0.25, 0.25),
            extent: Vec3::new(0.5, 0.5, 0.75),
            v0: Vec3::new(1., 0., 0.),
            v1: Vec3::new(0., 1., 0.),
            v2: Vec3::new(0., 0., 1.),
        },
        matte
    );
    builder.add_box(
        Box {
            origin: Vec3::new(0.5, 0.25, 0.25),
            extent: Vec3::new(0.5, 0.5, 0.75),
            v0: Vec3::new(1., 0., 0.),
            v1: Vec3::new(0., 1., 0.),
            v2: Vec3::new(0., 0., 1.),
        },
        matte
    );
    builder.add_box(
        Box {
            origin: Vec3::new(-1., 1.25, -1.),
            extent: Vec3::new(2., 0.2, 0.75),
            v0: Vec3::new(1., 0., 0.),
            v1: Vec3::new(0., 1., 0.),
            v2: Vec3::new(0., 0., 1.),
        },
        matte
    );
    builder.add_box(
        Box {
            origin: Vec3::new(-1., 0.75, 0.25),
            extent: Vec3::new(2., 0.1, 0.75),
            v0: Vec3::new(1., 0., 0.),
            v1: Vec3::new(0., 1., 0.),
            v2: Vec3::new(0., 0., 1.),
        },
        glass
    );
    builder.add_box(
        Box {
            origin: Vec3::new(-0.4, 0.85, 0.5),
            extent: Vec3::new(0.2, 0.2, 0.2),
            v0: Vec3::new(1., 0., 1.).normalized(),
            v1: Vec3::new(0., 1., 0.),
            v2: Vec3::new(-1., 0., 1.).normalized(),
        },
        red
    );
    builder.add_disc(
        Disc {
            center: Vec3::new(-0.6, 1., -0.249),
            radius: 0.25,
            normal: Vec3::new(0., 0., 1.)
        },
        crimson_frame
    );
    builder.add_disc(
        Disc {
            center: Vec3::new(0.6, 1., -0.249),
            radius: 0.25,
            normal: Vec3::new(0., 0., 1.)
        },
        blue_frame
    );
    builder.add_disc(
        Disc {
            center: Vec3::new(-0.6, 1., -0.245),
            radius: 0.2,
            normal: Vec3::new(0., 0., 1.)
        },
        mirror
    );
    builder.add_disc(
        Disc {
            center: Vec3::new(0.6, 1., -0.245),
            radius: 0.2,
            normal: Vec3::new(0., 0., 1.)
        },
        mirror
    );
    builder.add_sphere(
        Sphere {
            center: Vec3::new(0.4, 0.95, 0.5),
            radius: 0.1
        },
        green
    );
    builder.add_sphere(
        Sphere {
            center: Vec3::new(0.7, 0.9, 0.7),
            radius: 0.05
        },
        red
    );

    let camera = Camera::look_at(
        Vec3::new(0.993, 1.249, 1.406),
        Vec3::new(-0.047, 0.709, -0.0516),
        Vec3::new(0., 1., 0.)
    ).with_fov(60_f32.to_radians());

    (camera, builder)
}
