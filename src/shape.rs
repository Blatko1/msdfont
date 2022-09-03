use std::slice::Iter;

use owned_ttf_parser::OutlineBuilder;

use crate::contour::{Contour, Curve, Line, Quad, Segment, Winding};
use crate::font::Scale;
use crate::vector::Vector2;

#[derive(Debug)]
pub struct Shape {
    contours: Vec<Contour>,
}

impl Shape {
    /// Iterates over shape contours.
    #[inline]
    pub fn iter(&self) -> Iter<'_, Contour> {
        self.contours.iter()
    }
}

#[derive(Debug)]
pub struct ShapeBuilder {
    contours: Vec<Contour>,
    scale: Scale,

    // Temporary values
    shoelace: f32,
    last_point: Option<Vector2>,
    temp_segments: Vec<Segment>,
}

impl ShapeBuilder {
    // TODO add custom errors for the builder
    // TODO add a function for each step: "error check"
    pub fn new(scale: Scale) -> Self {
        Self {
            contours: Vec::new(),
            scale,

            shoelace: 0.0,
            last_point: None,
            temp_segments: Vec::new(),
        }
    }

    pub fn start_at(&mut self, x: f32, y: f32) {
        assert!(
            self.last_point.is_none(),
            "ShapeBuilder Error: The last contour has not been closed!"
        );
        assert!(
            self.temp_segments.is_empty(),
            "ShapeBuilder Error: The last contour wasn't closed!"
        );

        let to = Vector2::new(x * self.scale.0, y * self.scale.0);
        self.last_point = Some(to);
    }

    pub fn line_to(&mut self, x: f32, y: f32) {
        assert!(
            self.last_point.is_some(),
            "ShapeBuilder Error: Open a new contour before adding segments!"
        );

        let from = self.last_point.unwrap();
        let to = Vector2::new(x * self.scale.0, y * self.scale.0);
        let line = Line::new(from, to);

        self.shoelace += line.shoelace();
        self.temp_segments.push(Segment::Line(line));
        self.last_point = Some(to);
    }

    pub fn quad_to(&mut self, ctrl_x: f32, ctrl_y: f32, x: f32, y: f32) {
        assert!(
            self.last_point.is_some(),
            "ShapeBuilder Error: Open a new contour before adding segments!"
        );

        let from = self.last_point.unwrap();
        let control = Vector2::new(ctrl_x * self.scale.0, ctrl_y * self.scale.0);
        let to = Vector2::new(x * self.scale.0, y * self.scale.0);
        let quad = Quad::new(from, control, to);

        self.shoelace += quad.shoelace();
        self.temp_segments
            .push(Segment::Quadratic(Quad::new(from, control, to)));
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
        let ctrl1 = Vector2::new(ctrl1_x * self.scale.0, ctrl1_y * self.scale.0);
        let ctrl2 = Vector2::new(ctrl2_x * self.scale.0, ctrl2_y * self.scale.0);
        let to = Vector2::new(x * self.scale.0, y * self.scale.0);
        let curve = Curve::new(from, ctrl1, ctrl2, to);

        self.shoelace += curve.shoelace();
        self.temp_segments
            .push(Segment::Cubic(Curve::new(from, ctrl1, ctrl2, to)));
        self.last_point = Some(to);
        unimplemented!("Not implemented!!!")
    }

    pub fn close(&mut self) {
        assert!(
            !self.temp_segments.is_empty(),
            "ShapeBuilder Error: There are no contours to close or there are zero segments!"
        );
        assert!(
            self.last_point.is_some(),
            "ShapeBuilder Error: The last contour has already been closed!"
        );
        // TODO test if windings are right
        let winding = Winding(self.shoelace < 0.0);
        println!("winding: {:?}", winding);
        let segments = self.temp_segments.drain(..).collect::<Vec<_>>();

        self.contours.push(Contour::new(segments, winding));
        self.shoelace = 0.0;
        self.last_point = None;
    }

    pub fn build(self) -> Shape {
        Shape {
            contours: self.contours,
        }
    }
}

impl OutlineBuilder for ShapeBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        println!("moving to: {} {}", x, y);

        self.start_at(x, y);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        println!("line to: {} {}", x, y);

        self.line_to(x, y);
    }

    /// `x` and `y` represent the ending point and
    /// `x1` and `x2` represent the control point.
    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        println!(
            "quadratic parabola: x1: {}, y1: {}, x: {}, y: {}",
            x1, y1, x, y
        );

        self.quad_to(x1, y1, x, y);
    }

    /// `x` and `y` represent the ending point and
    /// `x1`, `y1`, `x2` and `y2` represent control points.
    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        println!(
            "cubic parabola: x1: {}, y1: {}, x2: {}, y2: {} x: {}, y: {}",
            x1, y1, x2, y2, x, y
        );

        self.curve_to(x1, y1, x2, y2, x, y)
    }

    fn close(&mut self) {
        println!("_________END_________");

        self.close();
    }
}

// TODO maybe add tests for each module
