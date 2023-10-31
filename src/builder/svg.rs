use std::collections::HashMap;

use crate::item::{
    Color,
    Color::{Blue, Lime, Purple, Red, RGBA},
    Fill,
    Fill::{Fillstyle, NoFill},
    GraphicItem, GraphicItems, Stroke,
    Stroke::{NoStroke, Strokestyle},
};
use crate::prelude::*;

use crate::path::{
    PathSegment::{C, L, M, Q, Z},
    PathSegments,
};

pub use super::GraphicBuilder;
use super::{BuilderOptions, SizeUnit};

pub struct SvgBuilder {}

impl SvgBuilder {
    pub fn new() -> Self {
        Self {}
    }
}

impl GraphicBuilder for SvgBuilder {
    fn build(
        &mut self,
        mut items: GraphicItems,
        options: Option<BuilderOptions>,
    ) -> Result<String> {
        // println!("items1:{:?}", items);
        let mut use_cache: HashMap<String, PathSegments> = HashMap::new();

        let items_bbox = items.bbox();
        // println!("items_bbox:{:?}", items_bbox);
        if items_bbox.0 != 0. || items_bbox.1 != 0. {
            items = items.move_items(-items_bbox.0, -items_bbox.1);
        }
        // println!("items2:{:?}", items);

        let mut svg = xmlwriter::XmlWriter::new(xmlwriter::Options::default());
        svg.start_element("svg");
        svg.write_attribute("xmlns", "http://www.w3.org/2000/svg");
        svg.write_attribute("xmlns:xlink", "http://www.w3.org/1999/xlink");

        let svg_width_value = items_bbox.2 + (-items_bbox.0);
        let svg_height_value = items_bbox.3 + (-items_bbox.1);

        let svg_width = match options {
            Some(ref options) => match options.size_unit {
                SizeUnit::Pixel => (round2(svg_width_value * options.size_scaling)).to_string(),
                SizeUnit::Rem => {
                    (round2(svg_width_value * options.size_scaling)).to_string() + "rem"
                }
            },
            None => svg_width_value.to_string(),
        };
        let svg_height = match options {
            Some(ref options) => match options.size_unit {
                SizeUnit::Pixel => (round2(svg_height_value * options.size_scaling)).to_string(),
                SizeUnit::Rem => {
                    (round2(svg_height_value * options.size_scaling)).to_string() + "rem"
                }
            },
            None => svg_height_value.to_string(),
        };

        // dbg!(&options, &svg_width_value, &svg_width, &svg_height);

        svg.write_attribute("width", svg_width.as_str());
        svg.write_attribute("height", svg_height.as_str());

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

        // Cache elements -----------------------------------------
        svg.start_element("defs");
        svg.write_attribute("visibility", "hidden");
        for item in items.0.iter() {
            match item {
                GraphicItem::Path(path, stroke, fill, cache) => match cache {
                    PathCacheInfo::Cache(ref tag, x, y) => {
                        dbg!("Use cache");
                        if !use_cache.contains_key(tag) {
                            use_cache.insert(tag.to_string(), path.clone());
                            svg.start_element("path");
                            svg.write_attribute("id", tag);
                            svg.write_attribute("d", path.to_string().as_str());
                            svg.end_element();
                        }
                    }
                    PathCacheInfo::NoCache => {}
                },
                _ => {}
            }
        }
        svg.end_element();

        for item in items.0.iter() {
            match item {
                GraphicItem::Line(x1, y1, x2, y2, stroke) => {
                    // println!("- Line:{:?}", item);

                    svg.start_element("line");
                    svg.write_attribute("x1", x1);
                    svg.write_attribute("y1", y1);
                    svg.write_attribute("x2", x2);
                    svg.write_attribute("y2", y2);
                    if let Stroke::Strokestyle(w, color) = stroke {
                        svg.write_attribute("stroke", color.to_string().as_str());
                        svg.write_attribute("stroke-width", w);
                    }
                    svg.end_element();
                }
                GraphicItem::Rect(x, y, w, h, stroke, fill) => {
                    // println!("- Rect:{:?}", item);
                    svg.start_element("rect");
                    svg.write_attribute("x", x);
                    svg.write_attribute("y", y);
                    svg.write_attribute("width", w);
                    svg.write_attribute("height", h);
                    if let Stroke::Strokestyle(w, color) = stroke {
                        svg.write_attribute("stroke", color.to_string().as_str());
                        svg.write_attribute("stroke-width", w);
                    }
                    if let Fill::Fillstyle(color) = fill {
                        svg.write_attribute("fill", color.to_string().as_str());
                    } else {
                        svg.write_attribute("fill", "none");
                    }

                    svg.end_element();
                }

                GraphicItem::Ellipse(x, y, w, h, stroke, fill) => {
                    // println!("- Rect:{:?}", item);
                    svg.start_element("ellipse");
                    svg.write_attribute("cx", &(x + w / 2.));
                    svg.write_attribute("cy", &(y + h / 2.));
                    svg.write_attribute("rx", &(w / 2.0));
                    svg.write_attribute("ry", &(h / 2.0));
                    if let Stroke::Strokestyle(w, color) = stroke {
                        svg.write_attribute("stroke", color.to_string().as_str());
                        svg.write_attribute("stroke-width", w);
                    }
                    if let Fill::Fillstyle(color) = fill {
                        svg.write_attribute("fill", color.to_string().as_str());
                    } else {
                        svg.write_attribute("fill", "none");
                    }

                    svg.end_element();
                }

                GraphicItem::Path(path, stroke, fill, cache) => match cache {
                    PathCacheInfo::Cache(ref tag, x, y) => {
                        let path = use_cache.get(tag).unwrap();
                        svg.start_element("use");
                        svg.write_attribute("href", &format!("#{}", tag));
                        svg.write_attribute("x", x);
                        svg.write_attribute("y", y);
                        if let Stroke::Strokestyle(w, color) = stroke {
                            svg.write_attribute("stroke", color.to_string().as_str());
                            svg.write_attribute("stroke-width", w);
                        }
                        if let Fill::Fillstyle(color) = fill {
                            svg.write_attribute("fill", color.to_string().as_str());
                        } else {
                            svg.write_attribute("fill", "none");
                        }
                        svg.end_element();
                    }
                    PathCacheInfo::NoCache => {
                        svg.start_element("path");
                        svg.write_attribute("d", path.to_string().as_str());
                        if let Stroke::Strokestyle(w, color) = stroke {
                            svg.write_attribute("stroke", color.to_string().as_str());
                            svg.write_attribute("stroke-width", w);
                        }
                        if let Fill::Fillstyle(color) = fill {
                            svg.write_attribute("fill", color.to_string().as_str());
                        } else {
                            svg.write_attribute("fill", "none");
                        }
                        svg.end_element();
                    }
                },
            }
        }

        // Bounding rectangle -----------------------------------------
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
        svg.write_attribute("stroke", "transparent");
        svg.write_attribute("stroke-width", "1");
        svg.write_attribute("fill", "none");
        svg.end_element();
        // ------------------------------------------------------

        let str = svg.end_document();
        Ok(str)
    }
}

fn round2(v: f32) -> f32 {
    (v * 100.).round() / 100.
}
