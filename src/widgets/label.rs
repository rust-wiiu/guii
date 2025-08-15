pub struct Label<'a> {
    text: &'a str,
}

impl<'a> Label<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text }
    }
}

impl super::Widget for Label<'_> {
    type Response = ();

    fn draw(self, ui: &mut crate::Ui<'_, impl super::RenderTarget>) -> Self::Response {
        let size = ui.text(
            self.text,
            ui.position.x,
            ui.position.y,
            32,
            ui.config.pallet.widget.content,
        );

        ui.position.y -= size.y + ui.config.layout.gap.y;
    }
}
