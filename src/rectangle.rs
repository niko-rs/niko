use crate::Point;

#[derive(Debug, Clone)]
pub struct Rectangle {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Rectangle {
    /// creates a new rectangle
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// returns if the other rectangle overlaps with this one
    pub fn overlaps_rectangle(&self, other: &Self) -> bool {
        self.x < (other.x + other.width) &&
        (self.x + self.width) > other.x &&

        self.y < (other.y + other.height) &&
        (self.y + self.height) > other.y
    }

    /// returns if the other rectangle is fully contained by this one
    pub fn contains_rectangle(&self, other: &Self) -> bool {
        other.x >= self.x &&
        other.x + other.width <= self.x + self.width &&
        other.y >= self.y &&
        other.y + other.height <= self.y + self.height
    }

    /// returns if the point is inside this rectangle
    pub fn contains_point(&self, point: Point) -> bool {
        point.x >= self.x && point.x <= (self.x + self.width) &&
        point.y >= self.y && point.y <= (self.y + self.height)
    }

    /// creates a new rectangle and inflates it in all directions
    pub fn inflate(&self, horizontal: i32, vertical: i32) -> Self {
        Self::new(
            self.x - horizontal,
            self.y - vertical,
            self.width + horizontal,
            self.height + vertical,
        )
    }

    /// creates a new rectangle and moves it
    pub fn translate(&self, x: i32, y: i32) -> Self {
        Self::new(
            self.x + x,
            self.y + y,
            self.width,
            self.height,
        )
    }

    /// creates a new rectangle that contains both rectangles
    pub fn union(&self, other: Self) -> Self {
        let x = self.x.min(other.x);
        let y = self.y.min(other.y);
        let width = (self.x + self.width).max(other.x + other.width) - x;
        let height = (self.y + self.height).max(other.y + other.height) - y;
        Self::new(
            x,
            y,
            width,
            height,
        )
    }

    pub(crate) fn to_rendering_position(&self, bounds: &Self) -> (f32, f32, f32, f32) {
        // TODO assume bounds x and y is 0 for now

        let left = self.x as f32 / bounds.width as f32;
        let right = (self.x + self.width) as f32 / bounds.width as f32;

        
        let top = self.y as f32 / bounds.height as f32;
        let bottom = (self.y + self.height) as f32 / bounds.height as f32;

        (left, right, top, bottom)
    }
}

impl Into<(i32, i32, i32, i32)> for Rectangle {
    fn into(self) -> (i32, i32, i32, i32) {
        (self.x, self.y, self.width, self.height)
    }
}

impl From<(i32, i32, i32, i32)> for Rectangle {
    fn from(values: (i32, i32, i32, i32)) -> Self {
        Self::new(values.0, values.1, values.2, values.3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        let a = Rectangle::new(0, 0, 3, 3);
        let b = Rectangle::new(1, 1, 1, 1);

        assert!(a.overlaps_rectangle(&b));
        assert!(b.overlaps_rectangle(&a));

        assert!(a.contains_rectangle(&b));
        assert!(!b.contains_rectangle(&a));
    }
}
