use crate::path::{PathSegment, PathSegments};

#[derive(Debug, Clone)]
pub struct Rectangle(pub f32, pub f32, pub f32, pub f32);

#[derive(Debug)]
pub enum GraphicItem {
    Line(f32, f32, f32, f32, Stroke),
    Rect(f32, f32, f32, f32, Stroke, Fill),
    Ellipse(f32, f32, f32, f32, Stroke, Fill),
    Path(PathSegments, Stroke, Fill),
}

#[derive(Debug, Clone)]
pub enum Stroke {
    NoStroke,
    Strokestyle(f32, i32), // (width, color)
}

#[derive(Debug, Clone)]
pub enum Fill {
    NoFill,
    Fillstyle(i32), // color
}

#[derive(Debug)]
pub struct GraphicItems(pub Vec<GraphicItem>);

impl GraphicItems {
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

                GraphicItem::Path(path, stroke, _) => {
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
        println!(
            "x_min:{}, y_min:{}, x_max:{}, y_max:{}",
            x_min, y_min, x_max, y_max
        );
        Rectangle(x_min, y_min, x_max, y_max)
    }

    pub fn move_items(&self, move_x: f32, move_y: f32) -> GraphicItems {
        let mut ret = vec![];
        for item in self.0.iter() {
            let new_item = match item {
                GraphicItem::Line(x1, y1, x2, y2, stroke) => GraphicItem::Line(
                    x1 + move_x,
                    y1 + move_y,
                    x2 + move_x,
                    y2 + move_y,
                    stroke.clone(),
                ),
                GraphicItem::Rect(x, y, w, h, stroke, fill) => {
                    GraphicItem::Rect(x + move_x, y + move_y, *w, *h, stroke.clone(), fill.clone())
                }
                GraphicItem::Ellipse(x, y, w, h, stroke, fill) => GraphicItem::Ellipse(
                    *x + move_x,
                    y + move_y,
                    *w,
                    *h,
                    stroke.clone(),
                    fill.clone(),
                ),
                GraphicItem::Path(path, stroke, fill) => {
                    GraphicItem::Path(path.clone(), stroke.clone(), fill.clone())
                }
            };
            ret.push(new_item);
        }

        return GraphicItems(ret);
    }
}
