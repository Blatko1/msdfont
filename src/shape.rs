use rusttype::OutlineBuilder;
use std::slice::Iter;

use crate::math::{ContourSignedDistance, SignedDistance};
use crate::vector::Vector2;

#[derive(Debug)]
pub struct Shape {
    contours: Vec<Contour>,
}

impl Shape {
    /// Iterates over shape contours.
    pub fn iter(&self) -> Iter<'_, Contour> {
        self.contours.iter()
    }
}

#[derive(Debug)]
pub struct Contour {
    segments: Vec<Segment>,
    winding: Winding,
}

impl Contour {
    pub fn get_distance_from(&self, point: Vector2) -> ContourSignedDistance {
        let mut shortest_dist = SignedDistance::MAX;
        for segment in &self.segments {
            let dist = match segment {
                Segment::Line(line) => line.calculate_distance(point),
                Segment::Quadratic(quad) => quad.calculate_distance(point),
                Segment::Cubic(curve) => unimplemented!("Not yet!!!"),
            };

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

#[test]
fn quad_curve_test() {
    let curve = Quad {
        from: Vector2 {
            x: 114.5726,
            y: 75.58819,
        },
        ctrl: Vector2 {
            x: 54.5726,
            y: 75.58819,
        },
        to: Vector2 {
            x: 112.56276,
            y: 82.80722,
        },
    };
    let point = Vector2 { x: 120.0, y: 72.0 };
    let sd = curve.calculate_distance(point);

    println!("sd: {:?}", sd);
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
