use thiserror::*;
use wasm_bindgen::prelude::*;

pub type Error = anyhow::Error;

#[derive(Debug, Error)]
pub enum NikoError {
    #[error("Platform Error: {0}")]
    PlatformError(String)
}
