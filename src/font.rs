use rusttype::{Font, OutlineBuilder, Point, Rect, Scale};

use crate::path::PathSegment;

pub struct PathText {
    pub bounding_box: Rect<f32>,
    pub path_segments: Vec<PathSegment>,
}

impl PathText {
    pub fn new(bounding_box: Rect<f32>, p: Vec<PathSegment>) -> Self {
        Self {
            bounding_box,
            path_segments: p,
        }
    }

    pub fn builder() -> FontTextBuilder<'static> {
        Default::default()
    }
}

pub struct FontTextBuilder<'a> {
    pub fill: &'a str,
    pub size: f32,
    pub start: Point<f32>,
    pub letter_spacing: f32,
}

impl Default for FontTextBuilder<'static> {
    fn default() -> Self {
        Self {
            fill: "#000",
            size: 32.,
            start: Point { x: 0., y: 0. },
            letter_spacing: 1.,
        }
    }
}

impl FontTextBuilder<'_> {
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn start(mut self, start: Point<f32>) -> Self {
        self.start = start;
        self
    }

    pub fn build(&self, font: &Font, text: &str) -> PathText {
        let mut p: Vec<PathSegment> = Vec::new();
        let mut x = self.start.x;
        let scale = Scale::uniform(self.size);
        let v_metrics: rusttype::VMetrics = font.v_metrics(scale);

        // println!("txt:{:?}", text);
        // println!("scale:{:?}", scale);
        // println!("v_metrics:{:?}", v_metrics);
        // println!("self.start:{:?}", self.start);
        // println!("self.letter_spacing:{:?}", self.letter_spacing);

        for glyph in font.layout(
            text,
            scale,
            Point {
                x,
                y: self.start.y + v_metrics.ascent,
            },
        ) {
            // println!("- glyph:{:?}", glyph);
            let bounding_box = match glyph.unpositioned().exact_bounding_box() {
                Some(bounding_box) => bounding_box,
                None => Rect {
                    min: Point {
                        x: scale.x / 5.,
                        y: 0.,
                    },
                    max: Point {
                        x: scale.x / 5.,
                        y: 0.,
                    },
                },
            };

            x += bounding_box.min.x;

            let mut xpathbuilder = crate::font::PathTextBuilder {
                x: x,
                y: v_metrics.ascent + bounding_box.min.y,
                p: &mut p,
            };

            glyph.build_outline(&mut xpathbuilder);
            x += bounding_box.width() + self.letter_spacing;
        }

        let bounding_box = Rect {
            min: self.start,
            max: Point { x, y: self.size },
        };

        PathText::new(bounding_box, p)
    }
}

pub struct PathTextBuilder<'a> {
    pub x: f32,
    pub y: f32,
    pub p: &'a mut Vec<PathSegment>,
}

impl<'a> PathTextBuilder<'a> {
    pub fn new(x: f32, y: f32, p: &'a mut Vec<PathSegment>) -> Self {
        // let mut p: Vec<PathSegment> = vec![];
        Self { x, y, p }
    }
}

impl OutlineBuilder for PathTextBuilder<'_> {
    fn move_to(&mut self, x: f32, y: f32) {
        // println!("move_to:");
        self.p.push(PathSegment::M(x + self.x, y + self.y));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        // println!("line_to:");
        self.p.push(PathSegment::L(x + self.x, y + self.y));
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        // println!("quad_to:");
        self.p.push(PathSegment::Q(
            x1 + self.x,
            y1 + self.y,
            x + self.x,
            y + self.y,
        ));
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        // println!("curve_to:");
        self.p.push(PathSegment::C(
            x1 + self.x,
            y1 + self.y,
            x2 + self.x,
            y2 + self.y,
            x + self.x,
            y + self.y,
        ));
    }

    fn close(&mut self) {
        self.p.push(PathSegment::Z);
    }
}
