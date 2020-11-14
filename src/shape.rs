use super::conf::*;
use super::util::{Point, Vertex, GLS};
use glium::{index::IndicesSource, uniform, Display, Frame, Program, Surface, VertexBuffer};

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
            x,
            y,
        }
    }

    pub fn draw<'a>(
        &self,
        target: &mut Frame,
        indices: impl Into<IndicesSource<'a>>,
        program: &Program,
    ) {
        // logical coor -> opengl coor
        let ax = (GLS / WIDTH) * (self.x + self.w / 2.0) - GLS / 2.0;
        let ay = (GLS / HEIGHT) * (self.y + self.h / 2.0) - GLS / 2.0;
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
