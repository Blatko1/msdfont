use owned_ttf_parser::OutlineBuilder;

use crate::Scale;
use crate::shape::{Contour, Curve, Line, Quad, Segment, Winding, Shape};
use crate::font::{NormScale};
use crate::vector::Vector2;

/// `PathBuilder` (aka. `ShapeBuilder`) builds a path from five opentype font instructions:
/// - `move_to`
/// - `line_to`
/// - `quad_to`
/// - `curve_to`
/// - `close`
/// 
/// After processing all instructions a shape can easily be created.
#[derive(Debug)]
pub struct PathBuilder {
    contours: Vec<Contour>,
    scale: NormScale,

    // Temporary values
    shoelace: f32,
    last_point: Option<Vector2>,
    temp_segments: Vec<Segment>,
}

impl PathBuilder {
    // TODO add custom errors for the builder
    // TODO add a function for each step: "error check"

    /// Creates a new builder with scale set to `1` meaning none path instructions
    /// will be scaled. Use [`PathBuilder::new_with_scale`] for scaling.
    pub fn new() -> Self {
        Self {
            contours: Vec::new(),
            scale: Scale::default().normalize(1.0),

            shoelace: 0.0,
            last_point: None,
            temp_segments: Vec::new(),
        }
    }

    pub fn new_with_scale(scale: Scale) -> Self {
        Self {
            contours: Vec::new(),
            scale: scale.normalize(1.0),

            shoelace: 0.0,
            last_point: None,
            temp_segments: Vec::new(),
        }
    }

    pub(crate) fn new_with_norm_scale(scale: NormScale) -> Self {
        Self {
            contours: Vec::new(),
            scale,

            shoelace: 0.0,
            last_point: None,
            temp_segments: Vec::new(),
        }
    }

    pub fn open_at(&mut self, x: f32, y: f32) {
        assert!(
            self.last_point.is_none(),
            "ShapeBuilder Error: The last contour has not been closed!"
        );
        assert!(
            self.temp_segments.is_empty(),
            "ShapeBuilder Error: The last contour wasn't closed!"
        );

        let to = Vector2::new(x * self.scale, y * self.scale);
        self.last_point = Some(to);
    }

    pub fn line_to(&mut self, x: f32, y: f32) {
        assert!(
            self.last_point.is_some(),
            "ShapeBuilder Error: Open a new contour before adding segments!"
        );

        let from = self.last_point.unwrap();
        let to = Vector2::new(x * self.scale, y * self.scale);
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
        let control = Vector2::new(ctrl_x * self.scale, ctrl_y * self.scale);
        let to = Vector2::new(x * self.scale, y * self.scale);
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
        let ctrl1 = Vector2::new(ctrl1_x * self.scale, ctrl1_y * self.scale);
        let ctrl2 = Vector2::new(ctrl2_x * self.scale, ctrl2_y * self.scale);
        let to = Vector2::new(x * self.scale, y * self.scale);
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
        //println!("winding: {:?}", winding);
        let segments = self.temp_segments.drain(..).collect::<Vec<_>>();

        self.contours.push(Contour::new(segments, winding));
        self.shoelace = 0.0;
        self.last_point = None;
    }

    // TODO add checks if last contour was closed
    pub fn build_shape(self) -> Shape {
        Shape::new(self.contours)
    }
}

impl OutlineBuilder for PathBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        //println!("moving to: {} {}", x, y);

        self.open_at(x, y);
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

// TODO maybe add tests for each module
