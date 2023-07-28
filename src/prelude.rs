pub use crate::error::GraphicsError::{Basic, Generic};
pub type Result<T> = anyhow::Result<T>;

pub use crate::builder::fuse::*;
pub use crate::builder::svg::*;
pub use crate::glyphs::cadenza::*;
pub use crate::glyphs::ebgaramond::*;
pub use crate::item::Color::*;
pub use crate::item::Fill::*;
pub use crate::item::Fill::{Fillstyle, NoFill};
pub use crate::item::GraphicItem::*;
pub use crate::item::Stroke::*;
pub use crate::item::Stroke::{NoStroke, Strokestyle};
pub use crate::item::*;
pub use crate::path::PathSegment::*;
pub use crate::path::PathSegments;
pub use crate::path::*;
