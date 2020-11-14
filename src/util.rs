use glium::implement_vertex;
use std::f32::consts::PI;

pub const GLS: f32 = 2.0;
pub const RADTODEG: f32 = 2.0 * (PI / 360.0);

pub type Point = [f32; 2];

#[derive(Clone, Copy)]
pub struct Vertex {
    pos: Point,
}
implement_vertex!(Vertex, pos);
impl Vertex {
    pub fn new(x: f32, y: f32) -> Self {
        Self { pos: [x, y] }
    }
}

pub struct Input {
    pub w: bool,
    pub a: bool,
    pub s: bool,
    pub d: bool,
}
impl Input {
    pub fn new() -> Self {
        Self {
            w: false,
            a: false,
            s: false,
            d: false,
        }
    }
}
