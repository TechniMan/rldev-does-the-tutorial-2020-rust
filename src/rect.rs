pub struct Rect {
    pub x1 : i32,
    pub x2 : i32,
    pub y1 : i32,
    pub y2 : i32
}
impl Rect {
    pub fn new(x_pos: i32, y_pos: i32, width: i32, height: i32) -> Rect {
        Rect { x1: x_pos, y1: y_pos, x2: x_pos + width, y2: y_pos + height }
    }

    /// Returns whether this `Rect` intersects with `other`
    pub fn intersect(&self, other: &Rect) -> bool {
        self.x1 <= other.x2 &&
            self.x2 >= other.x1 &&
            self.y1 <= other.y2 &&
            self.y2 >= other.y1
    }

    /// Centre point of this `Rect`
    pub fn centre(&self) -> (i32, i32) {
        ((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }
}
