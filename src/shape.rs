use rusttype::OutlineBuilder;
use std::slice::Iter;

use crate::math::{ContourSignedDistance, SignedDistance};
use crate::overlaps::OverlapData;
use crate::vector::Vector2;

#[derive(Debug)]
pub struct Shape {
    contours: Vec<Contour>,
    overlaps: OverlapData,
}

impl Shape {
    /// Iterates over shape contours.
    #[inline]
    pub fn iter(&self) -> Iter<'_, Contour> {
        self.contours.iter()
    }

    pub fn are_overlapping(&self, id1: ContourID, id2: ContourID) -> bool {
        self.overlaps.are_overlapping(id1, id2)
    }

    pub fn has_overlaps(&self) -> bool {
        !self.overlaps.is_empty()
    }
}

// TODO implement Form<> Into<> and some functions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ContourID(u16);

#[derive(Debug)]
pub struct Contour {
    id: ContourID,
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
            contour_id: self.id,
            contour_winding: self.winding,
        }
    }

    pub fn id(&self) -> ContourID {
        self.id
    }

    pub fn iter(&self) -> Iter<'_, Segment> {
        self.segments.iter()
    }

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
    fn distance(&self, point: Vector2) -> SignedDistance {
        match self {
            Segment::Line(line) => line.calculate_distance(point),
            Segment::Quadratic(quad) => quad.calculate_distance(point),
            Segment::Cubic(curve) => todo!(),
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
    shoelace: f32,
}

impl ShapeBuilder {
    // TODO add custom errors for the builder
    pub fn new<P: Into<Vector2>>(pos: P) -> Self {
        Self {
            contours: Vec::new(),
            last_point: None,
            pos: pos.into(),
            shoelace: 0.0,
        }
    }

    pub fn start_at(&mut self, x: f32, y: f32) {
        assert!(
            self.last_point.is_none(),
            "ShapeBuilder Error: The last contour has not been closed!"
        );
        let id = ContourID(self.contours.len() as u16);
        // Open a new contour:
        self.contours.push(Contour {
            id,
            segments: Vec::new(),
            winding: Winding(false),
        });

        let to = Vector2::new(x + self.pos.x, y + self.pos.y);
        self.last_point = Some(to);
    }

    pub fn line_to(&mut self, x: f32, y: f32) {
        assert!(
            self.last_point.is_some(),
            "ShapeBuilder Error: Open a new contour before adding segments!"
        );

        let from = self.last_point.unwrap();
        let to = Vector2::new(x + self.pos.x, y + self.pos.y);
        let line = Line::new(from, to);

        self.shoelace += line.shoelace();
        self.add_segment(Segment::Line(line));
        self.last_point = Some(to);
    }

    pub fn quad_to(&mut self, ctrl_x: f32, ctrl_y: f32, x: f32, y: f32) {
        assert!(
            self.last_point.is_some(),
            "ShapeBuilder Error: Open a new contour before adding segments!"
        );

        let from = self.last_point.unwrap();
        let control = Vector2::new(ctrl_x + self.pos.x, ctrl_y + self.pos.y);
        let to = Vector2::new(x + self.pos.x, y + self.pos.y);
        let quad = Quad::new(from, control, to);

        self.shoelace += quad.shoelace();
        self.add_segment(Segment::Quadratic(Quad::new(from, control, to)));
        self.last_point = Some(to);
    }

    pub fn curve_to(
        &mut self,
        ctrl1_x: f32,
        ctrl1_y: f32,
        ctrl2_x: f32,
        ctrl2_y: f32,
        x: f32,
        y: f32,
    ) {
        assert!(
            self.last_point.is_some(),
            "ShapeBuilder Error: Open a new contour before adding segments!"
        );

        let from = self.last_point.unwrap();
        let ctrl1 = Vector2::new(ctrl1_x, ctrl1_y);
        let ctrl2 = Vector2::new(ctrl2_x, ctrl2_y);
        let to = Vector2::new(x + self.pos.x, y + self.pos.y);
        let curve = Curve::new(from, ctrl1, ctrl2, to);

        self.shoelace += curve.shoelace();
        self.add_segment(Segment::Cubic(Curve::new(from, ctrl1, ctrl2, to)));
        self.last_point = Some(to);
        unimplemented!("Not implemented!!!")
    }

    pub fn close(&mut self) {
        assert!(
            !self.contours.is_empty(),
            "ShapeBuilder Error: There are no contours to close!"
        );
        assert!(
            self.last_point.is_some(),
            "ShapeBuilder Error: The last contour has already been closed!"
        );
        let last = self.contours.last_mut().unwrap();
        assert!(
            !last.segments.is_empty(),
            "ShapeBuilder Error: The current contour has no segments!"
        );

        last.winding = Winding(self.shoelace > 0.0);
        self.shoelace = 0.0;
        self.last_point = None;
    }

    pub fn build(self) -> Shape {
        let overlaps = OverlapData::from_contours(&self.contours);
        println!("overlaps: {:?}", overlaps);
        Shape {
            contours: self.contours,
            overlaps,
        }
    }

    #[inline]
    fn add_segment(&mut self, seg: Segment) {
        self.contours.last_mut().unwrap().segments.push(seg);
    }
}

impl OutlineBuilder for ShapeBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        //println!("moving to: {} {}", x, y);

        self.start_at(x, y);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        //println!("line to: {} {}", x, y);

        self.line_to(x, y);
    }

    /// `x` and `y` represent the ending point and
    /// `x1` and `x2` represent the control point.
    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        //println!(
        //    "quadratic parabola: x1: {}, y1: {}, x: {}, y: {}",
        //    x1, y1, x, y
        //);

        self.quad_to(x1, y1, x, y);
    }

    /// `x` and `y` represent the ending point and
    /// `x1`, `y1`, `x2` and `y2` represent control points.
    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        //println!(
        //    "cubic parabola: x1: {}, y1: {}, x2: {}, y2: {} x: {}, y: {}",
        //    x1, y1, x2, y2, x, y
        //);

        self.curve_to(x1, y1, x2, y2, x, y)
    }

    fn close(&mut self) {
        //println!("_________END_________");

        self.close();
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
