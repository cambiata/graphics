use std::fmt;

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
    fn build(&mut self, items: GraphicItems) -> String;
}

pub struct SvgBuilder {}

impl SvgBuilder {
    pub fn new() -> Self {
        Self {}
    }
}

impl GraphicBuilder for SvgBuilder {
    fn build(&mut self, mut items: GraphicItems) -> String {
        let items_bbox = items.bbox();
        println!("items_bbox:{:?}", items_bbox);
        if items_bbox.0 != 0. || items_bbox.1 != 0. {
            println!("MOVE {} {}", items_bbox.0, items_bbox.1);
            items = items.move_items(-items_bbox.0, -items_bbox.1);
        }
        println!("items2:{:?}", items);

        let mut svg = xmlwriter::XmlWriter::new(xmlwriter::Options::default());
        svg.start_element("svg");
        svg.write_attribute_fmt(
            "viewBox",
            format_args!(
                "{} {} {} {}",
                0,
                0,
                items_bbox.2 + (-items_bbox.0),
                items_bbox.3 + (-items_bbox.1)
            ),
        );

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
                        svg.write_attribute("stroke", "orange");
                        svg.write_attribute("stroke-width", w);
                    }
                    svg.end_element();
                }
                GraphicItem::Rect(x, y, w, h, stroke, fill) => {
                    println!("- Rect:{:?}", item);
                    svg.start_element("rect");
                    svg.write_attribute("x", x);
                    svg.write_attribute("y", y);
                    svg.write_attribute("width", w);
                    svg.write_attribute("height", h);
                    if let Stroke::Strokestyle(w, color) = stroke {
                        svg.write_attribute("stroke", "blue");
                        svg.write_attribute("stroke-width", w);
                    }
                    if let Fill::Fillstyle(color) = fill {
                        svg.write_attribute("fill", "red");
                    } else {
                        svg.write_attribute("fill", "none");
                    }

                    svg.end_element();
                }

                GraphicItem::Ellipse(x, y, w, h, stroke, fill) => {
                    println!("- Rect:{:?}", item);
                    svg.start_element("ellipse");
                    svg.write_attribute("cx", &(x + w / 2.));
                    svg.write_attribute("cy", &(y + h / 2.));
                    svg.write_attribute("rx", &(w / 2.0));
                    svg.write_attribute("ry", &(h / 2.0));
                    if let Stroke::Strokestyle(w, color) = stroke {
                        svg.write_attribute("stroke", "purple");
                        svg.write_attribute("stroke-width", w);
                    }
                    if let Fill::Fillstyle(color) = fill {
                        svg.write_attribute("fill", "lime");
                    } else {
                        svg.write_attribute("fill", "none");
                    }

                    svg.end_element();
                }

                GraphicItem::Path(path, stroke, fill) => {
                    println!("- Path:{:?}", item);
                    svg.start_element("path");
                    svg.write_attribute("d", path.to_string().as_str());
                    if let Stroke::Strokestyle(w, color) = stroke {
                        svg.write_attribute("stroke", "dodgerblue");
                        svg.write_attribute("stroke-width", w);
                    }
                    if let Fill::Fillstyle(color) = fill {
                        svg.write_attribute("fill", "tomato");
                    } else {
                        svg.write_attribute("fill", "none");
                    }
                    svg.end_element();
                }
            }
        }

        svg.start_element("rect");
        svg.write_attribute("x", "0");
        svg.write_attribute("y", "0");
        svg.write_attribute(
            "width",
            (items_bbox.2 + (-items_bbox.0)).to_string().as_str(),
        );
        svg.write_attribute(
            "height",
            (items_bbox.3 + (-items_bbox.1)).to_string().as_str(),
        );
        svg.write_attribute("stroke", "black");
        svg.write_attribute("stroke-width", "1");
        svg.write_attribute("fill", "none");
        svg.end_element();

        let str = svg.end_document();
        println!("str:{}", str);
        str
    }
}

pub struct TestBuilder;

impl TestBuilder {
    pub fn new() -> Self {
        Self
    }
}

impl GraphicBuilder for TestBuilder {
    fn build(&mut self, items: GraphicItems) -> String {
        println!("TestBuilder::build()");
        for item in items.0.iter() {
            println!("- item:{:?}", item);
        }
        "TestBuilder".to_string()
    }
}

pub struct FuseBuilder;

impl FuseBuilder {
    pub fn new() -> Self {
        Self
    }
}

impl GraphicBuilder for FuseBuilder {
    fn build(&mut self, mut items: GraphicItems) -> String {
        let items_bbox = items.bbox();
        println!("items_bbox:{:?}", items_bbox);
        if items_bbox.0 != 0. || items_bbox.1 != 0. {
            println!("MOVE {} {}", items_bbox.0, items_bbox.1);
            items = items.move_items(-items_bbox.0, -items_bbox.1);
        }

        let before = include_str!("fuse_before.txt");
        let after = include_str!("fuse_after.txt");
        let mut buffer = "\n\n-- dynamically added items:\n\n".to_string();

        fn get_fill_color(color: &Color) -> &str {
            println!("- - - - get color:{:?}", color);
            match color {
                Blue => "{R = 0, G = 0, B = 1, A = 1}",
                Red => "{R = 1, G = 0, B = 0, A = 1}",
                Lime => "{R = 0, G = 1, B = 0, A = 1}",
                Purple => "{R = .5, G = 0, B = .5, A = 1}",
                Black => "{R = 0, G = 0, B = 0, A = 1}",
                White => "{R = 1, G = 1, B = 1, A = 1}",
                _ => "{R = .5, G = .5, B = .5, A = 1}",
            }
        }

        fn add_after_line(mut buffer: String, color: &Color) -> String {
            let color_str = get_fill_color(color);
            buffer.push_str("\n\tic = ImageChannel(out, 8)");
            buffer.push_str("\n\tic:ShapeFill(line)	");
            buffer.push_str("\n\tcs = ChannelStyle()");
            buffer.push_str(format!("\n\tcs.Color = Pixel({})", color_str).as_str());
            buffer.push_str("\n\tif self.Status == \"OK\" then");
            buffer.push_str("\n\t    ic:PutToImage(\"CM_Merge\", cs)");
            buffer.push_str("\n\tend");
            buffer.push_str("\n\t");
            buffer
        }

        for item in items.0.iter() {
            match item {
                GraphicItem::Line(x1, y1, x2, y2, stroke) => {
                    if let Strokestyle(width, color) = stroke {
                        buffer.push_str("\n\tline = Shape()");
                        buffer.push_str(format!("\n\tline:MoveTo({}, {})", *x1, *y1).as_str());
                        buffer.push_str(format!("\n\tline:LineTo({}, {})", *x2, *y2).as_str());
                        buffer.push_str(
                            format!("\n\tline = line:OutlineOfShape({},\"OLT_Solid\")", width)
                                .as_str(),
                        );
                        buffer = add_after_line(buffer, color);
                    }
                }
                GraphicItem::Rect(x, y, w, h, stroke, fill) => {
                    if let Fillstyle(color) = fill {
                        buffer.push_str("\n\tline = Shape()");
                        buffer.push_str(format!("\n\tline:MoveTo({}, {})", *x, *y).as_str());
                        buffer.push_str(format!("\n\tline:LineTo({}, {})", *x + *w, *y).as_str());
                        buffer.push_str(
                            format!("\n\tline:LineTo({}, {})", *x + *w, *y + *h).as_str(),
                        );
                        buffer.push_str(format!("\n\tline:LineTo({}, {})", *x, *y + *h).as_str());
                        buffer.push_str("\n\tline:Close()");
                        buffer = add_after_line(buffer, color);
                    }

                    if let Strokestyle(width, color) = stroke {
                        buffer.push_str("\n\tline = Shape()");
                        buffer.push_str(format!("\n\tline:MoveTo({}, {})", *x, *y).as_str());
                        buffer.push_str(format!("\n\tline:LineTo({}, {})", *x + *w, *y).as_str());
                        buffer.push_str(
                            format!("\n\tline:LineTo({}, {})", *x + *w, *y + *h).as_str(),
                        );
                        buffer.push_str(format!("\n\tline:LineTo({}, {})", *x, *y + *h).as_str());
                        buffer.push_str(format!("\n\tline:LineTo({}, {})", *x, *y).as_str());
                        buffer.push_str(
                            format!("\n\tline = line:OutlineOfShape({},\"OLT_Solid\")", width)
                                .as_str(),
                        );
                        buffer = add_after_line(buffer, color);
                    }
                }
                GraphicItem::Path(path, stroke, fill) => {
                    fn add_path(mut buffer: String, path: &PathSegments) -> String {
                        let mut prev_x: f32 = 0.;
                        let mut prev_y: f32 = 0.;
                        buffer.push_str("\n\tline = Shape()");
                        for segment in path.0.iter() {
                            match segment {
                                M(x, y) => {
                                    buffer.push_str(
                                        format!("\n\tline:MoveTo({}, {})", x, y).as_str(),
                                    );
                                    prev_x = *x;
                                    prev_y = *y;
                                }
                                L(x, y) => {
                                    buffer.push_str(
                                        format!("\n\tline:LineTo({}, {})", x, y).as_str(),
                                    );
                                    prev_x = *x;
                                    prev_y = *y;
                                }
                                Q(x1, y1, x, y) => {
                                    let c1x = prev_x + (2. / 3.) * (x1 - prev_x);
                                    let c1y = prev_y + (2. / 3.) * (y1 - prev_y);
                                    let c2x = x + (2. / 3.) * (x1 - x);
                                    let c2y = y + (2. / 3.) * (y1 - y);

                                    buffer.push_str(format!("\n\tline = BezierTo2(line, {{X={}, Y={}}}, {{X={}, Y={}}}, {{X={}, Y={}}}, {{X={}, Y={}}}, 20)", prev_x, prev_y, c1x,c1y, c2x, c2y, x, y).as_str());

                                    prev_x = *x;
                                    prev_y = *y;
                                }
                                C(x1, y1, x2, y2, x, y) => {
                                    // addLine('line = BezierTo2(line, {X=${cubic.sx}, Y=${cubic.sy}}, {X=${cubic.c1x}, Y=${cubic.c1y}},  {X=${cubic.c2x}, Y=${cubic.c2y}}, {X=${cubic.ex}, Y=${cubic.ey}}, 20)');
                                    buffer.push_str(format!("\n\tline = BezierTo2(line, {{X={}, Y={}}}, {{X={}, Y={}}}, {{X={}, Y={}}}, {{X={}, Y={}}}, 20)", prev_x, prev_y, x1, y1, x2, y2, x, y).as_str());
                                    prev_x = *x;
                                    prev_y = *y;
                                }
                                Z => {}
                            }
                        }
                        buffer
                    }

                    if let Fillstyle(color) = fill {
                        buffer = add_path(buffer, &path);
                        buffer = add_after_line(buffer, color);
                    }

                    if let Strokestyle(width, color) = stroke {
                        buffer = add_path(buffer, &path);
                        buffer.push_str(
                            format!("\n\tline = line:OutlineOfShape({},\"OLT_Solid\")", width)
                                .as_str(),
                        );
                        buffer = add_after_line(buffer, color);
                    }
                }
                _ => {}
            }
        }

        let contents = format!("{}\n\n{}\n\n{}", before, buffer, after);
        contents
    }
}
