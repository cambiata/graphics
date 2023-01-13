#[derive(Debug)]
pub enum PathSegment {
    M(f32, f32),
    L(f32, f32),
    Q(f32, f32, f32, f32),
    C(f32, f32, f32, f32, f32, f32),
    Z,
}
#[derive(Debug)]
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
}
