use crate::path::{PathSegment, PathSegments};

#[derive(Debug)]
pub enum GraphicItem {
    Line(f32, f32, f32, f32, Stroke),
    Rect(f32, f32, f32, f32, Stroke, Fill),
    Path(PathSegments, Stroke, Fill),
}

#[derive(Debug)]
pub enum Stroke {
    NoStroke,
    Strokestyle(u32, i32), // (width, color)
}

#[derive(Debug)]
pub enum Fill {
    NoFill,
    Fillstyle(i32), // color
}

pub struct GraphicItems(pub Vec<GraphicItem>);

impl GraphicItems {
    pub fn bbox(&self) {
        let mut x_min = f32::MAX;
        let mut y_min = f32::MAX;
        let mut x_max = f32::MIN;
        let mut y_max = f32::MIN;

        for item in self.0.iter() {
            match item {
                GraphicItem::Line(x1, y1, x2, y2, _) => {
                    x_min = x_min.min(*x1);
                    y_min = y_min.min(*y1);
                    x_max = x_max.max(*x1);
                    y_max = y_max.max(*y1);
                    x_min = x_min.min(*x2);
                    y_min = y_min.min(*y2);
                    x_max = x_max.max(*x2);
                    y_max = y_max.max(*y2);
                }
                GraphicItem::Rect(x, y, w, h, _, _) => {
                    x_min = x_min.min(*x);
                    y_min = y_min.min(*y);
                    x_max = x_max.max(*x);
                    y_max = y_max.max(*y);
                    x_min = x_min.min(*x + *w);
                    y_min = y_min.min(*y + *h);
                    x_max = x_max.max(*x + *w);
                    y_max = y_max.max(*y + *h);
                }
                GraphicItem::Path(path, _, _) => {
                    for segment in path.0.iter() {
                        match segment {
                            PathSegment::M(x, y) | PathSegment::L(x, y) => {
                                x_min = x_min.min(*x);
                                y_min = y_min.min(*y);
                                x_max = x_max.max(*x);
                                y_max = y_max.max(*y);
                            }
                            PathSegment::Q(x1, y1, x, y) => {
                                x_min = x_min.min(*x);
                                y_min = y_min.min(*y);
                                x_max = x_max.max(*x);
                                y_max = y_max.max(*y);
                                x_min = x_min.min(*x1);
                                y_min = y_min.min(*y1);
                                x_max = x_max.max(*x1);
                                y_max = y_max.max(*y1);
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
        println!(
            "x_min:{}, y_min:{}, x_max:{}, y_max:{}",
            x_min, y_min, x_max, y_max
        );
    }
}
