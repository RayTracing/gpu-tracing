// Originally written in 2023 by Arman Uguray <arman.uguray@gmail.com>
// SPDX-License-Identifier: CC-BY-4.0

use {
    anyhow::{Context, Result},
    winit::{
        event::{DeviceEvent, ElementState, Event, MouseScrollDelta, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::{Window, WindowBuilder},
    },
};

mod algebra;
mod camera;
mod gallery;
mod render;
mod scene;

const WIDTH: u32 = 1600;
const HEIGHT: u32 = 1200;

#[pollster::main]
async fn main() -> Result<()> {
    let event_loop = EventLoop::new()?;
    let window_size = winit::dpi::PhysicalSize::new(WIDTH, HEIGHT);
    let window = WindowBuilder::new()
        .with_inner_size(window_size)
        .with_resizable(false)
        .with_title("GPU Path Tracer".to_string())
        .build(&event_loop)?;

    let (device, queue, surface) = connect_to_gpu(&window).await?;
    let mut renderer = render::PathTracer::new(device, queue, WIDTH, HEIGHT);
    let (mut camera, builder) = gallery::another_scene_with_spheres();
    let scene_resources = builder.build(renderer.device(), renderer.scene_group_layout());
    // let mut gallery =
    //    gallery::Gallery::new(renderer.device(), renderer.scene_group_layout());

    let mut left_mouse_button_pressed = false;
    let mut right_mouse_button_pressed = false;

    event_loop.run(|event, control_handle| {
        control_handle.set_control_flow(ControlFlow::Poll);
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => control_handle.exit(),
                WindowEvent::KeyboardInput {
                    device_id: _,
                    event,
                    ..
                } => {
                    use winit::keyboard::{KeyCode, PhysicalKey};

                    if event.state != ElementState::Pressed {
                        return;
                    }
                    match event.physical_key {
                        PhysicalKey::Code(KeyCode::ArrowUp) => {
                            camera.adjust_fov(1_f32.to_radians());
                            // gallery.current_camera_mut().adjust_fov(1_f32.to_radians());
                            renderer.reset_samples();
                        }
                        PhysicalKey::Code(KeyCode::ArrowDown) => {
                            camera.adjust_fov(-1_f32.to_radians());
                            // gallery.current_camera_mut().adjust_fov(-1_f32.to_radians());
                            renderer.reset_samples();
                        }
                        _ => (),
                    }
                }
                WindowEvent::MouseInput {
                    device_id: _,
                    state,
                    button,
                } => {
                    use winit::event::MouseButton;

                    let pressed = state == ElementState::Pressed;
                    match button {
                        MouseButton::Left => left_mouse_button_pressed = pressed,
                        MouseButton::Right => right_mouse_button_pressed = pressed,
                        _ => (),
                    }
                }
                WindowEvent::RedrawRequested => {
                    // Wait for the next available frame buffer.
                    let frame: wgpu::SurfaceTexture = surface
                        .get_current_texture()
                        .expect("failed to get current texture");

                    let render_target = frame
                        .texture
                        .create_view(&wgpu::TextureViewDescriptor::default());

                    //println!("{camera:?}");
                    //let scene = gallery.current_scene();
                    //renderer.render_frame(&scene.camera, &scene.resources, &render_target);
                    renderer.render_frame(&camera, &scene_resources, &render_target);

                    frame.present();
                    window.request_redraw();
                }
                _ => (),
            },
            Event::DeviceEvent { event, .. } => match event {
                DeviceEvent::MouseWheel { delta } => {
                    let delta = match delta {
                        MouseScrollDelta::PixelDelta(delta) => 0.001 * delta.y as f32,
                        MouseScrollDelta::LineDelta(_, y) => y * 0.1,
                    };
                    camera.zoom(delta);
                    //gallery.current_camera_mut().zoom(delta);
                    renderer.reset_samples();
                }
                DeviceEvent::MouseMotion { delta: (dx, dy) } => {
                    let dx = dx as f32 * 0.01;
                    let dy = dy as f32 * -0.01;
                    if left_mouse_button_pressed {
                        camera.orbit(dx, dy);
                        //gallery.current_camera_mut().orbit(dx, dy);
                        renderer.reset_samples();
                    }
                    if right_mouse_button_pressed {
                        camera.pan(dx, dy);
                        //gallery.current_camera_mut().pan(dx, dy);
                        renderer.reset_samples();
                    }
                }
                _ => (),
            },
            _ => (),
        }
    })?;
    Ok(())
}

async fn connect_to_gpu(
    window: &Window,
) -> Result<(wgpu::Device, wgpu::Queue, wgpu::Surface)> {
    use wgpu::TextureFormat::{Bgra8Unorm, Rgba8Unorm};

    // Create an "instance" of wgpu. This is the entry-point to the API.
    let instance = wgpu::Instance::default();

    // Create a drawable "surface" that is associated with the window.
    let surface = instance.create_surface(window)?;

    // Request a GPU that is compatible with the surface. If the system has multiple GPUs then
    // pick the high performance one.
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        })
        .await
        .context("failed to find a compatible adapter")?;

    // Connect to the GPU. "device" represents the connection to the GPU and allows us to create
    // resources like buffers, textures, and pipelines. "queue" represents the command queue that
    // we use to submit commands to the GPU.
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default())
        .await
        .context("failed to connect to the GPU")?;

    // Configure the texture memory backing the surface. Our renderer will draw to a surface
    // texture every frame.
    let caps = surface.get_capabilities(&adapter);
    let format = caps
        .formats
        .into_iter()
        .find(|it| matches!(it, Rgba8Unorm | Bgra8Unorm))
        .context("could not find preferred texture format (Rgba8Unorm or Bgra8Unorm)")?;
    let size = window.inner_size();
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::AutoVsync,
        alpha_mode: caps.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 3,
    };
    surface.configure(&device, &config);

    Ok((device, queue, surface))
}
