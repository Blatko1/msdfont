use hashbrown::HashMap;

use crate::shape::{Curve, Line, Quad, Segment, Contour, ContourID};

#[derive(Debug)]
pub struct OverlapData {
    overlaps_map: HashMap<ContourID, Vec<ContourID>>
}

impl OverlapData {
    // TODO find a way to improve efficiency
    pub fn from_contours(contours: &Vec<Contour>) -> Option<Self> {
        let mut data = HashMap::new();
        for contour in contours.iter() {
            data.insert(contour.id(), Vec::new());
        }

        let len = contours.len();
        // TODO explain
        // Compare each contour with another avoiding duplicate comparisons.
        // Current contour
        for (index, contour) in (&contours[0..len - 1]).iter().enumerate() {
            // Compare with contour
            for other in contours.iter().skip(index + 1) {
                if contour.overlaps(other) {
                    data.get_mut(&contour.id()).unwrap().push(other.id());
                    data.get_mut(&other.id()).unwrap().push(contour.id());
                }
            }
        }

        if data.is_empty() {
            None
        } else {
        data.retain(|_, overlaps| {
            if overlaps.is_empty() {
                false
            } else {
                true
            }
        });
        Some(Self {
            overlaps_map: data,
        })
    }
        
    }
}

trait Intersections {
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

impl Contour {
    fn overlaps(&self, other: &Self) -> bool {
        // TODO explain
        // If an intersection is found immediately return `true`.
        for segment in self.iter() {
            for other in other.iter() {
                if segment.intersects_with(other) {
                    return true;
                }
            }
        }
        false
    }
}

impl Segment {
    fn intersects_with(&self, other: &Self) -> bool {
        match self {
            Segment::Line(line) => line.intersects_with(other),
            Segment::Quadratic(quad) => quad.intersects_with(other),
            Segment::Cubic(curve) => curve.intersects_with(other),
        }
    }
}

impl Intersections for Line {
    #[inline]
    fn intersects_line(&self, other: &Line) -> bool {
        crate::math::line_line_intersect(self.from, self.to, other.from, other.to)
    }

    #[inline]
    fn intersects_quad(&self, other: &Quad) -> bool {
        crate::math::quad_line_intersect(
            other.from, other.ctrl, other.to, self.from, self.to,
        )
    }

    #[inline]
    fn intersects_curve(&self, other: &Curve) -> bool {
        crate::math::cubic_line_intersect(
            other.from,
            other.ctrl1,
            other.ctrl2,
            other.to,
            self.from,
            self.to,
        )
    }
}

impl Intersections for Quad {
    #[inline]
    fn intersects_line(&self, other: &Line) -> bool {
        crate::math::quad_line_intersect(
            self.from, self.ctrl, self.to, other.from, other.to,
        )
    }

    #[inline]
    fn intersects_quad(&self, other: &Quad) -> bool {
        crate::math::quad_quad_intersect(
            self.from, self.ctrl, self.to, other.from, other.ctrl, self.to,
        )
    }

    #[inline]
    fn intersects_curve(&self, other: &Curve) -> bool {
        crate::math::cubic_quad_intersect(
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

impl Intersections for Curve {
    #[inline]
    fn intersects_line(&self, other: &Line) -> bool {
        crate::math::cubic_line_intersect(
            self.from, self.ctrl1, self.ctrl2, self.to, other.from, other.to,
        )
    }

    #[inline]
    fn intersects_quad(&self, other: &Quad) -> bool {
        crate::math::cubic_quad_intersect(
            self.from, self.ctrl1, self.ctrl2, self.to, other.from, other.ctrl,
            other.to,
        )
    }

    #[inline]
    fn intersects_curve(&self, other: &Curve) -> bool {
        crate::math::cubic_cubic_intersect(
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
