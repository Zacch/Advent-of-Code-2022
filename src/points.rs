use std::ops;

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

    pub fn manhattan_distance(&self, p: &Point) -> i32 {
        i32::abs(self.x - p.x) + i32::abs(self.y - p.y)
    }
}

impl ops::Add<&Point> for Point {
    type Output = Point;
    fn add(self, rhs: &Point) -> Point { Point::new(self.x + rhs.x, self.y + rhs.y) }
}
