use core::fmt::Display;

pub struct Response {
    pub clicked: bool,
    pub changed: bool,
}

pub trait Bound: Display {}
impl<T: Display> Bound for T {}

pub struct Grid<'a, T: Bound> {
    text: &'a str,
    columns: usize,
    index: &'a mut usize,
    data: &'a [T],
}

impl<'a, T: Bound> Grid<'a, T> {
    pub fn new(text: &'a str, columns: usize, index: &'a mut usize, data: &'a [T]) -> Self {
        Self {
            text,
            columns,
            index,
            data,
        }
    }
}

impl<T: Bound> super::Widget for Grid<'_, T> {
    type Response = Response;

    fn draw(self, ui: &mut crate::Ui<'_, impl super::RenderTarget>) -> Self::Response {
        todo!()
    }
}
