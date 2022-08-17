use cgmath::Vector2;
use rusttype::{OutlineBuilder, Point};

use crate::math::SignedDistance;

#[derive(Debug)]
pub struct Shape {
    contours: Vec<Contour>,
}

impl Shape {
    pub fn get_segments(&self) -> &[Contour] {
        self.contours.as_slice()
    }
}

#[derive(Debug)]
pub struct Contour {
    pub segments: Vec<Segment>,
}

#[derive(Debug)]
pub enum Segment {
    Line(Line),
    Quadratic(Quad),
    Cubic(Curve),
    End(),
}

#[derive(Copy, Clone, Debug)]
pub struct Line {
    pub from: Vector2<f32>,
    pub to: Vector2<f32>,
}

#[derive(Copy, Clone, Debug)]
pub struct Quad {
    pub from: Vector2<f32>,
    pub control: Vector2<f32>,
    pub to: Vector2<f32>,
}

#[derive(Copy, Clone, Debug)]
pub struct Curve {
    pub from: Vector2<f32>,
    pub control1: Vector2<f32>,
    pub control2: Vector2<f32>,
    pub to: Vector2<f32>,
}

impl Line {
    pub fn new(from: Vector2<f32>, to: Vector2<f32>) -> Self {
        Self { from, to }
    }

    pub fn calculate_sd(&self, point: Vector2<f32>) -> SignedDistance {
        crate::math::signed_distance_from_line(*self, point)
    }
}

impl Quad {
    pub fn new(from: Vector2<f32>, control: Vector2<f32>, to: Vector2<f32>) -> Self {
        Self { from, control, to }
    }

    pub fn calculate_sd(&self, point: Vector2<f32>) -> SignedDistance {
        crate::math::signed_distance_from_quad(*self, point)
    }
}

impl Curve {
    pub fn new(
        from: Vector2<f32>,
        control1: Vector2<f32>,
        control2: Vector2<f32>,
        to: Vector2<f32>,
    ) -> Self {
        Self {
            from,
            control1,
            control2,
            to,
        }
    }
}

#[derive(Debug)]
pub struct ShapeBuilder {
    contours: Vec<Contour>,
    last_point: Option<Vector2<f32>>,
    position: Point<f32>,
}

impl ShapeBuilder {
    pub fn new(position: Point<f32>) -> Self {
        Self {
            contours: Vec::new(),
            last_point: None,
            position,
        }
    }

    pub fn build(self) -> Shape {
        Shape {
            contours: self.contours,
        }
    }

    fn move_to(&mut self, x: f32, y: f32) {
        self.add_shape();

        let to = Vector2::new(x + self.position.x, y + self.position.y);
        self.last_point = Some(to);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        let from = self.last_point.unwrap();
        let to = Vector2::new(x + self.position.x, y + self.position.y);
        self.add_segment(Segment::Line(Line::new(from, to)));
        self.last_point = Some(to);
    }

    fn quad_to(&mut self, ctrl_x1: f32, ctrl_y1: f32, x: f32, y: f32) {
        let from = self.last_point.unwrap();
        let control =
            Vector2::new(ctrl_x1 + self.position.x, ctrl_y1 + self.position.y);
        let to = Vector2::new(x + self.position.x, y + self.position.y);
        self.add_segment(Segment::Quadratic(Quad::new(from, control, to)));
        self.last_point = Some(to);
    }

    fn curve_to(
        &mut self,
        ctrl1_x: f32,
        ctrl1_y: f32,
        ctrl2_x: f32,
        ctrl2_y: f32,
        x: f32,
        y: f32,
    ) {
        let from = self.last_point.unwrap();
        let control1 = Vector2::new(ctrl1_x, ctrl1_y);
        let control2 = Vector2::new(ctrl2_x, ctrl2_y);
        let to = Vector2::new(x + self.position.x, y + self.position.y);
        self.add_segment(Segment::Cubic(Curve::new(from, control1, control2, to)));
        self.last_point = Some(to);
    }

    fn close(&mut self) {
        self.add_segment(Segment::End())
    }

    fn add_shape(&mut self) {
        self.contours.push(Contour {
            segments: Vec::new(),
        });
    }

    fn add_segment(&mut self, seg: Segment) {
        self.contours.last_mut().unwrap().segments.push(seg);
    }
}

impl OutlineBuilder for ShapeBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        println!("pocinjem na: {} {}", x, y);
        self.move_to(x, y);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        println!("crta do: {} {}", x, y);
        self.line_to(x, y);
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        println!(
            "kvadraticna parabola: x1: {}, y1: {}, x: {}, y: {}",
            x1, y1, x, y
        );
        self.quad_to(x1, y1, x, y);
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        println!(
            "kubna parabola: x1: {}, y1: {}, x2: {}, y2: {} x: {}, y: {}",
            x1, y1, x2, y2, x, y
        );
        self.curve_to(x1, y1, x2, y2, x, y);
    }

    fn close(&mut self) {
        println!("_________kraj________");
        self.close();
    }
}

#[test]
fn quad_curve_test() {
    let curve = Quad {
        from: Vector2 {
            x: 114.5726,
            y: 75.58819,
        },
        control: Vector2 {
            x: 54.5726,
            y: 75.58819,
        },
        to: Vector2 {
            x: 112.56276,
            y: 82.80722,
        },
    };
    let point = Vector2 { x: 120.0, y: 72.0 };
    let sd = curve.calculate_sd(point);

    println!("sd: {:?}", sd);
}
