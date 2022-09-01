use hashbrown::HashMap;

use crate::{
    contour::{Contour, ContourID, Curve, Line, Quad, Segment},
    vector::Vector2,
};

#[derive(Debug)]
pub struct OverlapData {
    /// Used for checking if a specific contour has intersections and
    /// finding the intersection points.
    ///
    /// - `key` represents the contour
    /// - `value` represents all contours and intersection points
    /// which the key intersects
    intersections: HashMap<ContourID, Vec<IntersectionsData>>,

    /// Used in rare cases when a contour is placed inside the surrounding contour
    /// with the same winding.
    /// When that happens the surrounded contour should be ignored.
    /// Represents all contour that are being surrounded by the same winding contour.
    surrounded: Vec<ContourID>,
}

impl OverlapData {
    // TODO find a way to improve efficiency
    pub fn from_contours(contours: &Vec<Contour>) -> Self {
        let mut data = HashMap::new();
        let surrounded = Vec::new();
        contours.iter().for_each(|contour| {
            data.insert_unique_unchecked(contour.id(), Vec::new());
        });

        let len = contours.len();
        // TODO explain
        // Compare each contour with another avoiding duplicate comparisons.
        // Current contour
        for (index, contour) in (&contours[0..len - 1]).iter().enumerate() {
            // Compare with
            for other in contours.iter().skip(index + 1) {
                let intersections = contour.get_intersections(other);
                if intersections.is_empty() {
                    // If there are no intersections check if one surrounds the other or vice versa.
                    if let Some(surrounded) =
                        Contour::get_surrounded_contour(contour, other)
                    {
                    }
                } else {
                    data.get_mut(&contour.id()).unwrap().push(
                        IntersectionsData::new(other.id(), intersections.clone()),
                    );
                    data.get_mut(&other.id())
                        .unwrap()
                        .push(IntersectionsData::new(contour.id(), intersections));
                }
            }
        }
        data.retain(|_, overlaps| if overlaps.is_empty() { false } else { true });

        Self {
            intersections: data,
            surrounded,
        }
    }

    pub fn are_overlapping(&self, id1: ContourID, id2: ContourID) -> bool {
        self.intersections
            .get(&id1)
            .unwrap()
            .iter()
            .any(|elem| elem.id == id2)
    }

    pub fn has_intersections(&self, id: ContourID) -> bool {
        self.intersections.contains_key(&id)
    }

    pub fn is_empty(&self) -> bool {
        self.intersections.is_empty()
    }
}

/// Represents a contour and all intersection points with another contour.
#[derive(Debug)]
pub struct IntersectionsData {
    id: ContourID,
    intersections: Vec<Vector2>,
}

impl IntersectionsData {
    #[inline]
    fn new(id: ContourID, intersections: Vec<Vector2>) -> Self {
        Self { id, intersections }
    }
}

impl Contour {
    fn get_intersections(&self, other: &Self) -> Vec<Vector2> {
        let mut intersections = Vec::new();
        // TODO explain
        for segment in self.iter() {
            for other in other.iter() {
                let inters = segment.get_intersections(other);
                intersections.extend(inters.iter());
            }
        }
        intersections
    }

    fn surrounds(&self, other: &Contour) -> bool {
        let outer = self.bbox();
        let inner = other.bbox();

        // Checks if `outer` bounding box surrounds `inner` bounding box.
        if outer.tl.x < inner.tl.x {
            if outer.br.x > inner.br.x {
                if outer.tl.y > inner.tl.y {
                    if outer.br.y < inner.br.y {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn get_surrounded_contour(
        contour1: &Contour,
        contour2: &Contour,
    ) -> Option<ContourID> {
        if contour1.surrounds(contour2) {
            return Some(contour2.id());
        } else if contour2.surrounds(contour1) {
            return Some(contour1.id());
        }
        None
    }
}

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
