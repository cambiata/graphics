use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rectangle(pub f32, pub f32, pub f32, pub f32);
