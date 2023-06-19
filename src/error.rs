use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GraphicsError {
    Generic(String),
}

impl std::fmt::Display for GraphicsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GraphicsError::Generic(s) => write!(f, "GraphicsError::Generic error: {}", s),
        }
    }
}
