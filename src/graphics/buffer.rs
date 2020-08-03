use glow::HasContext;
use crate::{
    Error,
    NikoError,
    graphics::{
        BufferType,
        BufferUsage,
        BufferId,
    },
};

unsafe fn create_buffer(gl: &glow::Context, target: u32, usage: u32, data: &[u8]) -> Result<BufferId, Error> {
    let buffer = gl.create_buffer()
        .map_err(|error| NikoError::PlatformError(error))?;

    gl.bind_buffer(target, Some(buffer));
    gl.buffer_data_u8_slice(target, data, usage);

    Ok(buffer)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Buffer {
    inner: BufferId,
}

impl Buffer {
    pub fn create(gl: &glow::Context, buffer_type: BufferType, buffer_usage: BufferUsage, data: &[u8]) -> Result<Self, Error> {
        let inner = unsafe { create_buffer(gl, buffer_type.into(), buffer_usage.into(), data)? };

        Ok(Self {
            inner,
        })
    }

    pub(crate) fn get_inner(&self) -> BufferId {
        self.inner
    }
}
