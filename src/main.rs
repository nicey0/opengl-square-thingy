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

    // input
    let mut inputs = Input::new();

    // shape
    let mut square = Shape::new(
        &display,
        &vec![[-0.5, 0.5], [0.5, 0.5], [-0.5, -0.5], [0.5, -0.5]],
        0.0,
        0.0,
        20.0,
        20.0,
    );

    // shaders
    let vs = fs::read_to_string("src/shaders/vs.glsl").expect("Unable to read file");
    let fs = fs::read_to_string("src/shaders/fs.glsl").expect("Unable to read file");
    // rendering stuf
    let indices = index::NoIndices(index::PrimitiveType::TriangleStrip);
    let program = Program::from_source(&display, &vs, &fs, None).unwrap();

    // event loop
    event_loop.run(move |ev, _, control_flow| {
        let next_frame_time = Instant::now() + Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        //
        // update
        //
        if inputs.w {
            square.y += SPEED;
        }
        if inputs.a {
            square.x -= SPEED;
        }
        if inputs.s {
            square.y -= SPEED;
        }
        if inputs.d {
            square.x += SPEED;
        }

        if square.x < -square.w {
            square.x = WIDTH;
        } else if square.x >= WIDTH {
            square.x = -square.w
        }
        if square.y < -square.h {
            square.y = HEIGHT;
        } else if square.y >= HEIGHT {
            square.y = -square.h
        }

        //
        // render
        //
        let mut target: Frame = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        square.draw(&mut target, &indices, &program);
        target.finish().unwrap();

        // events
        match ev {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    match (input.state, input.virtual_keycode) {
                        (s, Some(k)) => match k {
                            VirtualKeyCode::W => inputs.w = s == ElementState::Pressed,
                            VirtualKeyCode::A => inputs.a = s == ElementState::Pressed,
                            VirtualKeyCode::S => inputs.s = s == ElementState::Pressed,
                            VirtualKeyCode::D => inputs.d = s == ElementState::Pressed,
                            _ => {}
                        },
                        _ => {}
                    }
                }
                _ => return,
            },
            _ => {}
        }
    });
}
