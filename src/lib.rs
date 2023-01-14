pub mod builder;
pub mod fuse;
pub mod item;
pub mod path;

#[cfg(test)]
mod tests {

    use std::ptr::read;

    use crate::builder::FuseBuilder;

    use super::{
        builder::{GraphicBuilder, SvgBuilder, TestBuilder},
        fuse,
        item::{
            Color::{Black, Blue, Lime, Purple, Red, White, RGBA},
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
            Rect(0., 0., 50., 50., Strokestyle(10., Lime), Fillstyle(Blue)),
            Ellipse(50., 0., 50., 50., Strokestyle(10., Purple), Fillstyle(Red)),
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
    fn test_scale() {
        let mut items = GraphicItems(vec![
            Rect(0., 0., 50., 50., Strokestyle(10., Lime), Fillstyle(Blue)),
            Ellipse(50., 0., 50., 50., Strokestyle(10., Purple), Fillstyle(Red)),
        ]);
        items = items.scale_items(2., 2., 2.);
        let svg = SvgBuilder::new().build(items);
        std::fs::write("scale.svg", svg).unwrap();
    }

    #[test]
    fn test_builder() {
        let items = GraphicItems(vec![
            Rect(0., 0., 50., 50., Strokestyle(10., Lime), Fillstyle(Blue)),
            Ellipse(50., 0., 50., 50., Strokestyle(10., Purple), Fillstyle(Red)),
        ]);
        TestBuilder::new().build(items);
    }

    #[test]
    fn svg_builder() {
        let items = GraphicItems(vec![
            Rect(0., 0., 50., 50., Strokestyle(5., Lime), Fillstyle(Blue)),
            Ellipse(50., 0., 50., 50., Strokestyle(10., Purple), Fillstyle(Red)),
            Line(0., 50., 100., 0., Strokestyle(5., Red)),
        ]);
        let svg = SvgBuilder::new().build(items);
        std::fs::write("test.svg", svg).unwrap();
    }

    #[test]
    fn svg_path() {
        let items = GraphicItems(vec![Path(
            PathSegments(vec![
                M(0., 0.),
                C(10., 0., 10., 10., 20., 10.),
                L(0., 10.),
                L(0., 0.),
            ]),
            Strokestyle(2., Lime),
            Fillstyle(Blue),
        )]);
        let svg = SvgBuilder::new().build(items);
        std::fs::write("path.svg", svg).unwrap();
    }

    #[test]
    fn test_fuse() {
        // let mut items = GraphicItems(vec![
        //     Rect(0., 0., 100., 100., Strokestyle(5., Red), Fillstyle(Blue)),
        //     // Ellipse(50., 0., 50., 50., Strokestyle(10., 0), Fillstyle(0)),
        //     // Line(0., 0., 100., 100., Strokestyle(5., 0)),
        // ]);
        let mut items = GraphicItems(vec![Path(
            PathSegments(vec![
                M(0., 0.),
                C(100., 0., 100., 100., 200., 100.),
                L(0., 100.),
                L(0., 0.),
            ]),
            Strokestyle(10., Red),
            Fillstyle(Blue),
        )]);

        // let mut items = GraphicItems(vec![
        //     Rect(0., 0., 50., 50., Strokestyle(5., Lime), Fillstyle(Purple)),
        //     Ellipse(50., 0., 50., 50., Strokestyle(10., Blue), Fillstyle(Red)),
        //     Line(0., 50., 100., 0., Strokestyle(5., Lime)),
        // ]);

        let factor = 0.001;
        items = items.scale_items(factor, factor, factor);
        let fuse = FuseBuilder::new().build(items);
        std::fs::write("C:/Users/Cambiata MusikProd/AppData/Roaming/Blackmagic Design/Fusion/Fuses/rust_test_fuse.fuse", fuse).unwrap();
    }

    #[test]
    fn test_json() {
        let json = include_str!("../cadenza/cadenza-8.json");
        let path = PathSegments::from_json(json);
        let path = path.scale_path(0.1, -0.1);
        let items = GraphicItems(vec![Path(path, NoStroke, Fillstyle(White))]);
        let items_fuse = items.scale_items(0.002, -0.002, 0.002);
        let svg = SvgBuilder::new().build(items);
        std::fs::write("cadenza.svg", svg).unwrap();
        let fuse = FuseBuilder::new().build(items_fuse);
        std::fs::write("C:/Users/Cambiata MusikProd/AppData/Roaming/Blackmagic Design/Fusion/Fuses/rust_test_fuse.fuse", fuse).unwrap();
    }
}
