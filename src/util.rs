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
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}
impl Shape {
    pub fn new(display: &Display, v: &Vec<Point>, x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            vbuf: make_shape(display, v),
            x,
            y,
            w,
            h,
        }
    }

    pub fn draw<'a>(
        &self,
        target: &mut Frame,
        indices: impl Into<index::IndicesSource<'a>>,
        program: &Program,
    ) {
        target
            .draw(
                &self.vbuf,
                indices,
                program,
                uniform! {},
                &Default::default(),
            )
            .unwrap();
    }
}

pub fn make_shape(display: &Display, v: &Vec<Point>) -> VertexBuffer<Vertex> {
    let mut shape: Vec<Vertex> = Vec::new();
    for p in v.iter() {
        shape.push(Vertex::new(p[0], p[1]));
    }
    VertexBuffer::new(display, &shape).unwrap()
}
