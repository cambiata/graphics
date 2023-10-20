use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GraphicsError {
    Basic,
    Generic(String),
}

impl std::fmt::Display for GraphicsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GraphicsError::Basic => write!(f, "GraphicsError::Basic error"),
            GraphicsError::Generic(s) => write!(f, "GraphicsError::Generic error: {}", s),
        }
    }
}
