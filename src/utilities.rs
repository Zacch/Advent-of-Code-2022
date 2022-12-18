use crate::points::Point;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub bottom: i32,
    pub right: i32,
}

impl Rect {
    pub fn contains(&self, p: Point) -> bool {
        p.x >= self.left && p.x < self.right && p.y >= self.bottom && p.y < self.top
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct IntRange {
    pub start: i32,
    pub end: i32,
}

/// An inclusive range of i32s, i. e., `end` is included in the range
impl IntRange {
    pub fn new(start: i32, end: i32) -> IntRange {
        IntRange { start, end }
    }
    pub fn touches(&self, other: &IntRange) -> bool {
        self.start <= other.end + 1 && self.end >= other.start - 1
    }
}
