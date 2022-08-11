use std::ops::Neg;

use crate::shape::Line;
use cgmath::{InnerSpace, Vector2};

#[derive(Debug, PartialEq)]
pub struct SignedDistance {
    pub extended_dist: f32,
    pub real_dist: f32,
    pub orthogonality: f32,
    pub sign: f32,
}

impl SignedDistance {
    pub const MAX: Self = SignedDistance {
        extended_dist: f32::MAX,
        real_dist: f32::MAX,
        orthogonality: 0.0,
        sign: f32::NAN,
    };
}

impl PartialOrd for SignedDistance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering;
        match self.real_dist.abs().partial_cmp(&other.real_dist.abs()) {
            Some(Ordering::Less) => Some(Ordering::Less),
            Some(Ordering::Greater) => Some(Ordering::Greater),
            Some(Ordering::Equal) => other.orthogonality.partial_cmp(&self.orthogonality),
            None => None,
        }
    }
}

pub fn line_sd(line: Line, point: Vector2<f32>) -> SignedDistance {
    let p0 = line.from;
    let p1 = line.to;
    let p = point;

    let p_p0 = p - p0;
    let p1_p0 = p1 - p0;

    // Find the "t" from the line function
    // and restrict it to an interval [0.0, 1.0].
    let extended_pos = p_p0.dot(p1_p0) / p1_p0.dot(p1_p0);
    let real_pos = extended_pos.clamp(0.0, 1.0);

    // Put "t" in bezier function and get the closest
    // point to the current pixel "p"
    let bezier = p0 + real_pos * p1_p0;
    let extended_bezier = p0 + extended_pos * p1_p0;
    let bezier_p = bezier - p;

    // Get the distance from current pixel "p" to bezier line.
    let real_dist = bezier_p.magnitude();
    let extended_dist = extended_bezier.magnitude();

    // Invert the vector to get distance from bezier line to "p".
    let p_bezier = bezier_p.neg();
    let ortho: f32 = if p_bezier.x == 0.0 && p_bezier.y == 0.0 {
        0.0
    } else {
        p1_p0.normalize().perp_dot(p_bezier.normalize())
    };
    let sign = ortho.signum();
    let orthogonality = ortho.abs();

    SignedDistance {
        extended_dist,
        real_dist,
        orthogonality,
        sign,
    }
}
