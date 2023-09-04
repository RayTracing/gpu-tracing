// Originally written in 2023 by Arman Uguray <arman.uguray@gmail.com>
//
// To the extent possible under law, the author(s) have dedicated all copyright and related and
// neighboring rights to this software to the public domain worldwide. This software is
// distributed without any warranty.
//
// You should have received a copy (see file COPYING.txt) of the CC0 Public Domain Dedication
// along with this software. If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.

use {
    anyhow::{Context, Result},
    winit::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    },
};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

#[pollster::main]
async fn main() -> Result<()> {
    let event_loop = EventLoop::new();
    let window_size = winit::dpi::LogicalSize::new(WIDTH, HEIGHT);
    let window = WindowBuilder::new()
        .with_inner_size(window_size)
        .with_title("GPU Path Tracer".to_string())
        .build(&event_loop)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::RedrawRequested(_) => {
                // TODO: draw frame
            }
            Event::MainEventsCleared => {
                // draw repeatedly
                window.request_redraw();
            }
            _ => (),
        }
    });
}
