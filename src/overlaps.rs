use crate::shape::{Line, Quad, Curve, Shape, Contour, Segment};

pub trait Intersectable {
    // TODO add docs for each
    #[inline]
    fn intersects_with(&self, other: &Segment) -> bool {
        match other {
            Segment::Line(line) => self.intersects_line(line),
            Segment::Quadratic(quad) => self.intersects_quad(quad),
            Segment::Cubic(curve) => self.intersects_curve(curve),
        }
    }

    fn intersects_line(&self, other: &Line) -> bool;

    fn intersects_quad(&self, other: &Quad) -> bool;

    fn intersects_curve(&self, other: &Curve) -> bool;
}

impl Intersectable for Line {
    #[inline]
    fn intersects_line(&self, other: &Line) -> bool {
        crate::math::lines_intersect(self.from, self.to, other.from, other.to)
    }

    #[inline]
    fn intersects_quad(&self, other: &Quad) -> bool {
        todo!()
    }

    #[inline]
    fn intersects_curve(&self, other: &Curve) -> bool {
        todo!()
    }
}

impl Intersectable for Quad {
    #[inline]
    fn intersects_line(&self, other: &Line) -> bool {
        todo!()
    }

    #[inline]
    fn intersects_quad(&self, other: &Quad) -> bool {
        todo!()
    }

    #[inline]
    fn intersects_curve(&self, other: &Curve) -> bool {
        todo!()
    }
}

impl Intersectable for Curve {
    #[inline]
    fn intersects_line(&self, other: &Line) -> bool {
        todo!()
    }

    #[inline]
    fn intersects_quad(&self, other: &Quad) -> bool {
        todo!()
    }

    #[inline]
    fn intersects_curve(&self, other: &Curve) -> bool {
        todo!()
    }
}