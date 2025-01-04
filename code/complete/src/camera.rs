// Originally written in 2024 by Arman Uguray <arman.uguray@gmail.com>
// SPDX-License-Identifier: CC-BY-4.0

use {
    bytemuck::{Pod, Zeroable},
    std::f32::consts::{FRAC_PI_2, PI},
};

use crate::algebra::Vec3;

#[derive(Debug, Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct CameraUniforms {
    origin: Vec3,
    _pad0: u32,
    u: Vec3,
    _pad1: u32,
    v: Vec3,
    _pad2: u32,
    w: Vec3,
    _pad3: u32,
}

pub struct Camera {
    uniforms: CameraUniforms,
    center: Vec3,
    up: Vec3,
    distance: f32,
    azimuth: f32,
    altitude: f32,
}

impl Camera {
    pub fn look_at(origin: Vec3, center: Vec3, up: Vec3) -> Camera {
        let center_to_origin = origin - center;
        let distance = center_to_origin.length().max(0.01); // Prevent distance of 0
        let neg_w = center_to_origin.normalized();
        let azimuth = neg_w.x().atan2(neg_w.z());
        let altitude = neg_w.y().asin();
        Self::with_spherical_coords(center, up, distance, azimuth, altitude)
    }

    pub fn with_spherical_coords(
        center: Vec3,
        up: Vec3,
        distance: f32,
        azimuth: f32,
        altitude: f32,
    ) -> Camera {
        let mut camera = Camera {
            uniforms: CameraUniforms::zeroed(),
            center,
            up,
            distance,
            azimuth,
            altitude,
        };
        camera.calculate_uniforms();
        camera
    }

    pub fn uniforms(&self) -> &CameraUniforms {
        &self.uniforms
    }

    pub fn zoom(&mut self, displacement: f32) {
        self.distance = (self.distance - displacement).max(0.0);  // Prevent negative distance
        self.uniforms.origin = self.center - self.distance * self.uniforms.w;
    }

    pub fn pan(&mut self, du: f32, dv: f32) {
        let pan = du * self.uniforms.u + dv * self.uniforms.v;
        self.center += pan;
        self.uniforms.origin += pan;
    }

    pub fn orbit(&mut self, du: f32, dv: f32) {
        const MAX_ALT: f32 = FRAC_PI_2 - 1e-6;
        self.altitude = (self.altitude + dv).clamp(-MAX_ALT, MAX_ALT);
        self.azimuth += du;
        self.azimuth %= 2. * PI;
        self.calculate_uniforms();
    }

    fn calculate_uniforms(&mut self) {
        let w = {
            let (y, xz_scale) = self.altitude.sin_cos();
            let (x, z) = self.azimuth.sin_cos();
            -Vec3::new(x * xz_scale, y, z * xz_scale)
        };
        let origin = self.center - self.distance * w;
        let u = w.cross(&self.up).normalized();
        let v = u.cross(&w);
        self.uniforms.origin = origin;
        self.uniforms.u = u;
        self.uniforms.v = v;
        self.uniforms.w = w;
    }
}
