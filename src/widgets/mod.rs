pub(crate) use super::Ui;
pub(crate) use wut::gx2::target::RenderTarget;

pub mod button;
pub mod checkbox;
pub mod grid;
pub mod label;
pub mod number;
pub mod select;

pub trait Widget {
    type Response;
    fn draw(self, ui: &mut super::Ui<'_, impl RenderTarget>) -> Self::Response;
}
