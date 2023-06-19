use crate::prelude::*;

pub mod fuse;
pub mod svg;

use crate::item::{
    Color,
    Color::{Blue, Lime, Purple, Red, RGBA},
    Fill,
    Fill::{Fillstyle, NoFill},
    GraphicItem, GraphicItems, Stroke,
    Stroke::{NoStroke, Strokestyle},
};

use crate::path::{
    PathSegment::{C, L, M, Q, Z},
    PathSegments,
};

pub trait GraphicBuilder {
    fn build(&mut self, items: GraphicItems) -> Result<String>;
}

//----------------------------------------------------------------------

pub struct TestBuilder;

impl TestBuilder {
    pub fn new() -> Self {
        Self
    }
}

impl GraphicBuilder for TestBuilder {
    fn build(&mut self, items: GraphicItems) -> Result<String> {
        println!("TestBuilder::build()");
        for item in items.0.iter() {
            println!("- item:{:?}", item);
        }
        Ok("TestBuilder".to_string())
    }
}
