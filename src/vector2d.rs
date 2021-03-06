
// TODO: maybe use some other (2D) vector create that already exists...

use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, Div};

// TODO: add mul_assign and div_assign

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector2D {
    pub x: f64,
    pub y: f64,
}

impl Add for Vector2D {
    type Output = Vector2D;

    fn add(self, rhs: Vector2D) -> Vector2D {
        Vector2D { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl AddAssign for Vector2D {
    fn add_assign(&mut self, rhs: Vector2D) {
        *self = Vector2D { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Sub for Vector2D {
    type Output = Vector2D;

    fn sub(self, rhs: Vector2D) -> Vector2D {
        Vector2D { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl SubAssign for Vector2D {
    fn sub_assign(&mut self, rhs: Vector2D) {
        *self = Vector2D { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl Mul<f64> for Vector2D {
    type Output = Vector2D;

    fn mul(self, rhs: f64) -> Vector2D {
        Vector2D { x: self.x * rhs, y: self.y * rhs }
    }
}

impl Div<f64> for Vector2D {
    type Output = Vector2D;

    fn div(self, rhs: f64) -> Vector2D {
        Vector2D { x: self.x / rhs, y: self.y / rhs }
    }
}
