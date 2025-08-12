use crate::{font::Atlus, guii::Guii};
use wut::gx2::{
    color::Color,
    target::RenderTarget,
    types::{Extend, Mat3x2, Vec2, Vec3},
};

pub struct Context<'a, T: RenderTarget> {
    guii: &'a mut Guii<T>,
    width: usize,
    height: usize,
    z: f32,
}

impl<'a, T: RenderTarget> Context<'a, T> {
    const Z_INCREASE: f32 = 0.0001;

    pub(crate) fn new(guii: &'a mut Guii<T>, width: usize, height: usize) -> Self {
        Self {
            guii,
            width,
            height,
            z: 0.0,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn vertex(&mut self, vertex: Vec3<f32>, tex: Vec2<f32>, color: Color) {
        let _ = self.guii.vertices.push(vertex).unwrap();
        let _ = self.guii.tex.push(tex);
        let _ = self.guii.colors.push(color).unwrap();
    }

    pub fn triangle(&mut self, vert: Mat3x2<f32>, tex: Option<Mat3x2<f32>>, z: f32, color: Color) {
        let tex = tex.unwrap_or(Mat3x2::from(-1.0));

        self.vertex(vert.a.extend(z), tex.a, color);
        self.vertex(vert.b.extend(z), tex.b, color);
        self.vertex(vert.c.extend(z), tex.c, color);
    }

    pub fn rect(
        &mut self,
        x: impl Coordinate,
        y: impl Coordinate,
        w: impl Coordinate,
        h: impl Coordinate,
        color: Color,
    ) {
        let x = x.absolute(self.width) as f32;
        let y = y.absolute(self.height) as f32;
        let w = w.absolute(self.width) as f32;
        let h = h.absolute(self.height) as f32;
        // let z = z as f32;

        // println!("{} {} {} {} {}", x, y, w, h, z);

        self.triangle(
            Mat3x2::new(
                Vec2::new(x, y),
                Vec2::new(x + w, y),
                Vec2::new(x + w, y + h),
            ),
            None,
            self.z,
            color,
        );
        self.triangle(
            Mat3x2::new(
                Vec2::new(x, y),
                Vec2::new(x + w, y + h),
                Vec2::new(x, y + h),
            ),
            None,
            self.z,
            color,
        );

        self.z += Self::Z_INCREASE;
    }

    pub fn text(
        &mut self,
        text: impl AsRef<str>,
        x: impl Coordinate,
        y: impl Coordinate,
        scale: impl Scaling,
        color: Color,
    ) {
        let mut x = x.absolute(self.width);
        let mut y = y.absolute(self.height);
        let scale = scale.relative(Atlus::PX);

        let origin = (x, y);

        for c in text.as_ref().chars() {
            if c == '\n' {
                x = origin.0;
                y -= (Atlus::PX as f32 * scale) as usize;
                continue;
            }

            let (tex, metrics) = self.guii.atlus.get(c);

            let tex = *tex;
            let metrics = *metrics;

            let w = metrics.width as f32 * scale;
            let h = metrics.height as f32 * scale;
            let a = (metrics.advance_width * scale) as usize;

            let b = y as f32 + (metrics.ymin as f32 * scale);
            let l = x as f32 + (metrics.xmin as f32 * scale);
            let t = b + h;
            let r = l + w;

            self.triangle(
                Mat3x2::new(Vec2::new(l, b), Vec2::new(r, b), Vec2::new(r, t)),
                Some(Mat3x2::new(tex.lb(), tex.rb(), tex.rt())),
                self.z,
                color,
            );

            self.triangle(
                Mat3x2::new(Vec2::new(l, b), Vec2::new(r, t), Vec2::new(l, t)),
                Some(Mat3x2::new(tex.lb(), tex.rt(), tex.lt())),
                self.z,
                color,
            );

            x += a;

            // line_height = line_height.max(metrics.advance_height as f32);
        }

        self.z += Self::Z_INCREASE;
    }
}

pub trait Coordinate {
    fn absolute(&self, reference: usize) -> usize;
    fn relative(&self, reference: usize) -> f32;
}

impl Coordinate for usize {
    fn absolute(&self, _: usize) -> usize {
        *self
    }

    fn relative(&self, reference: usize) -> f32 {
        *self as f32 / reference as f32
    }
}
impl Coordinate for f32 {
    fn absolute(&self, reference: usize) -> usize {
        (*self * reference as f32) as usize
    }

    fn relative(&self, reference: usize) -> f32 {
        *self / reference as f32
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
