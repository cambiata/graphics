pub mod builder;
pub mod item;
pub mod path;

#[cfg(test)]
mod tests {
    use super::{
        builder::{GraphicBuilder, SvgBuilder, TestBuilder},
        item::{
            Fill::{Fillstyle, NoFill},
            GraphicItem::{Ellipse, Line, Path, Rect},
            GraphicItems,
            Stroke::{NoStroke, Strokestyle},
        },
        path::{
            PathSegment::{C, L, M, Q, Z},
            PathSegments,
        },
    };

    #[test]
    fn test_bbox() {
        let items = GraphicItems(vec![
            Rect(0., 0., 50., 50., Strokestyle(10., 0), Fillstyle(0)),
            Ellipse(50., 0., 50., 50., Strokestyle(10., 0), Fillstyle(0)),
        ]);
        let bbox = items.bbox();
        println!("bbox:{:?}", bbox);
    }

    #[test]
    fn test_move() {
        let items = GraphicItems(vec![Rect(0., 0., 10., 10., NoStroke, NoFill)]);
        let moved_items = items.move_items(100., 100.);
        println!("moved_items:{:?}", moved_items);
    }

    #[test]
    fn test_builder() {
        let items = GraphicItems(vec![
            Rect(0., 0., 50., 50., Strokestyle(10., 0), Fillstyle(0)),
            Ellipse(50., 0., 50., 50., Strokestyle(10., 0), Fillstyle(0)),
        ]);
        TestBuilder::new().build(items);
    }

    #[test]
    fn svg_builder() {
        let items = GraphicItems(vec![
            Rect(0., 0., 50., 50., Strokestyle(5., 0), Fillstyle(0)),
            Ellipse(50., 0., 50., 50., Strokestyle(10., 0), Fillstyle(0)),
            Line(0., 50., 100., 0., Strokestyle(5., 0)),
        ]);
        let svg = SvgBuilder::new().build(items);
        std::fs::write("test.svg", svg).unwrap();
    }
}
