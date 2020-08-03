use crate::{Error, NikoError};
use glow::*;

unsafe fn create_texture(gl: &glow::Context, width: u32, height: u32, data: &[u8]) -> Result<WebTextureKey, Error> {
    let texture = gl.create_texture()
        .map_err(|error| NikoError::PlatformError(error))?;

    gl.bind_texture(glow::TEXTURE_2D, Some(texture));

    gl.tex_image_2d(
        glow::TEXTURE_2D,
        0,
        glow::RGBA as i32,
        width as i32,
        height as i32,
        0,
        glow::RGBA,
        glow::UNSIGNED_BYTE,
        Some(data),
    );

    gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::CLAMP_TO_EDGE as i32);
    gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::CLAMP_TO_EDGE as i32);
    gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32);

    Ok(texture)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Image {
    pub(crate) inner: WebTextureKey,
}

impl Image {
    pub fn create(gl: &glow::Context, width: u32, height: u32, data: &[u8]) -> Result<Self, Error> {

        let inner = unsafe { create_texture(gl, width, height, data)? };

        Ok(Self {
            inner,
        })
    }
}
