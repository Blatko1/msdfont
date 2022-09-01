use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub const ZERO: Vector2 = Vector2 { x: 0.0, y: 0.0 };

    #[inline]
    pub fn new(x: f32, y: f32) -> Self {
        Vector2 { x, y }
    }

    #[inline]
    pub fn from(p: (f32, f32)) -> Self {
        Vector2 { x: p.0, y: p.1 }
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == Vector2::ZERO
    }

    #[inline]
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    #[inline]
    pub fn magnitude2(self) -> f32 {
        Self::dot(self, self)
    }

    #[inline]
    pub fn magnitude(self) -> f32 {
        self.magnitude2().sqrt()
    }

    #[inline]
    pub fn neg(self) -> Self {
        -self
    }

    #[inline]
    pub fn normalize(self) -> Self {
        self * (1.0 / self.magnitude())
    }

    #[inline]
    pub fn cross(self, other: Self) -> f32 {
        (self.x * other.y) - (self.y * other.x)
    }
}

impl Add for Vector2 {
    type Output = Vector2;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Vector2 {
    type Output = Vector2;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul for Vector2 {
    type Output = Vector2;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Vector2::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl Div for Vector2 {
    type Output = Vector2;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Vector2::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl Mul<f32> for Vector2 {
    type Output = Vector2;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Vector2::new(self.x * rhs, self.y * rhs)
    }
}

impl Div<f32> for Vector2 {
    type Output = Vector2;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Vector2::new(self.x / rhs, self.y / rhs)
    }
}

impl Mul<Vector2> for f32 {
    type Output = Vector2;

    #[inline]
    fn mul(self, rhs: Vector2) -> Self::Output {
        Vector2::new(self * rhs.x, self * rhs.y)
    }
}

impl Neg for Vector2 {
    type Output = Vector2;

    #[inline]
    fn neg(self) -> Self::Output {
        Vector2::new(-self.x, -self.y)
    }
}
