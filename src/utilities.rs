#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn origin() -> Point { Point{ x: 0, y: 0} }
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
    pub fn up(&self) -> Point { Self::new(self.x, self.y + 1) }
    pub fn down(&self) -> Point {
        Point::new(self.x, self.y - 1)
    }
    pub fn left(&self) -> Point {
        Point::new(self.x - 1, self.y)
    }
    pub fn right(&self) -> Point {
        Point::new(self.x + 1, self.y)
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
