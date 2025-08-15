use crate::{
    config::{Config, layout::Scaling},
    font::Atlus,
    guii::Guii,
    widgets,
};
use wut::gx2::{
    color::Color,
    target::RenderTarget,
    types::{Extend, Mat3x2, Vec2, Vec3},
};

pub struct Ui<'l, Target: RenderTarget> {
    pub(crate) guii: &'l mut Guii<Target>,
    z: f32,
    pub(crate) index: usize,
    pub(crate) position: Vec2<f32>,
    pub(crate) input: Option<wut::gamepad::State>,
    pub(crate) config: Config,
}

impl<'l, Target: RenderTarget> Ui<'l, Target> {
    const Z_INCREASE: f32 = 0.0001;

    pub(crate) fn new(guii: &'l mut Guii<Target>, style: Config) -> Self {
        let input = guii.gamepad.poll().unwrap();

        Self {
            guii,
            z: 0.0,
            index: 0,
            position: Vec2::new(80.0, 1000.0),
            input: Some(input),
            config: style,
        }
    }

    pub fn size(&self) -> (usize, usize) {
        Target::size()
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

    pub fn rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: Color) -> Vec2<f32> {
        // let x = x.absolute(self.width) as f32;
        // let y = y.absolute(self.height) as f32;
        // let w = w.absolute(self.width) as f32;
        // let h = h.absolute(self.height) as f32;
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

        Vec2::new(w, h)
    }

    pub fn text(
        &mut self,
        text: &str,
        mut x: f32,
        mut y: f32,
        scale: impl Scaling,
        color: Color,
    ) -> Vec2<f32> {
        let scale = scale.relative(Atlus::PX);
        // let mut x = x;
        // let mut y = y;

        let origin = (x, y);

        for c in text.chars() {
            if c == '\n' {
                x = origin.0;
                y += (Atlus::PX as f32 * scale);
                continue;
            }

            let (tex, metrics) = self.guii.atlus.get(c);

            let tex = *tex;
            let metrics = *metrics;

            let w = metrics.width as f32 * scale;
            let h = metrics.height as f32 * scale;
            let a = (metrics.advance_width * scale);

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

        Vec2::new(x - origin.0, (scale * Atlus::PX as f32))
    }

    pub fn border(&mut self, x: f32, y: f32, w: f32, h: f32, size: f32, color: Color) -> Vec2<f32> {
        self.rect(x, y, w, size, color);
        self.rect(x, y, size, h, color);
        self.rect(x + w - size, y, size, h, color);
        self.rect(x, y + h - size, w, size, color);

        Vec2::new(w, h)
    }

    pub fn add<W: widgets::Widget>(&mut self, widget: W) -> W::Response {
        widget.draw(self)
    }

    pub fn label(&mut self, text: &str) {
        // let size = self.text(
        //     text,
        //     self.layout.current.x,
        //     self.layout.current.y,
        //     32,
        //     Color::black(),
        // );

        // self.layout.current.y += size.y + self.layout.gap.y;

        self.add(widgets::label::Label::new(text))
    }

    pub fn button(&mut self, text: &str) -> widgets::button::Response {
        self.add(widgets::button::Button::new(text))

        // const PADDING: usize = 10;

        // let response = button::Response { clicked: false };

        // let size = self.guii.atlus.layout(text, 32);

        // let color = if self.index == self.guii.focus {
        //     if self.input.trigger.contains(wut::gamepad::Button::Up) {
        //         self.guii.focus += 1;
        //         self.input.trigger ^= wut::gamepad::Button::Up;
        //     }
        //     if self.input.trigger.contains(wut::gamepad::Button::Down) {
        //         self.guii.focus -= 1;
        //         self.input.trigger ^= wut::gamepad::Button::Down;
        //     }

        //     Color::red()
        // } else {
        //     Color::black()
        // };

        // let pos = self.layout.pos();

        // let size = self.rect(pos.x, pos.y, size.x + PADDING * 2, size.y + PADDING, color);

        // self.text(text, pos.x + PADDING, pos.y + PADDING, 32, Color::white());

        // self.layout.advance(size);
        // self.index += 1;

        // response
    }

    pub fn number<'a, T: widgets::number::Bound>(
        &mut self,
        text: &str,
        value: &'a mut T,
        range: core::ops::RangeInclusive<T>,
        delta: T,
    ) -> widgets::number::Response {
        self.add(widgets::number::Number::new(text, value, range, delta))
    }

    pub fn checkbox(&mut self, text: &str, value: &mut bool) -> widgets::checkbox::Response {
        /*
        const PADDING: usize = 10;
        const SCALE: usize = 32;
        const WHITE: Color = Color::white();
        const BLACK: Color = Color::black();

        let response = checkbox::Response { changed: false };

        let color: Color = if self.index == self.guii.focus {
            Color::red()
        } else {
            Color::black()
        };

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
            color,
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
        self.index += 1;

        response

        */

        self.add(widgets::checkbox::Checkbox::new(text, value))
    }

    pub fn select<T: widgets::select::Bound>(
        &mut self,
        text: &str,
        index: &mut usize,
        options: &[T],
    ) -> widgets::select::Response {
        /*
        const PADDING: usize = 10;
        const SCALE: usize = 32;
        const WHITE: Color = Color::white();
        const BLACK: Color = Color::black();

        let response = select::Response { changed: false };

        let color: Color = if self.index == self.guii.focus {
            Color::red()
        } else {
            Color::black()
        };

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
            color,
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
        self.index += 1;

        response
        */

        self.add(widgets::select::Select::new(text, index, options))
    }

    pub fn grid<T: widgets::grid::Bound>(
        &mut self,
        text: &str,
        columns: usize,
        index: &mut usize,
        data: &[T],
    ) -> widgets::grid::Response {
        self.add(widgets::grid::Grid::new(text, columns, index, data))
    }
}
