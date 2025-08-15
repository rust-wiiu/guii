pub struct Response {
    pub changed: bool,
}

pub struct Checkbox<'a> {
    text: &'a str,
    value: &'a mut bool,
}

impl<'a> Checkbox<'a> {
    pub fn new(text: &'a str, value: &'a mut bool) -> Self {
        Self { text, value }
    }
}

impl super::Widget for Checkbox<'_> {
    type Response = Response;

    fn draw(self, ui: &mut crate::Ui<'_, impl super::RenderTarget>) -> Self::Response {
        todo!()
    }
}
