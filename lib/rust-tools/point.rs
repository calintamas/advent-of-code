use std::ops::{Add, AddAssign, Mul, Sub};

#[derive(Debug, Hash, Clone, Copy, Eq)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
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

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Mul<isize> for Point {
    type Output = Point;

    fn mul(self, scalar: isize) -> Point {
        Point {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}
