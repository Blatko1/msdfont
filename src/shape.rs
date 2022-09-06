use std::slice::Iter;

use crate::{math::Distance, vector::Vector2, font::BBox};

#[derive(Debug)]
pub struct Shape {
    contours: Vec<Contour>,
}

impl Shape {
    #[inline]
    pub fn new(contours: Vec<Contour>) -> Self {
        Self { contours }
    }

    /// Iterates over shape contours.
    #[inline]
    pub fn iter(&self) -> Iter<'_, Contour> {
        self.contours.iter()
    }

    // TODO
    pub fn bbox(&self) -> BBox {
        BBox { tl: Vector2::new(0.0, 1000.0), br: Vector2::new(1000.0, 000.0) }
    }
}

#[derive(Debug)]
pub struct Contour {
    segments: Vec<Segment>,
    winding: Winding,
}

impl Contour {
    pub fn new(segments: Vec<Segment>, winding: Winding) -> Self {
        Self { segments, winding }
    }

    /// Returns the [`Distance`] to the provided point.
    pub fn distance(&self, point: Vector2) -> Distance {
        self.segments
            .iter()
            .map(|segment| segment.distance(point))
            .reduce(|accum, item| {
                // To learn more about the comparison go to `SignedDistance::partial_cmp`
                if accum < item {
                    accum
                } else {
                    item
                }
            })
            .expect("No distances?? Somehow resolve this error if it happens")
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_, Segment> {
        self.segments.iter()
    }

    #[inline]
    pub fn winding(&self) -> Winding {
        self.winding
    }
}

#[derive(Debug)]
pub enum Segment {
    Line(Line),
    Quadratic(Quad),
    Cubic(Curve),
}

impl Segment {
    fn distance(&self, point: Vector2) -> Distance {
        match self {
            Segment::Line(l) => l.calculate_distance(point),
            Segment::Quadratic(q) => q.calculate_distance(point),
            Segment::Cubic(c) => todo!(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Line {
    pub from: Vector2,
    pub to: Vector2,
}

#[derive(Copy, Clone, Debug)]
pub struct Quad {
    pub from: Vector2,
    pub ctrl: Vector2,
    pub to: Vector2,
}

#[derive(Copy, Clone, Debug)]
pub struct Curve {
    pub from: Vector2,
    pub ctrl1: Vector2,
    pub ctrl2: Vector2,
    pub to: Vector2,
}

impl Line {
    pub fn new(from: Vector2, to: Vector2) -> Self {
        Self { from, to }
    }

    pub fn calculate_distance(&self, point: Vector2) -> Distance {
        crate::math::line_signed_distance(*self, point)
    }

    // TODO explain
    #[inline]
    pub fn shoelace(&self) -> f32 {
        self.from.cross(self.to)
    }
}

impl Quad {
    pub fn new(from: Vector2, ctrl: Vector2, to: Vector2) -> Self {
        Self { from, ctrl, to }
    }

    pub fn calculate_distance(&self, point: Vector2) -> Distance {
        crate::math::quad_signed_distance(*self, point)
    }

    // TODO explain
    #[inline]
    pub fn shoelace(&self) -> f32 {
        self.from.cross(self.to)
    }
}

impl Curve {
    pub fn new(from: Vector2, ctrl1: Vector2, ctrl2: Vector2, to: Vector2) -> Self {
        Self {
            from,
            ctrl1,
            ctrl2,
            to,
        }
    }

    // TODO explain
    #[inline]
    pub fn shoelace(&self) -> f32 {
        self.from.cross(self.to)
    }
}

/// Used to determine if contour is additive or subtractive.
///
/// In other words, if the winding is set to `true`, contour
/// is drawn clockwise and is additive meaning it fills the
/// surrounded area.
///
/// If the winding is set to `false` the opposite is true
/// meaning it creates cutouts.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Winding(pub bool);

impl Winding {
    /// Checks if the winding is clockwise.
    #[inline]
    pub fn is_cw(&self) -> bool {
        self.0 == true
    }

    /// Checks if the winding is counter clockwise.
    #[inline]
    pub fn is_ccw(&self) -> bool {
        !self.is_cw()
    }
}
// TODO maybe needed in future
// #[derive(Debug, Clone, Copy)]
// pub struct BoundBox {
//     /// Top left point.
//     pub tl: Vector2,
//
//     /// Bottom right point.
//     pub br: Vector2,
// }
//
// impl BoundBox {
//     const ZERO: BoundBox = BoundBox {
//         tl: Vector2::ZERO,
//         br: Vector2::ZERO,
//     };
// }
