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
}
