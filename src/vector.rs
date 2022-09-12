use std::ops::{Add, Div, Mul, Neg, Sub};

use num_traits::{Num, real::Real};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector2<N: Num + std::ops::Neg<Output = N>> {
    pub x: N,
    pub y: N,
}

impl<N: Num + std::ops::Neg<Output = N>> Vector2<N> {
    pub const ZERO_F32: Vector2<f32> = Vector2 { x: 0.0, y: 0.0 };
    pub const ZERO_I32: Vector2<i32> = Vector2 { x: 0, y: 0 };

    #[inline]
    pub fn new(x: N, y: N) -> Self {
        Vector2 { x, y }
    }

    #[inline]
    pub fn from(p: (N, N)) -> Self {
        Vector2 { x: p.0, y: p.1 }
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero()
    }

    #[inline]
    pub fn dot(self, other: Self) -> N {
        self.x * other.x + self.y * other.y
    }

    #[inline]
    pub fn neg(self) -> Self {
        -self
    }

    #[inline]
    pub fn cross(self, other: Self) -> N {
        (self.x * other.y) - (self.y * other.x)
    }
}

impl<N: Real> Vector2<N> {
    #[inline]
    pub fn magnitude2(self) -> N {
        Self::dot(self, self)
    }

    #[inline]
    pub fn magnitude(self) -> N {
        self.magnitude2().sqrt()
    }

    #[inline]
    pub fn normalize(self) -> Self {
        let mag = self.magnitude();
        assert!(!mag.is_zero(), "Math Error: Failed at normalizing 0 length vector.");
        self * (mag.recip())
    }
}

impl<N: Num + std::ops::Neg<Output = N>> Add for Vector2<N> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<N: Num + std::ops::Neg<Output = N>> Sub for Vector2<N> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<N: Num + std::ops::Neg<Output = N>> Mul for Vector2<N> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Vector2::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl<N: Num + std::ops::Neg<Output = N>> Div for Vector2<N> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Vector2::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl<N: Num + std::ops::Neg<Output = N> + Copy> Mul<N> for Vector2<N> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: N) -> Self::Output {
        Vector2::new(self.x * rhs, self.y * rhs)
    }
}

impl<N: Num + std::ops::Neg<Output = N> + Copy> Div<N> for Vector2<N> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: N) -> Self::Output {
        Vector2::new(self.x / rhs, self.y / rhs)
    }
}

impl<N: Num + std::ops::Neg<Output = N>> Neg for Vector2<N> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Vector2::new(-self.x, -self.y)
    }
}

impl Mul<Vector2<f32>> for f32 {
    type Output = Vector2<f32>;

    #[inline]
    fn mul(self, rhs: Vector2<f32>) -> Self::Output {
        Vector2::new(self * rhs.x, self * rhs.y)
    }
}