use rusttype::OutlineBuilder;
use std::slice::Iter;

use crate::math::{ContourSignedDistance, SignedDistance};
use crate::overlaps::Intersectable;
use crate::vector::Vector2;

#[derive(Debug)]
pub struct Shape {
    contours: Vec<Contour>,
    overlaps: Option<ContourOverlaps>,
}

impl Shape {
    /// Iterates over shape contours.
    #[inline]
    pub fn iter(&self) -> Iter<'_, Contour> {
        self.contours.iter()
    }

    pub fn find_overlaps(&mut self) {
        let len = self.contours.len();
        // TODO explain
        // Compare each contour with another avoiding duplicate comparisons.
        for (index, contour) in (&self.contours[0..len-1]).iter().enumerate() {
            for other in self.contours.iter().skip(index + 1) {
                if contour.overlaps(other) {
                    todo!()
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Contour {
    segments: Vec<Segment>,
    winding: Winding,
}

impl Contour {
    /// Returns the shortest distance from the provided point to the contour.
    pub fn get_distance(&self, point: Vector2) -> ContourSignedDistance {
        let mut shortest_dist = SignedDistance::MAX;
        // TODO maybe use iter
        for segment in &self.segments {
            let dist = segment.distance(point);

            // To learn more about the comparison go to `SignedDistance::partial_cmp`
            if dist < shortest_dist {
                shortest_dist = dist;
            }
        }
        ContourSignedDistance {
            distance: shortest_dist,
            contour_winding: self.winding,
        }
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        let len = self.segments.len();
        // TODO explain
        // Compare each segment with another avoiding duplicate comparisons.
        // If an intersection is found immediately return `true`.
        for (index, segment) in (&self.segments[0..len-1]).iter().enumerate() {
            for other in self.segments.iter().skip(index + 1) {
                if segment.intersects_with(other) {
                    return true;
                }
            }
        }
        false
    }

    pub fn winding(&self) -> Winding {
        self.winding
    }
}

#[derive(Debug)]
pub struct ContourOverlaps {

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
            Segment::Line(line) => line.calculate_distance(point),
            Segment::Quadratic(quad) => quad.calculate_distance(point),
            Segment::Cubic(curve) => todo!(),
        }
    }

    fn intersects_with(&self, other: &Self) -> bool {
        match self {
            Segment::Line(line) => line.intersects_with(other),
            Segment::Quadratic(quad) => quad.intersects_with(other),
            Segment::Cubic(curve) => curve.intersects_with(other),
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

    /// Get the resulting point from this line function with `t` as a param.
    pub fn point(&self, t: f32) -> Vector2 {
        crate::math::line_fn(self.from, self.to, t)
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

    pub fn calculate_distance(&self, point: Vector2) -> SignedDistance {
        crate::math::signed_distance_from_quad(*self, point)
    }

    /// Get the resulting point from this curve function with `t` as a param.
    pub fn point(&self, t: f32) -> Vector2 {
        crate::math::quadratic_fn(self.from, self.ctrl, self.to, t)
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

#[derive(Debug)]
pub struct ShapeBuilder {
    contours: Vec<Contour>,
    last_point: Option<Vector2>,
    pos: Vector2,
    contour_area: f32,
}

impl ShapeBuilder {
    pub fn new<P: Into<(f32, f32)>>(pos: P) -> Self {
        Self {
            contours: Vec::new(),
            last_point: None,
            pos: Vector2::from(pos.into()),
            contour_area: 0.0,
        }
    }

    pub fn build(self) -> Shape {
        Shape {
            contours: self.contours,
            overlaps: None,
        }
    }

    #[inline]
    fn add_shape(&mut self) {
        self.contours.push(Contour {
            segments: Vec::new(),
            winding: Winding(false),
        });
    }

    #[inline]
    fn add_segment(&mut self, seg: Segment) {
        self.contours.last_mut().unwrap().segments.push(seg);
    }
}

impl OutlineBuilder for ShapeBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        println!("moving to: {} {}", x, y);
        self.add_shape();

        let to = Vector2::new(x + self.pos.x, y + self.pos.y);
        self.last_point = Some(to);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        println!("line to: {} {}", x, y);
        let from = self.last_point.unwrap();
        let to = Vector2::new(x + self.pos.x, y + self.pos.y);
        let line = Line::new(from, to);

        self.contour_area += line.shoelace();
        self.add_segment(Segment::Line(line));
        self.last_point = Some(to);
    }

    /// `x` and `y` represent the ending point and
    /// `x1` and `x2` represent the control point.
    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        println!(
            "quadratic parabola: x1: {}, y1: {}, x: {}, y: {}",
            x1, y1, x, y
        );
        let from = self.last_point.unwrap();
        let control = Vector2::new(x1 + self.pos.x, y1 + self.pos.y);
        let to = Vector2::new(x + self.pos.x, y + self.pos.y);
        let quad = Quad::new(from, control, to);

        self.contour_area += quad.shoelace();
        self.add_segment(Segment::Quadratic(Quad::new(from, control, to)));
        self.last_point = Some(to);
    }

    /// `x` and `y` represent the ending point and
    /// `x1`, `x2`, `x1` and `x2` represent control points.
    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        println!(
            "cubic parabola: x1: {}, y1: {}, x2: {}, y2: {} x: {}, y: {}",
            x1, y1, x2, y2, x, y
        );
        let from = self.last_point.unwrap();
        let ctrl1 = Vector2::new(x1, y1);
        let ctrl2 = Vector2::new(x2, y2);
        let to = Vector2::new(x + self.pos.x, y + self.pos.y);
        let curve = Curve::new(from, ctrl1, ctrl2, to);

        self.contour_area += curve.shoelace();
        self.add_segment(Segment::Cubic(Curve::new(from, ctrl1, ctrl2, to)));
        self.last_point = Some(to);
        unimplemented!("Not implemented!!!")
    }

    fn close(&mut self) {
        println!("_________END_________");

        let area = self.contour_area * 0.5;

        self.contours.last_mut().unwrap().winding = Winding(area > 0.0);
        self.contour_area = 0.0;
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
pub struct Winding(bool);

impl Winding {
    /// Check if the winding is clockwise.
    #[inline]
    pub fn is_cw(&self) -> bool {
        self.0 == true
    }

    /// Check if the winding is counter clockwise.
    #[inline]
    pub fn is_ccw(&self) -> bool {
        !self.is_cw()
    }
}

// TODO maybe add tests for each module