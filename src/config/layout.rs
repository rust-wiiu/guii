use wut::gx2::types::Vec2;

#[derive(Debug)]
pub struct Layout {
    pub origin: Vec2<f32>,
    pub current: Vec2<f32>,
    pub gap: Vec2<f32>,
}

impl Layout {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            origin: Vec2::new(x, y),
            current: Vec2::new(x, y),
            gap: Vec2::new(10.0, 10.0),
        }
    }
}

pub trait Scaling {
    fn absolute(&self, reference: usize) -> usize;
    fn relative(&self, reference: usize) -> f32;
}

impl Scaling for usize {
    fn absolute(&self, _: usize) -> usize {
        *self
    }

    fn relative(&self, reference: usize) -> f32 {
        *self as f32 / reference as f32
    }
}
impl Scaling for f32 {
    fn absolute(&self, reference: usize) -> usize {
        (*self * reference as f32) as usize
    }

    fn relative(&self, _: usize) -> f32 {
        *self
    }
}
