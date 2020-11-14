extern crate glium;

use glium::glutin::event::*;
use glium::{
    draw_parameters::PolygonMode, glutin, index::NoIndices, index::PrimitiveType, Display,
    DrawParameters, Frame, Program, Surface,
};

use std::fs;
use std::time::{Duration, Instant};

mod conf;
mod shape;
mod util;
use conf::*;
use shape::*;
use util::*;

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Glium Testing")
        .with_decorations(true)
        .with_inner_size(glutin::dpi::LogicalSize::new(WIDTH, HEIGHT))
        .with_resizable(false);
    let cb = glutin::ContextBuilder::new();
    let display = Display::new(wb, cb, &event_loop).unwrap();

    // input
    let mut inputs = Input::new();

    // shape
    let mut square = Shape::new(
        &display,
        &vec![[-0.5, 0.5], [0.5, 0.5], [-0.5, -0.5], [0.5, -0.5]],
        0.0,
        0.0,
        25.0,
        25.0,
    );

    let mut bullets: Vec<Shape> = Vec::new();

    // shaders
    let vs = fs::read_to_string("src/shaders/vs.glsl").expect("Unable to read file");
    let fs = fs::read_to_string("src/shaders/fs.glsl").expect("Unable to read file");
    // rendering stuf
    let indices = NoIndices(PrimitiveType::TriangleStrip);
    let program = Program::from_source(&display, &vs, &fs, None).unwrap();
    let params = DrawParameters {
        //polygon_mode: PolygonMode::Line,
        //line_width: Some(1.0),
        ..Default::default()
    };

    // event loop
    let mut grounded = false;
    let mut yspd = 0.0;
    event_loop.run(move |ev, _, control_flow| {
        let next_frame_time = Instant::now() + Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        //
        // update
        //
        square.y += yspd;
        if inputs.w {
            if grounded {
                yspd = SPEED * 5.0;
                grounded = false;
            }
        }
        if !grounded {
            yspd -= 0.01;
        }

        if inputs.a {
            square.x -= SPEED;
        }
        if inputs.d {
            square.x += SPEED;
        }
        for b in bullets.iter_mut() {
            b.x += 1.0;
        }
        bullets.retain(|b| b.x < WIDTH);
        // collisions
        if square.x < -square.w * 2.0 {
            square.x = WIDTH;
        } else if square.x >= WIDTH {
            square.x = -square.w * 2.0;
        }
        if square.y < 0.0 {
            grounded = true;
            square.y = 0.0;
        } else if square.y >= HEIGHT {
            square.y = -square.h * 2.0
        }

        //
        // render
        //
        let mut target: Frame = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        square.draw(&mut target, &indices, &program, &params, [1.0, 1.0, 0.0]);
        for b in bullets.iter() {
            b.draw(&mut target, &indices, &program, &params, [1.0, 1.0, 1.0]);
        }
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
                            VirtualKeyCode::Space => bullets.push(Shape::new(
                                &display,
                                &vec![[-0.5, 0.5], [0.5, 0.5], [-0.5, -0.5], [0.5, -0.5]],
                                square.x + square.w,
                                square.y + square.h / 2.0 - 2.5,
                                5.0,
                                5.0,
                            )),
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
