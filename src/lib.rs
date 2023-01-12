#[derive(Debug)]
pub enum PathSegment {
    M(f32, f32),
    L(f32, f32),
    Q(f32, f32, f32, f32),
    C(f32, f32, f32, f32, f32, f32),
    Z,
}
#[derive(Debug)]
pub struct PathSegments(Vec<PathSegment>);

impl PathSegments {
    fn to_string(&self) -> String {
        use std::fmt::Write;
        let mut path_buf = String::from("");
        for segment in self.0.iter() {
            match segment {
                PathSegment::M(x, y) => write!(path_buf, "M {} {} ", x, y).unwrap(),
                PathSegment::L(x, y) => write!(path_buf, "L {} {} ", x, y).unwrap(),
                PathSegment::Q(x1, y1, x, y) => {
                    write!(path_buf, "Q {} {} {} {} ", x1, y1, x, y).unwrap()
                }
                PathSegment::C(x1, y1, x2, y2, x, y) => {
                    write!(path_buf, "C {} {} {} {} {} {} ", x1, y1, x2, y2, x, y).unwrap()
                }
                PathSegment::Z => path_buf.push_str("Z "),
            }
        }
        println!("path_buf:{:?}", path_buf);
        path_buf
    }
}

#[derive(Debug)]
pub enum GraphicItem {
    Line(i32, i32, i32, i32, Stroke),
    Rect(i32, i32, i32, i32, Stroke, Fill),
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

pub struct GraphicItems(Vec<GraphicItem>);

pub trait GraphicBuilder {
    fn build(&mut self, items: GraphicItems);
}
struct TestBuilder;

impl TestBuilder {
    fn new() -> Self {
        Self
    }
}

impl GraphicBuilder for TestBuilder {
    fn build(&mut self, items: GraphicItems) {
        println!("TestBuilder::build()");
        for item in items.0.iter() {
            println!("- item:{:?}", item);
        }
    }
}

struct SvgBuilder {}

impl SvgBuilder {
    fn new() -> Self {
        Self {}
    }
}

impl GraphicBuilder for SvgBuilder {
    fn build(&mut self, items: GraphicItems) {
        let mut svg = xmlwriter::XmlWriter::new(xmlwriter::Options::default());
        svg.start_element("svg");
        svg.write_attribute_fmt("viewBox", format_args!("{} {} {} {}", 0, 0, 100, 100));
        for item in items.0.iter() {
            match item {
                GraphicItem::Line(x1, y1, x2, y2, stroke) => {
                    println!("- Line:{:?}", item);
                    svg.start_element("line");
                    svg.write_attribute("x1", x1);
                    svg.write_attribute("y1", y1);
                    svg.write_attribute("x2", x2);
                    svg.write_attribute("y2", y2);
                    if let Stroke::Strokestyle(w, color) = stroke {
                        svg.write_attribute("stroke", color);
                        svg.write_attribute("stroke-width", w);
                    }
                    svg.end_element();
                }
                GraphicItem::Rect(x, y, w, h, stroke, fill) => {
                    println!("- Rect:{:?}", item);
                    svg.start_element("rect");
                    svg.write_attribute("x", x);
                    svg.write_attribute("y", y);
                    svg.write_attribute("w", w);
                    svg.write_attribute("h", h);
                    if let Stroke::Strokestyle(w, color) = stroke {
                        svg.write_attribute("stroke", color);
                        svg.write_attribute("stroke-width", w);
                    }
                    if let Fill::Fillstyle(color) = fill {
                        svg.write_attribute("color", color);
                    }

                    svg.end_element();
                }
                GraphicItem::Path(path, stroke, fill) => {
                    println!("- Path:{:?}", item);
                    svg.start_element("path");
                    svg.write_attribute("d", path.to_string().as_str());
                    if let Stroke::Strokestyle(w, color) = stroke {
                        svg.write_attribute("stroke", color);
                        svg.write_attribute("stroke-width", w);
                    }
                    if let Fill::Fillstyle(color) = fill {
                        svg.write_attribute("color", color);
                    }
                    svg.end_element();
                }
            }
        }
        let str = svg.end_document();
        println!("str:{}", str);
    }
}

#[cfg(test)]
mod tests {
    use crate::{GraphicBuilder, PathSegment, SvgBuilder, TestBuilder};

    use super::{
        Fill::{Fillstyle, NoFill},
        GraphicItem::{Line, Path, Rect},
        GraphicItems,
        PathSegment::{C, L, M, Q, Z},
        PathSegments,
        Stroke::{NoStroke, Strokestyle},
    };

    fn testdata() -> GraphicItems {
        let line = Line(10, 10, 100, 10, Strokestyle(3, 0));
        let path = Path(
            PathSegments(vec![M(0.0, 0.0), L(10.0, 10.0)]),
            NoStroke,
            NoFill,
        );
        let items = GraphicItems(vec![line, path]);
        items
    }

    #[test]
    fn example() {
        assert_eq!(3, 2 + 1);
    }

    #[test]
    fn test_line() {
        let items = testdata();
        assert_eq!(items.0.len(), 1);
    }

    #[test]
    fn test_builder() {
        let items = testdata();
        TestBuilder::new().build(items);
    }

    #[test]
    fn svg_builder() {
        let items = testdata();
        SvgBuilder::new().build(items);
    }
}
