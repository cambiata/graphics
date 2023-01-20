use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rectangle(pub f32, pub f32, pub f32, pub f32);

impl Rectangle {
    pub fn overlap_x(&self, right: &Rectangle) -> f32 {
        if self.1 + self.3 <= right.1 {
            return 0.;
        } else if (self.1 >= right.1 + right.3) {
            return 0.;
        } else if (right.0 >= self.0 + self.2) {
            return 0.;
        } else {
            return self.0 + self.3 - right.0;
        }
    }
}

pub struct Rectangles {
    items: Vec<Rectangle>,
}

impl Rectangles {
    pub fn new(items: Vec<Rectangle>) -> Self {
        Self { items }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Rectangle> {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Rectangle> {
        self.into_iter()
    }

    fn overlap_x(&self, right: &Rectangles) -> f32 {
        let mut max_overlap = f32::MIN;
        for left_r in self {
            for right_r in right {
                max_overlap = max_overlap.max(left_r.overlap_x(right_r));
            }
        }
        max_overlap
    }

    fn move_xy(&self, x: f32, y: f32) -> Rectangles {
        let new_items: Vec<Rectangle> = self
            .items
            .iter()
            .map(|r| Rectangle(r.0 + x, r.1 + y, r.2, r.3))
            .collect();

        Rectangles::new(new_items)
    }
}

impl<'a> IntoIterator for &'a Rectangles {
    type Item = &'a Rectangle;

    type IntoIter = std::slice::Iter<'a, Rectangle>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}

impl<'a> IntoIterator for &'a mut Rectangles {
    type Item = &'a mut Rectangle;

    type IntoIter = std::slice::IterMut<'a, Rectangle>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let r_left = Rectangle(0., 0., 10., 10.);
        let r_right = Rectangle(0., 9., 10., 10.);
        let overlap_x = r_left.overlap_x(&r_right);
        println!("overlap_x:{:?}", overlap_x);
    }
    #[test]
    fn example2() {
        let left = Rectangles::new(vec![
            Rectangle(0., 0., 10., 10.),
            Rectangle(10., 0., 10., 10.),
        ]);
        let right = Rectangles::new(vec![
            Rectangle(0., 0., 10., 10.),
            Rectangle(0., 10., 10., 10.),
        ])
        .move_xy(19., -20.);

        let overlap_x = left.overlap_x(&right);
        println!("overlap_x:{:?}", overlap_x);
    }
}
