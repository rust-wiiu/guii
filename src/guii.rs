use crate::{context::Context, error::GuiiError, font::Atlus, vector::Vector};
use core::marker::PhantomData;
use wut::gx2::{
    buffer::Flags,
    shader::{
        self, Attribute, ShaderType,
        sampler::{CompareFn, Sampler, TexClamp, TexXyFilter},
    },
    target::{RenderTarget, Renderable},
    types::{Mat4x4, Vec2, Vec3, Vec4},
};

static PROGRAM: shader::Program = shader::Program::from(include_bytes!("shader.gsh"));

pub struct Guii<T: RenderTarget> {
    pub(crate) vertices: Vector<Vec3<f32>>,
    pub(crate) tex: Vector<Vec2<f32>>,
    pub(crate) colors: Vector<Vec4<f32>>,
    pub(crate) sampler: Sampler,
    pub(crate) atlus: Atlus,
    // indices: Vector<u32>,
    projection: Mat4x4<f32>,
    shader: shader::Shader,
    target: PhantomData<T>,
}

impl<T: RenderTarget> Guii<T> {
    pub fn new_indexed(shader_index: u32) -> Result<Self, GuiiError> {
        // wut::sys::GX2SetBlendControl(target, colorSrcBlend, colorDstBlend, colorCombine, useAlphaBlend, alphaSrcBlend, alphaDstBlend, alphaCombine);

        Ok(Self {
            vertices: Vector::default(Flags::VertexBuffer)?,
            tex: Vector::default(Flags::VertexBuffer)?,
            colors: Vector::default(Flags::VertexBuffer)?,
            sampler: Sampler::new(TexClamp::Clamp, TexXyFilter::Linear),
            atlus: Atlus::new()?,
            // indices: Vector::new(Flags::BindIndexBuffer)?,
            projection: T::ortho(),
            shader: shader::Shader::new(
                shader_index,
                &PROGRAM,
                [
                    Attribute::new::<Vec3<f32>>("vertex", 0, 0),
                    Attribute::new::<Vec2<f32>>("tex", 1, 0),
                    Attribute::new::<Vec4<f32>>("color", 2, 0),
                ],
            )?,
            target: PhantomData,
        })
    }

    pub fn new() -> Result<Self, GuiiError> {
        Self::new_indexed(0)
    }

    pub fn build<F: Fn(&mut Context<T>) -> ()>(&mut self, f: F) {
        self.vertices.clear();
        self.tex.clear();
        self.colors.clear();

        let (width, height) = T::size();
        let mut context = Context::new(self, width, height);
        f(&mut context);

        // self.shader
        //     .attributes
        //     .position
        //     .set_buffer(self.vertices.get().try_into().unwrap());
        // self.shader
        //     .attributes
        //     .color
        //     .set_buffer(self.colors.get().try_into().unwrap());
    }
}

impl<T: RenderTarget> Renderable<T> for Guii<T> {
    fn render(&self, target: T) {
        // let glyph = font::Glyph::new(&self.font, 'g', 17.0);

        // Configure blend state for alpha blending
        unsafe {
            // Enable color blending
            wut::sys::GX2SetColorControl(
                wut::sys::GX2LogicOp::GX2_LOGIC_OP_COPY,
                1, // blend enable
                0, // dither enable
                1, // color buffer enable
            );

            // Set blend equation and function
            wut::sys::GX2SetBlendControl(
                wut::sys::GX2RenderTarget::GX2_RENDER_TARGET_0,
                wut::sys::GX2BlendMode::GX2_BLEND_MODE_SRC_ALPHA,
                wut::sys::GX2BlendMode::GX2_BLEND_MODE_INV_SRC_ALPHA, // Changed this line
                wut::sys::GX2BlendCombineMode::GX2_BLEND_COMBINE_MODE_ADD,
                1,
                wut::sys::GX2BlendMode::GX2_BLEND_MODE_SRC_ALPHA,
                wut::sys::GX2BlendMode::GX2_BLEND_MODE_INV_SRC_ALPHA, // Changed this line
                wut::sys::GX2BlendCombineMode::GX2_BLEND_COMBINE_MODE_ADD,
            );

            wut::sys::GX2SetDepthOnlyControl(1, 1, CompareFn::Lequal.into());
        }

        self.shader
            .render(target)
            .attribute(self.vertices.get().try_into().unwrap())
            .attribute(self.tex.get().try_into().unwrap())
            .attribute(self.colors.get().try_into().unwrap())
            .uniform_var(ShaderType::Vertex, &self.projection)
            .texture(ShaderType::Pixel, self.atlus.texture())
            .sampler(ShaderType::Pixel, &self.sampler)
            .draw();
    }
}
