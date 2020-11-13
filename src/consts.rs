use glium::*;

pub const GLS: f32 = 2.0;

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

pub fn make_vertex_buffer(display: &Display, v: Vec<Point>) -> VertexBuffer<Vertex> {
    let mut shape: Vec<Vertex> = Vec::new();
    for p in v.iter() {
        shape.push(Vertex::new(p[0], p[1]));
    }
    VertexBuffer::new(display, &shape).unwrap()
}
