pub mod builder;
pub mod item;
pub mod path;

#[cfg(test)]
mod tests {
    use super::{
        builder::{GraphicBuilder, SvgBuilder, TestBuilder},
        item::{
            Fill::{Fillstyle, NoFill},
            GraphicItem::{Line, Path, Rect},
            GraphicItems,
            Stroke::{NoStroke, Strokestyle},
        },
        path::{
            PathSegment::{C, L, M, Q, Z},
            PathSegments,
        },
    };

    fn testdata() -> GraphicItems {
        let line = Line(10., 10., 100., 10., Strokestyle(3, 0));
        let path = Path(
            PathSegments(vec![M(5.0, 2.0), L(10.0, 10.0)]),
            NoStroke,
            NoFill,
        );
        let items = GraphicItems(vec![
            // line,
            path,
        ]);
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
    fn test_bbox() {
        let items = testdata();
        let bbox = items.bbox();
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
