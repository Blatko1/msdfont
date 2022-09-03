use crate::{
    contour::{Curve, Line, Quad, Segment},
    vector::Vector2,
};

impl Segment {
    fn get_intersections(&self, other: &Self) -> Vec<Vector2> {
        match self {
            Segment::Line(line) => line.get_intersections(other),
            Segment::Quadratic(quad) => quad.get_intersections(other),
            Segment::Cubic(curve) => curve.get_intersections(other),
        }
    }
}

impl Line {
    #[inline]
    fn get_intersections(&self, other: &Segment) -> Vec<Vector2> {
        let inters = match other {
            Segment::Line(line) => vec![self.intersects_line(line)],
            Segment::Quadratic(quad) => self.intersects_quad(quad).to_vec(),
            Segment::Cubic(curve) => self.intersects_curve(curve).to_vec(),
        };
        let intersections = inters.iter().flatten().map(|&inter| inter).collect();
        intersections
    }

    #[inline]
    fn intersects_line(&self, other: &Line) -> Option<Vector2> {
        crate::math::line_line_intersection(self.from, self.to, other.from, other.to)
    }

    #[inline]
    fn intersects_quad(&self, other: &Quad) -> [Option<Vector2>; 2] {
        crate::math::quad_line_intersection(
            other.from, other.ctrl, other.to, self.from, self.to,
        )
    }

    #[inline]
    fn intersects_curve(&self, other: &Curve) -> [Option<Vector2>; 3] {
        crate::math::cubic_line_intersection(
            other.from,
            other.ctrl1,
            other.ctrl2,
            other.to,
            self.from,
            self.to,
        )
    }
}

impl Quad {
    #[inline]
    fn get_intersections(&self, other: &Segment) -> Vec<Vector2> {
        let inters = match other {
            Segment::Line(line) => self.intersects_line(line).to_vec(),
            Segment::Quadratic(quad) => self.intersects_quad(quad).to_vec(),
            Segment::Cubic(curve) => self.intersects_curve(curve).to_vec(),
        };
        let intersections = inters.iter().flatten().map(|&inter| inter).collect();
        intersections
    }

    #[inline]
    fn intersects_line(&self, other: &Line) -> [Option<Vector2>; 2] {
        other.intersects_quad(self)
    }

    #[inline]
    fn intersects_quad(&self, other: &Quad) -> [Option<Vector2>; 4] {
        crate::math::quad_quad_intersection(
            self.from, self.ctrl, self.to, other.from, other.ctrl, self.to,
        )
    }

    #[inline]
    fn intersects_curve(&self, other: &Curve) -> [Option<Vector2>; 6] {
        crate::math::cubic_quad_intersection(
            other.from,
            other.ctrl1,
            other.ctrl2,
            other.to,
            self.from,
            self.ctrl,
            self.to,
        )
    }
}

impl Curve {
    #[inline]
    fn get_intersections(&self, other: &Segment) -> Vec<Vector2> {
        let inters = match other {
            Segment::Line(line) => self.intersects_line(line).to_vec(),
            Segment::Quadratic(quad) => self.intersects_quad(quad).to_vec(),
            Segment::Cubic(curve) => self.intersects_curve(curve).to_vec(),
        };
        let intersections = inters.iter().flatten().map(|&inter| inter).collect();
        intersections
    }

    #[inline]
    fn intersects_line(&self, other: &Line) -> [Option<Vector2>; 3] {
        other.intersects_curve(self)
    }

    #[inline]
    fn intersects_quad(&self, other: &Quad) -> [Option<Vector2>; 6] {
        other.intersects_curve(self)
    }

    #[inline]
    fn intersects_curve(&self, other: &Curve) -> [Option<Vector2>; 9] {
        crate::math::cubic_cubic_intersection(
            self.from,
            self.ctrl1,
            self.ctrl2,
            self.to,
            other.from,
            other.ctrl1,
            other.ctrl2,
            other.to,
        )
    }
}
