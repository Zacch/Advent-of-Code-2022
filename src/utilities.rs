
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn origin() -> Point { Point{ x: 0, y: 0} }
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
    pub fn up(&self) -> Point { Point::new(self.x, self.y + 1) }
    pub fn down(&self) -> Point {
        Point::new(self.x, self.y - 1)
    }
    pub fn left(&self) -> Point {
        Point::new(self.x - 1, self.y)
    }
    pub fn right(&self) -> Point {
        Point::new(self.x + 1, self.y)
    }

    pub fn n(&self) -> Point { Point::new(self.x, self.y + 1) }
    pub fn s(&self) -> Point { Point::new(self.x, self.y - 1) }
    pub fn w(&self) -> Point { Point::new(self.x - 1, self.y) }
    pub fn e(&self) -> Point {
        Point::new(self.x + 1, self.y)
    }

    pub fn ne(&self) -> Point { Point::new(self.x + 1, self.y + 1) }
    pub fn se(&self) -> Point { Point::new(self.x + 1, self.y - 1) }
    pub fn sw(&self) -> Point { Point::new(self.x - 1, self.y - 1) }
    pub fn nw(&self) -> Point { Point::new(self.x - 1, self.y + 1) }

    pub(crate) fn manhattan_distance(&self, p: &Point) -> i32 {
        i32::abs(self.x - p.x) + i32::abs(self.y - p.y)
    }
}

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
