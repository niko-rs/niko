use thiserror::*;

pub type Error = anyhow::Error;

#[derive(Debug, Error)]
pub enum NikoError {
    #[error("Platform Error: {0}")]
    PlatformError(String)
}
