use crate::config::controls::Action;

use super::Widget;

pub struct Response {
    pub clicked: bool,
}

pub struct Button<'a> {
    text: &'a str,
}

impl<'a> Button<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text }
    }
}

impl Widget for Button<'_> {
    type Response = Response;

    fn draw(self, ui: &mut super::Ui<'_, impl super::RenderTarget>) -> Self::Response {
        const PADDING: f32 = 10.0;

        let mut response = Response { clicked: false };

        let size = ui.guii.atlus.layout(self.text, 32);

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
        //     }

        //     ui.config.pallet.highlight.base
        // } else {
        //     ui.config.pallet.widget.base
        // };

        let color = if ui.guii.focus.focused(ui.index) {
            if let Some(input) = ui.input.take() {
                match ui.config.controls.check(&input) {
                    Action::Up => ui.guii.focus.prev(),
                    Action::Down => ui.guii.focus.next(),
                    Action::Accept => response.clicked = true,
                    _ => (),
                }
            }

            ui.config.pallet.highlight
        } else {
            ui.config.pallet.widget
        };

        let pos = ui.position;

        let size = ui.rect(
            pos.x,
            pos.y,
            size.x + PADDING * 2.0,
            size.y + PADDING,
            color.base,
        );

        ui.text(
            self.text,
            pos.x + PADDING,
            pos.y + PADDING,
            32,
            color.content,
        );

        ui.position.y -= size.y + ui.config.layout.gap.y;
        ui.index += 1;

        response
    }
}
