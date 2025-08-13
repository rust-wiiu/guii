use crate::{
    font::Atlus,
    guii::Guii,
    layout::{Coordinate, Layout, Scaling},
};
use core::fmt::Display;
use wut::{
    format,
    gx2::{
        color::Color,
        target::RenderTarget,
        types::{Extend, Mat3x2, Vec2, Vec3},
    },
    string::String,
};

pub struct Context<'l, Target: RenderTarget> {
    guii: &'l mut Guii<Target>,
    width: usize,
    height: usize,
    z: f32,
    layout: Layout,
}

impl<'l, Target: RenderTarget> Context<'l, Target> {
    const Z_INCREASE: f32 = 0.0001;

    pub(crate) fn new(guii: &'l mut Guii<Target>, width: usize, height: usize) -> Self {
        Self {
            guii,
            width,
            height,
            z: 0.0,
            layout: Layout::new(100, 100),
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
    ) -> Vec2<usize> {
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

        Vec2::new(w as usize, h as usize)
    }

    pub fn text(
        &mut self,
        text: &str,
        x: impl Coordinate,
        y: impl Coordinate,
        scale: impl Scaling,
        color: Color,
    ) -> Vec2<usize> {
        let mut x = x.absolute(self.width);
        let mut y = y.absolute(self.height);
        let scale = scale.relative(Atlus::PX);

        let origin = (x, y);

        for c in text.chars() {
            if c == '\n' {
                x = origin.0;
                y += (Atlus::PX as f32 * scale) as usize;
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

        Vec2::new(x - origin.0, (scale * Atlus::PX as f32) as usize)
    }

    pub fn border(
        &mut self,
        x: impl Coordinate,
        y: impl Coordinate,
        w: impl Coordinate,
        h: impl Coordinate,
        size: usize,
        color: Color,
    ) -> Vec2<usize> {
        let x = x.absolute(self.width);
        let y = y.absolute(self.height);
        let w = w.absolute(self.width);
        let h = h.absolute(self.height);

        self.rect(x, y, w, size, color);
        self.rect(x, y, size, h, color);
        self.rect(x + w - size, y, size, h, color);
        self.rect(x, y + h - size, w, size, color);

        Vec2::new(w, h)
    }

    pub fn label(&mut self, text: &str) {
        let size = self.text(
            text,
            self.layout.current.x,
            self.layout.current.y,
            32,
            Color::black(),
        );

        self.layout.current.y += size.y + self.layout.gap.y;
    }

    pub fn button(&mut self, text: &str) -> button::Response {
        const PADDING: usize = 10;

        let response = button::Response { clicked: false };

        let size = self.guii.atlus.layout(text, 32);

        let size = self.rect(
            self.layout.current.x,
            self.layout.current.y,
            size.x + PADDING * 2,
            size.y + PADDING,
            Color::black(),
        );

        self.text(
            text,
            self.layout.current.x + PADDING,
            self.layout.current.y + PADDING,
            32,
            Color::white(),
        );

        self.layout.current.y += size.y + self.layout.gap.y;

        response
    }

    pub fn number<'a, T: Display>(
        &mut self,
        text: &str,
        value: &'a mut T,
        delta: T,
    ) -> number::Response {
        const PADDING: usize = 10;
        const SCALE: usize = 32;
        const TEXT_COLOR: Color = Color::white();
        const RECT_COLOR: Color = Color::black();

        let response = number::Response { changed: false };

        let offset = self.text(
            text,
            self.layout.current.x,
            self.layout.current.y + PADDING,
            SCALE,
            RECT_COLOR,
        ) + Vec2::new(PADDING, 0);

        let text = format!("{:05}", value);

        let size = self.guii.atlus.layout(&text, 32);

        let value_pad = PADDING + SCALE + PADDING;

        let size = self.rect(
            offset.x + self.layout.current.x,
            self.layout.current.y,
            value_pad + size.x + value_pad,
            PADDING + size.y,
            RECT_COLOR,
        );

        self.text(
            &format!("{}", wut::font::icons::gamepad::LEFT),
            offset.x + self.layout.current.x + PADDING,
            self.layout.current.y + PADDING,
            SCALE,
            TEXT_COLOR,
        );

        self.text(
            &text,
            offset.x + value_pad + self.layout.current.x,
            self.layout.current.y + PADDING,
            32,
            TEXT_COLOR,
        );

        self.text(
            &format!("{}", wut::font::icons::gamepad::RIGHT),
            offset.x + self.layout.current.x + size.x - value_pad + PADDING,
            self.layout.current.y + PADDING,
            SCALE,
            TEXT_COLOR,
        );

        self.layout.current.y += size.y + self.layout.gap.y;

        response
    }

    pub fn checkbox(&mut self, text: &str, value: &mut bool) -> checkbox::Response {
        const PADDING: usize = 10;
        const SCALE: usize = 32;
        const WHITE: Color = Color::white();
        const BLACK: Color = Color::black();

        let response = checkbox::Response { changed: false };

        let size = self.text(
            text,
            self.layout.current.x,
            self.layout.current.y + PADDING,
            SCALE,
            BLACK,
        );

        let offset = size + Vec2::new(SCALE, 0);

        self.rect(
            self.layout.current.x + offset.x,
            self.layout.current.y,
            SCALE + 12,
            SCALE + 12,
            WHITE,
        );

        self.rect(
            self.layout.current.x + offset.x + 4,
            self.layout.current.y + 4,
            SCALE + 4,
            SCALE + 4,
            BLACK,
        );

        self.text(
            if *value { "x" } else { " " },
            self.layout.current.x + offset.x + 14,
            self.layout.current.y + 14,
            SCALE,
            WHITE,
        );

        self.layout.current.y += size.y + self.layout.gap.y;

        response
    }

    pub fn select<T: Display>(
        &mut self,
        text: &str,
        index: &mut usize,
        options: &[T],
    ) -> select::Response {
        const PADDING: usize = 10;
        const SCALE: usize = 32;
        const WHITE: Color = Color::white();
        const BLACK: Color = Color::black();

        let response = select::Response { changed: false };

        let offset = self.text(
            text,
            self.layout.current.x,
            self.layout.current.y + PADDING,
            SCALE,
            BLACK,
        ) + Vec2::new(PADDING, 0);

        let value = format!("{}", options[*index]);

        let size = self.guii.atlus.layout(&value, 32);

        let value_pad = PADDING + SCALE + PADDING;

        let size = self.rect(
            offset.x + self.layout.current.x,
            self.layout.current.y,
            value_pad + size.x + value_pad,
            PADDING + size.y,
            BLACK,
        );

        self.text(
            &format!("{}", wut::font::icons::gamepad::LEFT),
            offset.x + self.layout.current.x + PADDING,
            self.layout.current.y + PADDING,
            SCALE,
            WHITE,
        );

        self.text(
            &value,
            offset.x + value_pad + self.layout.current.x,
            self.layout.current.y + PADDING,
            32,
            WHITE,
        );

        self.text(
            &format!("{}", wut::font::icons::gamepad::RIGHT),
            offset.x + self.layout.current.x + size.x - value_pad + PADDING,
            self.layout.current.y + PADDING,
            SCALE,
            WHITE,
        );

        self.layout.current.y += size.y + self.layout.gap.y;

        response
    }

    pub fn grid<V: Display>(
        &mut self,
        text: &str,
        index: &mut usize,
        columns: usize,
        data: &[Target],
        fmt: &str,
    ) -> () {
        todo!()
    }
}

pub mod button {
    pub struct Response {
        pub clicked: bool,
    }
}

pub mod number {
    pub struct Response {
        pub changed: bool,
    }
}

pub mod checkbox {
    pub struct Response {
        pub changed: bool,
    }
}

pub mod select {
    pub struct Response {
        pub changed: bool,
    }
}
