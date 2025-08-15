use core::{
    fmt::Display,
    ops::{AddAssign, RangeInclusive, SubAssign},
};
use wut::{format, gx2::types::Vec2};

use crate::config::controls::Action;

#[derive(Debug, Default)]
pub struct Response {
    pub clicked: bool,
    pub changed: bool,
}

pub trait Bound: Sized + Display + AddAssign + SubAssign + PartialOrd {}
impl<T: Display + AddAssign + SubAssign + PartialOrd> Bound for T {}

pub struct Number<'a, T: Bound> {
    text: &'a str,
    value: &'a mut T,
    range: RangeInclusive<T>,
    delta: T,
}

impl<'a, T: Bound> Number<'a, T> {
    pub fn new(text: &'a str, value: &'a mut T, range: RangeInclusive<T>, delta: T) -> Self {
        Self {
            text,
            value,
            range,
            delta,
        }
    }
}

impl<T: Bound> super::Widget for Number<'_, T> {
    type Response = Response;

    fn draw(self, ui: &mut super::Ui<'_, impl super::RenderTarget>) -> Self::Response {
        const PADDING: f32 = 10.0;
        const SCALE: usize = 32;

        let mut response = Response::default();

        // let color = if ui.index == ui.guii.focus {
        //     if ui.input.trigger.contains(ui.config.controls.up) {
        //         ui.guii.focus = ui.guii.focus.saturating_sub(1);
        //         ui.input.trigger ^= ui.config.controls.up;
        //     } else if ui.input.trigger.contains(ui.config.controls.down) {
        //         ui.guii.focus = ui.guii.focus.saturating_add(1);
        //         ui.input.trigger ^= ui.config.controls.down;
        //     } else if ui.input.trigger.contains(ui.config.controls.accept) {
        //         response.clicked = true;
        //         ui.input.trigger ^= ui.config.controls.accept;
        //     } else if ui.input.trigger.contains(ui.config.controls.left)
        //         && *self.value > *self.range.start()
        //     {
        //         *self.value -= self.delta;
        //         response.changed = true;
        //         ui.input.trigger ^= ui.config.controls.left;
        //     } else if ui.input.trigger.contains(ui.config.controls.right)
        //         && *self.value < *self.range.end()
        //     {
        //         *self.value += self.delta;
        //         response.changed = true;
        //         ui.input.trigger ^= ui.config.controls.right;
        //     }
        // }

        let color = if ui.guii.focus.focused(ui.index) {
            if let Some(input) = ui.input.take() {
                match ui.config.controls.check(&input) {
                    Action::Up => ui.guii.focus.prev(),
                    Action::Down => ui.guii.focus.next(),
                    Action::Left => {
                        if *self.value > *self.range.start() {
                            *self.value -= self.delta;
                            response.changed = true;
                        }
                    }
                    Action::Right => {
                        if *self.value < *self.range.end() {
                            *self.value += self.delta;
                            response.changed = true;
                        }
                    }
                    Action::Accept => {
                        response.clicked = true;
                    }
                    _ => (),
                }
            }

            ui.config.pallet.highlight
        } else {
            ui.config.pallet.widget
        };

        let offset = ui.text(
            self.text,
            ui.position.x,
            ui.position.y,
            SCALE,
            ui.config.pallet.widget.content,
        ) + Vec2::new(PADDING, 0.0);

        let text = format!("{:05.2}", self.value);

        let size = ui.guii.atlus.layout(&text, 32);

        let value_pad = PADDING + SCALE as f32 + PADDING;

        let size = ui.rect(
            offset.x + ui.position.x,
            ui.position.y,
            value_pad + size.x + value_pad + PADDING,
            PADDING + size.y,
            color.base,
        );

        ui.text(
            &format!(
                "{}  {}  {}",
                wut::font::icons::gamepad::LEFT,
                &text,
                wut::font::icons::gamepad::RIGHT
            ),
            offset.x + ui.position.x + PADDING,
            ui.position.y + PADDING,
            32,
            color.content,
        );

        ui.position.y -= size.y + ui.config.layout.gap.y;
        ui.index += 1;

        response
    }
}
