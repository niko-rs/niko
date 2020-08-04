mod buffer;
mod buffer_type;
mod buffer_usage;
mod image;
mod shader;
mod sprite;
mod sprite_batch;
mod error;
mod check_error;

pub use buffer::*;
pub use buffer_type::*;
pub use buffer_usage::*;
pub use image::*;
pub use shader::*;
pub use sprite::*;
pub use sprite_batch::*;
pub use error::*;
use check_error::*;

pub type ShaderId = <glow::Context as glow::HasContext>::Shader;
pub type ProgramId = <glow::Context as glow::HasContext>::Program;
pub type BufferId = <glow::Context as glow::HasContext>::Buffer;
pub type TextureId = <glow::Context as glow::HasContext>::Texture;
pub type UniformLocationId = <glow::Context as glow::HasContext>::UniformLocation;
