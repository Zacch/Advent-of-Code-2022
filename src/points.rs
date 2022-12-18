use std::fmt::{Display, Formatter};
use std::ops;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn origin() -> Point { Point{ x: 0, y: 0} }
    pub fn new(x: i32, y: i32) -> Point { Point { x, y } }
    pub fn up(&self) -> Point { Point::new(self.x, self.y + 1) }
    pub fn down(&self) -> Point { Point::new(self.x, self.y - 1) }
    pub fn left(&self) -> Point { Point::new(self.x - 1, self.y) }
    pub fn right(&self) -> Point { Point::new(self.x + 1, self.y) }

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

//----------------------------------------------------------------------------

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Point3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}


impl Point3 {
    pub fn origin() -> Point3 { Point3{ x: 0, y: 0, z: 0 } }
    pub fn new(x: i32, y: i32, z: i32) -> Point3 { Point3 { x, y, z } }
    pub fn from_vector(v: Vec<i32>) -> Point3 { Point3::new(v[0], v[1], v[2]) }

    pub fn up(&self) -> Point3 { Point3::new(self.x, self.y + 1, self.z) }
    pub fn down(&self) -> Point3 { Point3::new(self.x, self.y - 1, self.z) }
    pub fn left(&self) -> Point3 { Point3::new(self.x - 1, self.y, self.z) }
    pub fn right(&self) -> Point3 { Point3::new(self.x + 1, self.y, self.z) }
    pub fn forward(&self) -> Point3 { Point3::new(self.x, self.y, self.z + 1) }
    pub fn back(&self) -> Point3 { Point3::new(self.x, self.y, self.z - 1) }

    pub fn go(&self, direction: Direction) -> Point3 {
        match direction {
            Direction::Up => { self.up() }
            Direction::Down => { self.down() }
            Direction::Left => { self.left() }
            Direction::Right => { self.right() }
            Direction::Forward => { self.forward() }
            Direction::Back => { self.back() }
        }
    }

    pub fn manhattan_distance(&self, p: &Point3) -> i32 {
        i32::abs(self.x - p.x) + i32::abs(self.y - p.y) + i32::abs(self.z - p.z)
    }
}

impl Display for Point3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{},{}]", self.x, self.y, self.z)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Direction { Up, Down, Left, Right, Forward, Back }

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct CubeFace {
    pub cube: Point3,
    pub direction: Direction,
}

impl CubeFace {
    pub fn new(cube: Point3, direction: Direction) -> CubeFace { CubeFace { cube, direction } }
}
