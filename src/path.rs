use rusttype::{OutlineBuilder, Scale};

use crate::shape::{Contour, Curve, Line, Quad, Segment, Shape, Winding};
use crate::vector::Vector2;
use crate::{BBox, Offset};

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
    offset: Offset,
    //scale: NormScale,

    // Temporary values
    shoelace: f32,
    last_point: Option<Vector2<f32>>,
    temp_segments: Vec<Segment>,
}

impl PathBuilder {
    // TODO add custom errors for the builder
    // TODO add a function for each step: "error check"

    /// Creates a new builder with scale set to `1` meaning none path instructions
    /// will be scaled. Use [`PathBuilder::new_with_scale`] for scaling.
    pub fn new(offset: Offset) -> Self {
        Self {
            contours: Vec::new(),
            offset,

            shoelace: 0.0,
            last_point: None,
            temp_segments: Vec::new(),
        }
    }

    pub fn open_at(&mut self, x: f32, y: f32) {
        self.open_at_check();

        let to = Vector2::new(x + self.offset.x, y + self.offset.y);
        self.last_point = Some(to);
    }

    pub fn line_to(&mut self, x: f32, y: f32) {
        self.line_to_check();

        let from = self.last_point.unwrap();
        let to = Vector2::new(x + self.offset.x, y + self.offset.y);
        let line = Line::new(from, to);

        self.shoelace += line.shoelace();
        self.temp_segments.push(Segment::Line(line));
        self.last_point = Some(to);
    }

    pub fn quad_to(&mut self, ctrl_x: f32, ctrl_y: f32, x: f32, y: f32) {
        self.quad_to_check();

        let from = self.last_point.unwrap();
        let control = Vector2::new(ctrl_x + self.offset.x, ctrl_y + self.offset.y);
        let to = Vector2::new(x + self.offset.x, y + self.offset.y);
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
        self.curve_to_check();

        let from = self.last_point.unwrap();
        let ctrl1 = Vector2::new(ctrl1_x + self.offset.x, ctrl1_y + self.offset.y);
        let ctrl2 = Vector2::new(ctrl2_x + self.offset.x, ctrl2_y + self.offset.y);
        let to = Vector2::new(x + self.offset.x, y + self.offset.y);
        let curve = Curve::new(from, ctrl1, ctrl2, to);

        self.shoelace += curve.shoelace();
        self.temp_segments
            .push(Segment::Cubic(Curve::new(from, ctrl1, ctrl2, to)));
        self.last_point = Some(to);
        unimplemented!("Not implemented!!!")
    }

    pub fn close(&mut self) {
        self.close_check();

        // TODO test if windings are right
        let winding = Winding(self.shoelace < 0.0);
        //println!("winding: {:?}", winding);
        let segments = self.temp_segments.drain(..).collect::<Vec<_>>();

        self.contours.push(Contour::new(segments, winding));
        self.shoelace = 0.0;
        self.last_point = None;
    }

    #[inline]
    fn open_at_check(&self) {
        assert!(
            self.last_point.is_none(),
            "PathBuilder Error: The last contour has not been closed!"
        );
        assert!(
            self.temp_segments.is_empty(),
            "PathBuilder Error: The last contour wasn't closed!"
        );
    }

    #[inline]
    fn line_to_check(&self) {
        assert!(
            self.last_point.is_some(),
            "PathBuilder Error: Open a new contour before adding segments!"
        );
    }

    #[inline]
    fn quad_to_check(&self) {
        assert!(
            self.last_point.is_some(),
            "PathBuilder Error: Open a new contour before adding segments!"
        );
    }

    #[inline]
    fn curve_to_check(&self) {
        assert!(
            self.last_point.is_some(),
            "PathBuilder Error: Open a new contour before adding segments!"
        );
    }

    #[inline]
    fn close_check(&self) {
        assert!(
            !self.temp_segments.is_empty(),
            "PathBuilder Error: There are no contours to close or there are zero segments!"
        );
        assert!(
            self.last_point.is_some(),
            "PathBuilder Error: The last contour has already been closed!"
        );
    }

    pub fn build_shape(self) -> Shape {
        assert!(
            !self.contours.is_empty(),
            "PathBuilder Error: There are no contours."
        );
        assert!(
            self.last_point.is_none(),
            "PathBuilder Error: The last contour is still open."
        );

        Shape::new(self.contours)
    }

    pub fn build_shape_scaled(mut self, scale: Scale) -> Shape {
        for contour in self.contours.iter_mut() {
            for segment in contour.segments.iter_mut() {
                match segment {
                    Segment::Line(l) => l.rescale(scale),
                    Segment::Quadratic(q) => q.rescale(scale),
                    Segment::Cubic(c) => c.rescale(scale),
                }
            }
        }

        self.build_shape()
    }
}

impl OutlineBuilder for PathBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        println!("moving to: {} {}", x, y);

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

pub struct ShapeBuilder {
    path: PathBuilder,
    bbox: BBox,
    scale: Option<Scale>,
}

impl ShapeBuilder {
    pub fn new(width: u32, height: u32, scale: Option<Scale>, offset: Offset) -> Self {
        let builder = PathBuilder::new(offset);
        let bbox = BBox::new(
            Vector2::ZERO_I32,
            Vector2::new(width as i32, height as i32),
        );
        Self {
            path: builder,
            bbox,
            scale,
        }
    }

    pub fn open_at(&mut self, x: f32, y: f32) {
        self.path.open_at(x, y);
    }

    pub fn line_to(&mut self, x: f32, y: f32) {
        self.path.line_to(x, y);
    }

    pub fn quad_to(&mut self, ctrl_x: f32, ctrl_y: f32, x: f32, y: f32) {
        self.path.quad_to(ctrl_x, ctrl_y, x, y);
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
        self.path.curve_to(ctrl1_x, ctrl1_y, ctrl2_x, ctrl2_y, x, y);
    }

    pub fn close(&mut self) {
        self.path.close();
    }

    pub fn build(mut self) -> (Shape, BBox) {
        if let Some(scale) = self.scale {
            self.bbox.scale(scale);
            (self.path.build_shape_scaled(scale), self.bbox)
        } else {
            (self.path.build_shape(), self.bbox)
        }
    }
}

// TODO maybe add tests for each module
