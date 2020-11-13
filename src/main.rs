extern crate glium;

use glium::glutin::event::*;
use glium::*;

use std::fs;
use std::time::{Duration, Instant};

mod conf;
mod util;
use conf::*;
use util::*;

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Glium Testing")
        .with_decorations(true)
        .with_inner_size(glutin::dpi::LogicalSize::new(WIDTH, HEIGHT))
        .with_resizable(false);
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    // shaders
    let vs = fs::read_to_string("src/shaders/vs.glsl").expect("Unable to read file");
    let fs = fs::read_to_string("src/shaders/fs.glsl").expect("Unable to read file");

    // shape
    let square = Shape::new(
        &display,
        &vec![[-0.5, 0.5], [0.5, 0.5], [-0.5, -0.5], [0.5, -0.5]],
        0.0,
        0.0,
        200.0,
        200.0,
    );

    // rendering stuf
    let indices = index::NoIndices(index::PrimitiveType::TriangleStrip);
    let program = Program::from_source(&display, &vs, &fs, None).unwrap();

    square.print();

    event_loop.run(move |ev, _, control_flow| {
        //
        // update
        //

        //
        // render
        //
        let mut target: Frame = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        square.draw(&mut target, &indices, &program);
        target.finish().unwrap();

        // event loop stuff
        let next_frame_time = Instant::now() + Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            _ => {}
        }
    });
}
