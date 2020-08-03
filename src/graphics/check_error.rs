use crate::Error;
use glow::{Context, HasContext};
use thiserror::*;

#[derive(Debug, Error)]
pub enum WebGLError {
    #[error("An unacceptable value has been specified for an enumerated argument. The command is ignored and the error flag is set.")]
    InvalidEnum,

    #[error("A numeric argument is out of range. The command is ignored and the error flag is set.")]
    InvalidValue,

    #[error("The specified command is not allowed for the current state. The command is ignored and the error flag is set.")]
    InvalidOperation,

    #[error("The currently bound framebuffer is not framebuffer complete when trying to render to or to read from it.")]
    InvalidFramebufferOperation,

    #[error("Not enough memory is left to execute the command.")]
    OutOfMemory,

    #[error("If the WebGL context is lost, this error is returned on the first call to getError. Afterwards and until the context has been restored, it returns gl.NO_ERROR.")]
    ContextLost,

    #[error("An unknown error occured")]
    Unknown,
}

pub fn check_error(gl: &Context) -> Result<(), Error> {
    unsafe {
        match gl.get_error() {
            glow::NO_ERROR => Ok(()),
            glow::INVALID_ENUM => Err(WebGLError::InvalidEnum.into()),
            glow::INVALID_VALUE => Err(WebGLError::InvalidValue.into()),
            glow::INVALID_OPERATION => Err(WebGLError::InvalidOperation.into()),
            glow::INVALID_FRAMEBUFFER_OPERATION => Err(WebGLError::InvalidFramebufferOperation.into()),
            glow::OUT_OF_MEMORY => Err(WebGLError::OutOfMemory.into()),
            glow::CONTEXT_LOST => Err(WebGLError::ContextLost.into()),
            _ => Err(WebGLError::Unknown.into()),
        }
    }
}
