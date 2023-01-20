use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum PathSegment {
    M(f32, f32),
    L(f32, f32),
    Q(f32, f32, f32, f32),
    C(f32, f32, f32, f32, f32, f32),
    Z,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PathSegments(pub Vec<PathSegment>);

impl PathSegments {
    pub fn to_string(&self) -> String {
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

    pub fn move_path(&self, move_x: f32, move_y: f32) -> PathSegments {
        let mut segments: Vec<PathSegment> = vec![];
        for segment in self.0.iter() {
            let new_segment: PathSegment = match segment {
                PathSegment::M(x, y) => PathSegment::M(x + move_x, y + move_y),
                PathSegment::L(x, y) => PathSegment::L(x + move_x, y + move_y),
                PathSegment::Q(x1, y1, x, y) => {
                    PathSegment::Q(x1 + move_x, y1 + move_y, x + move_x, y + move_y)
                }
                PathSegment::C(x1, y1, x2, y2, x, y) => PathSegment::C(
                    x1 + move_x,
                    y1 + move_y,
                    x2 + move_x,
                    y2 + move_y,
                    x + move_x,
                    y + move_y,
                ),
                PathSegment::Z => PathSegment::Z,
            };
            segments.push(new_segment);
        }
        PathSegments(segments)
    }

    pub fn scale_path(&self, scale_x: f32, scale_y: f32) -> PathSegments {
        let mut segments: Vec<PathSegment> = vec![];
        for segment in self.0.iter() {
            let new_segment: PathSegment = match segment {
                PathSegment::M(x, y) => PathSegment::M(x * scale_x, y * scale_y),
                PathSegment::L(x, y) => PathSegment::L(x * scale_x, y * scale_y),
                PathSegment::Q(x1, y1, x, y) => {
                    PathSegment::Q(x1 * scale_x, y1 * scale_y, x * scale_x, y * scale_y)
                }
                PathSegment::C(x1, y1, x2, y2, x, y) => PathSegment::C(
                    x1 * scale_x,
                    y1 * scale_y,
                    x2 * scale_x,
                    y2 * scale_y,
                    x * scale_x,
                    y * scale_y,
                ),
                PathSegment::Z => PathSegment::Z,
            };
            segments.push(new_segment);
        }
        PathSegments(segments)
    }

    pub fn from_json(json: &str) -> PathSegments {
        let path: Vec<PathSegment> = serde_json::from_str(json).unwrap();
        println!("path:{:?}", path);
        PathSegments(path)
    }
}
