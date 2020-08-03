use thiserror::*;

#[derive(Debug, Error)]
pub enum ShaderError {
    #[error("Error compiling shader: {0}")]
    ShaderCompileError(String),

    #[error("No attribute with name {0} on shader")]
    AttributeNotFound(String),

    #[error("No uniform with name {0} on shader")]
    UniformNotFound(String),
}
