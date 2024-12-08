use std::ops::{Add, Mul, Sub};

#[derive(Debug, Hash, Clone, Copy, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Mul<i32> for Point {
    type Output = Point;

    fn mul(self, scalar: i32) -> Point {
        Point {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}
