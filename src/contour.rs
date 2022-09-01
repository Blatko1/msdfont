use std::slice::Iter;

use crate::{math::SignedDistance, vector::Vector2};

// TODO implement Form<> Into<> and some functions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ContourID(pub u16);

#[derive(Debug)]
pub struct Contour {
    id: ContourID,
    segments: Vec<Segment>,
    winding: Winding,
    bound_box: BoundBox,
}

impl Contour {
    pub fn new(
        id: ContourID,
        segments: Vec<Segment>,
        winding: Winding,
        bound_box: BoundBox,
    ) -> Self {
        Self {
            id,
            segments,
            winding,
            bound_box,
        }
    }

    /// Returns the contour data with the signed distance to the provided point.
    pub fn get_data(&self, point: Vector2) -> ContourData {
        let mut shortest_dist = SignedDistance::MAX;

        for segment in &self.segments {
            let dist = segment.distance(point);

            // To learn more about the comparison go to `SignedDistance::partial_cmp`
            if dist < shortest_dist {
                shortest_dist = dist;
            }
        }
        ContourData {
            id: self.id,
            distance: shortest_dist,
            winding: self.winding,
        }
    }

    #[inline]
    pub fn id(&self) -> ContourID {
        self.id
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_, Segment> {
        self.segments.iter()
    }

    #[inline]
    pub fn winding(&self) -> Winding {
        self.winding
    }

    #[inline]
    pub fn bbox(&self) -> BoundBox {
        self.bound_box
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BoundBox {
    /// Top left point.
    pub tl: Vector2,

    /// Bottom right point.
    pub br: Vector2,
}

impl BoundBox {
    const ZERO: BoundBox = BoundBox {
        tl: Vector2::ZERO,
        br: Vector2::ZERO,
    };
}

#[derive(Debug)]
pub struct ContourData {
    pub id: ContourID,
    pub distance: SignedDistance,
    pub winding: Winding,
}

impl ContourData {
    /// Checks if the winding is clockwise.
    #[inline]
    pub fn is_cw(&self) -> bool {
        self.winding.is_cw()
    }

    /// Checks if the winding is counter clockwise.
    #[inline]
    pub fn is_ccw(&self) -> bool {
        self.winding.is_ccw()
    }

    /// Checks if this contour surrounds the pixel which has the stored distance.
    #[inline]
    pub fn is_surrounding(&self) -> bool {
        (self.distance.is_sign_positive() && self.is_cw())
            || (self.distance.is_sign_negative() && self.is_ccw())
    }
}

// TODO create struct which holds distance to segments used for
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ContourSignedDistance {
    pub distance: SignedDistance,
    pub contour_id: ContourID,
    pub contour_winding: Winding,
}

#[derive(Debug)]
pub enum Segment {
    Line(Line),
    Quadratic(Quad),
    Cubic(Curve),
}

impl Segment {
    fn distance(&self, point: Vector2) -> SignedDistance {
        match self {
            Segment::Line(l) => l.calculate_distance(point),
            Segment::Quadratic(q) => q.calculate_distance(point),
            Segment::Cubic(c) => todo!(),
        }
    }

    pub fn highest_point(&self) -> Vector2 {
        // TODO maybe implement a trait for extreme points
        match self {
            Segment::Line(l) => l.highest_point(),
            Segment::Quadratic(q) => q.highest_point(),
            Segment::Cubic(c) => c.highest_point(),
        }
    }

    pub fn lowest_point(&self) -> Vector2 {
        match self {
            Segment::Line(l) => l.lowest_point(),
            Segment::Quadratic(q) => q.lowest_point(),
            Segment::Cubic(c) => c.lowest_point(),
        }
    }

    pub fn leftmost_point(&self) -> Vector2 {
        match self {
            Segment::Line(l) => l.leftmost_point(),
            Segment::Quadratic(q) => q.leftmost_point(),
            Segment::Cubic(c) => c.leftmost_point(),
        }
    }

    pub fn rightmost_point(&self) -> Vector2 {
        match self {
            Segment::Line(l) => l.rightmost_point(),
            Segment::Quadratic(q) => q.rightmost_point(),
            Segment::Cubic(c) => c.rightmost_point(),
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

    pub fn calculate_distance(&self, point: Vector2) -> SignedDistance {
        crate::math::signed_distance_from_line(*self, point)
    }

    // TODO explain
    #[inline]
    pub fn shoelace(&self) -> f32 {
        self.from.cross(self.to)
    }

    #[inline]
    fn highest_point(&self) -> Vector2 {
        if self.from.y > self.to.y {
            self.from
        } else {
            self.to
        }
    }

    #[inline]
    fn lowest_point(&self) -> Vector2 {
        if self.from.y < self.to.y {
            self.from
        } else {
            self.to
        }
    }

    #[inline]
    fn leftmost_point(&self) -> Vector2 {
        if self.from.x < self.to.x {
            self.from
        } else {
            self.to
        }
    }

    #[inline]
    fn rightmost_point(&self) -> Vector2 {
        if self.from.x > self.to.x {
            self.from
        } else {
            self.to
        }
    }
}

impl Quad {
    pub fn new(from: Vector2, ctrl: Vector2, to: Vector2) -> Self {
        Self { from, ctrl, to }
    }

    pub fn calculate_distance(&self, point: Vector2) -> SignedDistance {
        crate::math::signed_distance_from_quad(*self, point)
    }

    // TODO explain
    #[inline]
    pub fn shoelace(&self) -> f32 {
        self.from.cross(self.to)
    }

    #[inline]
    fn highest_point(&self) -> Vector2 {
        let mut highest = self.from;
        if highest.y < self.ctrl.y {
            highest = self.ctrl;
        }
        if highest.y < self.to.y {
            highest = self.to;
        }
        highest
    }

    #[inline]
    fn lowest_point(&self) -> Vector2 {
        let mut lowest = self.from;
        if lowest.y > self.ctrl.y {
            lowest = self.ctrl;
        }
        if lowest.y > self.to.y {
            lowest = self.to;
        }
        lowest
    }

    #[inline]
    fn leftmost_point(&self) -> Vector2 {
        let mut leftmost = self.from;
        if leftmost.x > self.ctrl.x {
            leftmost = self.ctrl;
        }
        if leftmost.x > self.to.x {
            leftmost = self.to;
        }
        leftmost
    }

    #[inline]
    fn rightmost_point(&self) -> Vector2 {
        let mut rightmost = self.from;
        if rightmost.x < self.ctrl.x {
            rightmost = self.ctrl;
        }
        if rightmost.x < self.to.x {
            rightmost = self.to;
        }
        rightmost
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

    #[inline]
    fn highest_point(&self) -> Vector2 {
        let mut highest = self.from;
        if highest.y < self.ctrl1.y {
            highest = self.ctrl1;
        }
        if highest.y < self.ctrl2.y {
            highest = self.ctrl2;
        }
        if highest.y < self.to.y {
            highest = self.to;
        }
        highest
    }

    #[inline]
    fn lowest_point(&self) -> Vector2 {
        let mut lowest = self.from;
        if lowest.y > self.ctrl1.y {
            lowest = self.ctrl1;
        }
        if lowest.y > self.ctrl2.y {
            lowest = self.ctrl2;
        }
        if lowest.y > self.to.y {
            lowest = self.to;
        }
        lowest
    }

    #[inline]
    fn leftmost_point(&self) -> Vector2 {
        let mut leftmost = self.from;
        if leftmost.x > self.ctrl1.x {
            leftmost = self.ctrl1;
        }
        if leftmost.x > self.ctrl2.x {
            leftmost = self.ctrl2;
        }
        if leftmost.x > self.to.x {
            leftmost = self.to;
        }
        leftmost
    }

    #[inline]
    fn rightmost_point(&self) -> Vector2 {
        let mut rightmost = self.from;
        if rightmost.x < self.ctrl1.x {
            rightmost = self.ctrl1;
        }
        if rightmost.x < self.ctrl2.x {
            rightmost = self.ctrl2;
        }
        if rightmost.x < self.to.x {
            rightmost = self.to;
        }
        rightmost
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
