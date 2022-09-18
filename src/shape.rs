use std::slice::Iter;

use rusttype::Scale;

use crate::{font::BBox, math::Distance, vector::Vector2};

#[derive(Debug)]
pub struct Shape {
    pub contours: Vec<Contour>,
}

impl Shape {
    #[inline]
    pub fn new(contours: Vec<Contour>) -> Self {
        Self { contours }
    }

    // TODO maybe change to [`Scale`].
    // pub fn scale(&mut self, scale: f32) {
    //     for contour in self.contours.iter_mut() {
    //         for segment in contour.segments.iter_mut() {
    //             match segment {
    //                 Segment::Line(l) => l.rescale(scale),
    //                 Segment::Quadratic(q) => q.rescale(scale),
    //                 Segment::Cubic(c) => c.rescale(scale),
    //             }
    //         }
    //     }
    // }

    /// Returns a bounding box which is created paying attention to
    /// line and curve points instead of their bodies.
    /// TODO: maybe not needed
    pub fn bbox(&self) -> BBox {
        let mut x_iter = Vec::new();
        let mut y_iter = Vec::new();
        for contour in self.contours.iter() {
            for seg in contour.iter() {
                match seg {
                    Segment::Line(l) => {
                        x_iter.push(l.from.x);
                        x_iter.push(l.to.x);

                        y_iter.push(l.from.y);
                        y_iter.push(l.to.y);
                    }
                    Segment::Quadratic(q) => {
                        x_iter.push(q.from.x);
                        x_iter.push(q.ctrl.x);
                        x_iter.push(q.to.x);

                        y_iter.push(q.from.y);
                        y_iter.push(q.ctrl.y);
                        y_iter.push(q.to.y);
                    }
                    Segment::Cubic(c) => {
                        x_iter.push(c.from.x);
                        x_iter.push(c.ctrl1.x);
                        x_iter.push(c.ctrl2.x);
                        x_iter.push(c.to.x);

                        y_iter.push(c.from.y);
                        y_iter.push(c.ctrl1.y);
                        y_iter.push(c.ctrl2.y);
                        y_iter.push(c.to.y);
                    }
                }
            }
        }
        // Highest y point of the shape.
        let top = *y_iter
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .expect("No segments? Impossible!") as i32;
        let bottom = *y_iter
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .expect("No segments? Impossible!") as i32;
        let left = *x_iter
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .expect("No segments? Impossible!") as i32;
        let right = *x_iter
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .expect("No segments? Impossible!") as i32;

        BBox {
            tl: Vector2::new(left, top),
            br: Vector2::new(right, bottom),
        }
    }
}

#[derive(Debug)]
pub struct Contour {
    pub segments: Vec<Segment>,
    winding: Winding,
}

impl Contour {
    pub fn new(segments: Vec<Segment>, winding: Winding) -> Self {
        Self { segments, winding }
    }

    /// Returns the [`Distance`] to the provided point.
    pub fn distance(&self, point: Vector2<f32>) -> Distance {
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
    fn distance(&self, point: Vector2<f32>) -> Distance {
        match self {
            Segment::Line(l) => l.calculate_distance(point),
            Segment::Quadratic(q) => q.calculate_distance(point),
            Segment::Cubic(c) => c.calculate_distance(point),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Line {
    pub from: Vector2<f32>,
    pub to: Vector2<f32>,
}

#[derive(Copy, Clone, Debug)]
pub struct Quad {
    pub from: Vector2<f32>,
    pub ctrl: Vector2<f32>,
    pub to: Vector2<f32>,
}

#[derive(Copy, Clone, Debug)]
pub struct Curve {
    pub from: Vector2<f32>,
    pub ctrl1: Vector2<f32>,
    pub ctrl2: Vector2<f32>,
    pub to: Vector2<f32>,
}

impl Line {
    pub fn new(from: Vector2<f32>, to: Vector2<f32>) -> Self {
        Self { from, to }
    }

    // TODO maybe
    pub fn rescale(&mut self, scale: Scale) {
        self.from *= scale;
        self.to *= scale;
    }

    pub fn calculate_distance(&self, point: Vector2<f32>) -> Distance {
        crate::math::line_signed_distance(*self, point)
    }

    // TODO explain
    #[inline]
    pub fn shoelace(&self) -> f32 {
        self.from.cross(self.to)
    }
}

impl Quad {
    pub fn new(
        from: Vector2<f32>,
        ctrl: Vector2<f32>,
        to: Vector2<f32>,
    ) -> Self {
        Self { from, ctrl, to }
    }

    pub fn rescale(&mut self, scale: Scale) {
        self.from *= scale;
        self.ctrl *= scale;
        self.to *= scale;
    }

    pub fn calculate_distance(&self, point: Vector2<f32>) -> Distance {
        crate::math::quad_signed_distance(*self, point)
    }

    // TODO explain
    #[inline]
    pub fn shoelace(&self) -> f32 {
        self.from.cross(self.to)
    }
}

impl Curve {
    pub fn new(
        from: Vector2<f32>,
        ctrl1: Vector2<f32>,
        ctrl2: Vector2<f32>,
        to: Vector2<f32>,
    ) -> Self {
        Self {
            from,
            ctrl1,
            ctrl2,
            to,
        }
    }

    pub fn rescale(&mut self, scale: Scale) {
        self.from *= scale;
        self.ctrl1 *= scale;
        self.ctrl2 *= scale;
        self.to *= scale;
    }

    #[inline]
    pub fn calculate_distance(&self, point: Vector2<f32>) -> Distance {
        unimplemented!()
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
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Winding(pub bool);

impl Winding {
    /// Checks if the winding is clockwise.
    #[inline]
    pub fn is_cw(&self) -> bool {
        self.0
    }

    /// Checks if the winding is counter clockwise.
    #[inline]
    pub fn is_ccw(&self) -> bool {
        !self.is_cw()
    }
}
