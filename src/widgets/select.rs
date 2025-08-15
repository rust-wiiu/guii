use core::fmt::Display;

pub struct Response {
    pub changed: bool,
}

pub trait Bound: Display {}
impl<T: Display> Bound for T {}

pub struct Select<'a, T: Bound> {
    text: &'a str,
    index: &'a mut usize,
    options: &'a [T],
}

impl<'a, T: Bound> Select<'a, T> {
    pub fn new(text: &'a str, index: &'a mut usize, options: &'a [T]) -> Self {
        Self {
            text,
            index,
            options,
        }
    }
}

impl<T: Bound> super::Widget for Select<'_, T> {
    type Response = Response;

    fn draw(self, ui: &mut crate::Ui<'_, impl super::RenderTarget>) -> Self::Response {
        todo!()
    }
}
