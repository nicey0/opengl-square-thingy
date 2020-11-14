use super::conf::*;
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

pub struct Shape {
    vbuf: VertexBuffer<Vertex>,
    pub w: f32,
    pub h: f32,
    pub x: f32,
    pub y: f32,
}
impl Shape {
    pub fn new(display: &Display, v: &Vec<Point>, x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            vbuf: make_shape(display, v),
            w,
            h,
            x: x + (w / 2.0),
            y: y + (h / 2.0),
            //x: x + (-w + w / 2.0), // move from center to bottom left
            //y: y + (h / 2.0),
        }
    }

    pub fn draw<'a>(
        &self,
        target: &mut Frame,
        indices: impl Into<index::IndicesSource<'a>>,
        program: &Program,
    ) {
        // logical coor -> opengl coor
        let ax = (GLS / WIDTH) * self.x - GLS / 2.0;
        let ay = (GLS / HEIGHT) * self.y - GLS / 2.0;
        let aw = (GLS / WIDTH) * self.w;
        let ah = (GLS / HEIGHT) * self.h;

        target
            .draw(
                &self.vbuf,
                indices,
                program,
                &uniform! {
                    col: [1.0f32, 1.0f32, 0.0],
                    matrix: [
                       [aw,   0.0,  0.0,  0.0],
                       [0.0,  ah,   0.0,  0.0],
                       [0.0,  0.0,  0.0,  0.0],
                       [ax,  ay,  0.0,  1.0],
                    ],
                },
                &Default::default(),
            )
            .unwrap();
    }

    pub fn print(&self) {
        println!("x: {}, y: {}\nw: {}, h: {}", self.x, self.y, self.w, self.h);
    }
}

pub fn make_shape(display: &Display, v: &Vec<Point>) -> VertexBuffer<Vertex> {
    let mut shape: Vec<Vertex> = Vec::new();
    for p in v.iter() {
        shape.push(Vertex::new(p[0], p[1]));
    }
    VertexBuffer::new(display, &shape).unwrap()
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
