use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::prelude::*;

use crate::path::{PathSegment, PathSegments};

#[derive(Debug, Serialize, Deserialize)]
pub enum GraphicItem {
    Line(f32, f32, f32, f32, Stroke),
    Rect(f32, f32, f32, f32, Stroke, Fill),
    Ellipse(f32, f32, f32, f32, Stroke, Fill),
    Path(PathSegments, Stroke, Fill, PathCacheInfo),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PathCacheInfo {
    NoCache,
    Cache(String, f32, f32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Stroke {
    NoStroke,
    Strokestyle(f32, Color), // (width, color)
}

impl Stroke {
    pub fn scale(&self, s: f32) -> Stroke {
        match self {
            Stroke::Strokestyle(w, c) => Stroke::Strokestyle(*w * s, c.clone()),
            Self::NoStroke => Self::NoStroke,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Fill {
    NoFill,
    Fillstyle(Color), // color
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Color {
    RGBA(u8, u8, u8, u8),
    Blue,
    Red,
    Orange,
    Purple,
    Lime,
    Gray,
    LightGray,
    Green,
    Black,
    White,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::RGBA(r, g, b, a) => write!(f, "rgba({},{},{},{})", r, g, b, a),
            Color::Blue => write!(f, "blue"),
            Color::LightGray => write!(f, "lightgray"),
            Color::Gray => write!(f, "gray"),
            Color::Red => write!(f, "red"),
            Color::Orange => write!(f, "orange"),
            Color::Purple => write!(f, "purple"),
            Color::Lime => write!(f, "lime"),
            Color::Green => write!(f, "green"),
            Color::Black => write!(f, "black"),
            Color::White => write!(f, "white"),
        }
    }
}

impl Color {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "blue" => Color::Blue,
            "red" => Color::Red,
            "orange" => Color::Orange,
            "purple" => Color::Purple,
            "lime" => Color::Lime,
            "gray" => Color::Gray,
            "lightgray" => Color::LightGray,
            "green" => Color::Green,
            "black" => Color::Black,
            "white" => Color::White,
            _ => Color::Black,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GraphicItems(pub Vec<GraphicItem>);

impl GraphicItems {}

impl GraphicItems {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn push(&mut self, item: GraphicItem) {
        self.0.push(item)
        // self.0.push(item);
    }

    pub fn extend(&mut self, items: GraphicItems) {
        self.0.extend(items.0);
    }

    pub fn bbox(&self) -> Rectangle {
        let mut x_min = f32::MAX;
        let mut y_min = f32::MAX;
        let mut x_max = f32::MIN;
        let mut y_max = f32::MIN;

        fn get_stroke_width(stroke: &Stroke) -> f32 {
            match stroke {
                Stroke::Strokestyle(w, _) => w / 2.0,
                Stroke::NoStroke => 0.,
            }
        }

        for item in self.0.iter() {
            match item {
                GraphicItem::Line(x1, y1, x2, y2, stroke) => {
                    let sw = get_stroke_width(stroke);
                    x_min = x_min.min(*x1 - sw);
                    y_min = y_min.min(*y1 - sw);
                    x_max = x_max.max(*x1 + sw);
                    y_max = y_max.max(*y1 + sw);
                    x_min = x_min.min(*x2 - sw);
                    y_min = y_min.min(*y2 - sw);
                    x_max = x_max.max(*x2 + sw);
                    y_max = y_max.max(*y2 + sw);
                }
                GraphicItem::Rect(x, y, w, h, stroke, _) => {
                    let sw = get_stroke_width(stroke);
                    x_min = x_min.min(*x - sw);
                    y_min = y_min.min(*y - sw);
                    x_max = x_max.max(*x + sw);
                    y_max = y_max.max(*y + sw);
                    x_min = x_min.min(*x + *w - sw);
                    y_min = y_min.min(*y + *h - sw);
                    x_max = x_max.max(*x + *w + sw);
                    y_max = y_max.max(*y + *h + sw);
                }

                GraphicItem::Ellipse(x, y, w, h, stroke, _) => {
                    let sw = get_stroke_width(stroke);
                    x_min = x_min.min(*x - sw);
                    y_min = y_min.min(*y - sw);
                    x_max = x_max.max(*x + sw);
                    y_max = y_max.max(*y + sw);
                    x_min = x_min.min(*x + *w - sw);
                    y_min = y_min.min(*y + *h - sw);
                    x_max = x_max.max(*x + *w + sw);
                    y_max = y_max.max(*y + *h + sw);
                }

                GraphicItem::Path(path, stroke, _, _) => {
                    let sw = get_stroke_width(stroke);

                    for segment in path.0.iter() {
                        match segment {
                            PathSegment::M(x, y) | PathSegment::L(x, y) => {
                                x_min = x_min.min(*x - sw);
                                y_min = y_min.min(*y - sw);
                                x_max = x_max.max(*x + sw);
                                y_max = y_max.max(*y + sw);
                            }
                            PathSegment::Q(x1, y1, x, y) => {
                                x_min = x_min.min(*x - sw);
                                y_min = y_min.min(*y - sw);
                                x_max = x_max.max(*x + sw);
                                y_max = y_max.max(*y + sw);
                                x_min = x_min.min(*x1 - sw);
                                y_min = y_min.min(*y1 - sw);
                                x_max = x_max.max(*x1 + sw);
                                y_max = y_max.max(*y1 + sw);
                            }
                            PathSegment::C(x1, y1, x2, y2, x, y) => {
                                x_min = x_min.min(*x);
                                y_min = y_min.min(*y);
                                x_max = x_max.max(*x);
                                y_max = y_max.max(*y);
                                x_min = x_min.min(*x1);
                                y_min = y_min.min(*y1);
                                x_max = x_max.max(*x1);
                                y_max = y_max.max(*y1);
                                x_min = x_min.min(*x2);
                                y_min = y_min.min(*y2);
                                x_max = x_max.max(*x2);
                                y_max = y_max.max(*y2);
                            }
                            PathSegment::Z => {}
                        }
                    }
                }
            }
        }
        // println!(
        //     "x_min:{}, y_min:{}, x_max:{}, y_max:{}",
        //     x_min, y_min, x_max, y_max
        // );
        Rectangle(x_min, y_min, x_max, y_max)
    }

    pub fn move_items(&self, move_x: f32, move_y: f32) -> GraphicItems {
        let mut ret = vec![];
        for item in self.0.iter() {
            let new_item = match item {
                GraphicItem::Line(x1, y1, x2, y2, stroke) => GraphicItem::Line(x1 + move_x, y1 + move_y, x2 + move_x, y2 + move_y, stroke.clone()),
                GraphicItem::Rect(x, y, w, h, stroke, fill) => GraphicItem::Rect(x + move_x, y + move_y, *w, *h, stroke.clone(), fill.clone()),
                GraphicItem::Ellipse(x, y, w, h, stroke, fill) => GraphicItem::Ellipse(*x + move_x, y + move_y, *w, *h, stroke.clone(), fill.clone()),
                GraphicItem::Path(path, stroke, fill, cache) => GraphicItem::Path(path.move_path(move_x, move_y), stroke.clone(), fill.clone(), cache.clone()),
            };
            ret.push(new_item);
        }

        return GraphicItems(ret);
    }

    pub fn scale_items(&self, scale_x: f32, scale_y: f32, scale_stroke: f32) -> GraphicItems {
        let mut ret = vec![];

        for item in self.0.iter() {
            let new_item = match item {
                GraphicItem::Line(x1, y1, x2, y2, stroke) => GraphicItem::Line(x1 * scale_x, y1 * scale_y, x2 * scale_x, y2 * scale_y, stroke.scale(scale_stroke)),
                GraphicItem::Rect(x, y, w, h, stroke, fill) => GraphicItem::Rect(x * scale_x, y * scale_y, w * scale_x, h * scale_y, stroke.scale(scale_stroke), fill.clone()),
                GraphicItem::Ellipse(x, y, w, h, stroke, fill) => GraphicItem::Ellipse(x * scale_x, y * scale_y, w * scale_x, h * scale_y, stroke.clone(), fill.clone()),
                GraphicItem::Path(path, stroke, fill, cache) => GraphicItem::Path(path.scale_path(scale_x, scale_y), stroke.scale(scale_stroke), fill.clone(), cache.clone()),
            };
            ret.push(new_item);
        }
        return GraphicItems(ret);
    }
}
