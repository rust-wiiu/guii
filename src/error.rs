use thiserror::Error;
use wut::{
    font::FontError,
    gx2::{buffer::BufferError, shader::ShaderError},
};

#[derive(Debug, Error)]
pub enum GuiiError {
    #[error("Allocation of GX2 buffer failed")]
    BufferError(#[from] BufferError),
    #[error("Creating the ")]
    ShaderError(#[from] ShaderError),
    #[error("Fontdue")]
    FontdueError(&'static str),
    #[error("System font")]
    FontError(#[from] FontError),
}
