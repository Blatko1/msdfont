use std::f32::consts::PI;

use crate::shape::{Line, Quad, Winding};
use crate::vector::Vector2;

// TODO create struct which holds distance to segments used for
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ContourSignedDistance {
    pub distance: SignedDistance,
    pub contour_winding: Winding,
}

// TODO check if needed
//impl PartialOrd for ContourSignedDistance {
//    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//        self.distance.partial_cmp(&other.distance)
//    }
//}

/// Distance from pixel to contour
#[derive(Debug, PartialEq, Clone, Copy)]
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
        let diff = self.real_dist - other.real_dist;
        match diff.abs().partial_cmp(&0.01) {
            Some(Ordering::Less) => {
                other.orthogonality.partial_cmp(&self.orthogonality)
            }
            Some(Ordering::Greater) => self.real_dist.partial_cmp(&other.real_dist),
            Some(Ordering::Equal) => {
                other.orthogonality.partial_cmp(&self.orthogonality)
            }
            None => None,
        }
    }
}

pub fn signed_distance_from_line(line: Line, point: Vector2) -> SignedDistance {
    let p0 = line.from;
    let p1 = line.to;
    let p = point;

    let p_p0 = p - p0;
    let p1_p0 = p1 - p0; // Is also the direction

    // Find the "t" from the line function
    // and restrict it to an interval [0.0, 1.0].
    let extended_pos = p_p0.dot(p1_p0) / p1_p0.dot(p1_p0);
    let real_pos = extended_pos.clamp(0.0, 1.0);

    // Put "t" in bezier function and get the closest
    // point to the current pixel "p"
    let extended_bezier = p0 + extended_pos * p1_p0;
    let extend_bezier_p = extended_bezier - p;
    let bezier = p0 + real_pos * p1_p0;
    let bezier_p = bezier - p;

    // Get the distance from current pixel "p" to bezier line.
    let real_dist = bezier_p.magnitude();
    let extended_dist = extend_bezier_p.magnitude();

    // Invert the vector to get distance from bezier line to "p".
    let p_bezier = bezier_p.neg();
    let ortho: f32 = if p_bezier.is_zero() {
        0.0
    } else {
        p1_p0.normalize().cross(p_bezier.normalize())
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

pub fn signed_distance_from_quad(quad: Quad, point: Vector2) -> SignedDistance {
    let p0 = quad.from;
    let p1 = quad.ctrl;
    let p2 = quad.to;
    let p = point;

    let v = p - p0;
    let v1 = p1 - p0;
    let v2 = p2 - 2.0 * p1 + p0;
    // quadratic Bezier curve:
    // (v2 · v2)t^3 + 3(v1 · v2)t^2 + (2*v1 · v1 − v2 · v)t − v1 · v = 0
    // general quadratic:
    // a * t^3 + b * t^2 + c * t + d = 0

    let a = v2.dot(v2);
    let b = 3.0 * v1.dot(v2);
    let c = 2.0 * v1.dot(v1) - v2.dot(v);
    let d = -v1.dot(v);

    // Get roots:
    let roots = find_cubic_roots(a, b, c, d);

    let mut extended_pos = 0.0;
    let mut real_pos = 0.0;
    let mut closest_bezier = Vector2::new(f32::MAX, f32::MAX);
    let mut smallest_dist2 = f32::MAX; // Not square rooted

    // Compare all roots to find the closest "t" and smallest distance.
    for r in roots.iter().flatten() {
        // <-- automatically filters out Options with None
        // Use clamped root in the quadratic function.
        let t = r.clamp(0.0, 1.0);
        let bezier = t * t * v2 + 2.0 * t * v1 + p0;

        // Then compare the distances for each root.
        let dist2 = (bezier - p).magnitude2();
        if dist2 < smallest_dist2 {
            extended_pos = *r;
            real_pos = t;
            closest_bezier = bezier;
            smallest_dist2 = dist2;
        }
    }

    // Get the distance from current pixel "p" to bezier line.
    let extended_bezier =
        extended_pos * extended_pos * v2 + 2.0 * extended_pos * v1 + p0;
    let extended_dist = (extended_bezier - p).magnitude();
    let real_dist = smallest_dist2.sqrt();

    // Invert the vector to get distance from bezier line to "p". TODO explain
    let dir = 2.0 * v2 * real_pos + 2.0 * v1;
    let p_bezier = p - closest_bezier;
    let ortho: f32 = if p_bezier.is_zero() || dir.is_zero() {
        0.0
    } else {
        dir.normalize().cross(p_bezier.normalize())
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

fn find_quadratic_roots(a: f32, b: f32, c: f32) -> [Option<f32>; 2] {
    let discriminant = b * b - 4.0 * a * c;

    if a == 0.0 {
        if b == 0.0 {
            return [None, None];
        }
        return [Some(-c / b), None];
    }

    if discriminant < 0.0 {
        [None, None]
    } else if discriminant > 0.0 {
        let discriminant_sqrt = discriminant.sqrt();
        let a2 = 1.0 / (2.0 * a);
        // Root 1
        let x1 = -(b + discriminant_sqrt) * a2;
        // Root 2
        let x2 = (discriminant_sqrt - b) * a2;

        [Some(x1), Some(x2)]
    } else {
        let extreme_x = -0.5 * b / a;
        [Some(extreme_x), None]
    }
}

fn find_cubic_roots(a: f32, b: f32, c: f32, d: f32) -> [Option<f32>; 3] {
    if a == 0.0 {
        let roots = find_quadratic_roots(b, c, d);
        return [roots[0], roots[1], None];
    }

    // All formulas and procedures are explained at: https://mathworld.wolfram.com/CubicFormula.html

    let mut b = b / a;
    let c = c / a;
    let d = d / a;

    let q = (b * b - 3.0 * c) / 9.0; // TODO explain why we negate numerator
    let r = (2.0 * b * b * b + 27.0 * d - 9.0 * c * b) / 54.0;

    let qqq = q * q * q;
    let rr = r * r;
    let third = 1.0 / 3.0;
    b *= third;

    if rr > qqq {
        // D > 0.0
        // Then there is only one root.
        let s = -r.signum() * (r.abs() + (rr - qqq).sqrt()).cbrt();
        let x1 = (s + q / s) - b; // TODO exclain // ALSO CAN BE q/s=t WHYY??

        [Some(x1), None, None]
    } else {
        // D <= 0.0, q < 0.0
        // root1 = (2 * sqrt(-q)) * cos(theta/3) - (third * b);
        // root2 = (2 * sqrt(-q)) * cos((theta + 2*pi)/3) - (third * b);
        // root3 = (2 * sqrt(-q)) * cos((theta + 4*pi)/3) - (third * b);
        // root = m * cos((theta + ...)/3) - n;
        let q_sqrt = q.sqrt();
        let two_pi = 2.0 * PI;
        let theta = (r / q_sqrt.powi(3)).acos();
        let m = -2.0 * q_sqrt;
        let x1 = m * (theta * third).cos() - b;
        let x2 = m * ((theta + two_pi) * third).cos() - b;
        let x3 = m * ((theta - two_pi) * third).cos() - b;

        [Some(x1), Some(x2), Some(x3)]
    }
}

/// A line function.
/// - `p0` - line starting point
/// - `p1` - line ending point
/// - `t` - function parameter
#[inline]
pub fn line_fn(p0: Vector2, p1: Vector2, t: f32) -> Vector2 {
    p0 + t * (p1 - p0)
}

/// A line function.
/// - `p0` - curve starting point
/// - `p1` - curve control point
/// - `p2` - curve ending point
/// - `t` - function parameter
#[inline]
pub fn quadratic_fn(p0: Vector2, p1: Vector2, p2: Vector2, t: f32) -> Vector2 {
    p0 + 2.0 * t * (p1 - p0) + t * t * (p2 - 2.0 * p1 + p0)
}

#[test]
fn cubic_root_test() {
    let a = 1.0;
    let b = 100.4;
    let c = -100.4;
    let d = -0.29;
    let (roots, discriminant, q, r) = test_find_cubic_roots(a, b, c, d);

    assert!(q < 0.0);
    assert!(r < 0.0);
    assert!(discriminant < 0.0);
}

#[test]
fn cubic_root_test2() {
    let a = 1.0;
    let b = -1.0;
    let c = -1.6;
    let d = 2.5;
    let (roots, discriminant, q, r) = test_find_cubic_roots(a, b, c, d);

    assert!(discriminant > 0.0);
}

fn test_find_cubic_roots(
    _a: f32,
    _b: f32,
    _c: f32,
    _d: f32,
) -> (Vec<f32>, f32, f32, f32) {
    let b = _b / _a;
    let c = _c / _a;
    let d = _d / _a;

    let q = (3.0 * c - b * b) / 9.0; // TODO explain why we negate numerator
    let r = (9.0 * b * c - 27.0 * d - 2.0 * b * b * b) / 54.0;
    let qqq = q * q * q;
    let discriminant = qqq + r * r;
    let third = 1.0 / 3.0;

    if discriminant > 0.0 {
        // Then there is only one root.
        let s = (r + discriminant.sqrt()).cbrt();
        let t = (r - discriminant.sqrt()).cbrt();
        if s.is_nan() {
            println!("s: {}", s);
        }
        if t.is_nan() {
            println!("t: {}", t);
        }
        /*let temp = ((discriminant).sqrt() + r.abs()).powf(third);
        let sign = r.signum();
        let r = -sign * (temp + q / temp) - third * b;*/
        let x1 = (s + t) - third * b;

        return (vec![x1], discriminant, q, r);
    }
    let two_pi = 2.0 * PI;
    let theta = (r / (-qqq).sqrt()).acos();
    let m = 2.0 * (-q).sqrt();
    let n = b * third;
    let x1 = m * (theta / 3.0).cos() - n;
    let x2 = m * ((theta + two_pi) / 3.0).cos() - n;
    let x3 = m * ((theta + 2.0 * two_pi) / 3.0).cos() - n;
    if x1.is_nan() {
        println!("x1 je nan D <= 0.0")
    }
    if x2.is_nan() {
        println!("x2 je nan D <= 0.0")
    }
    if x3.is_nan() {
        println!("x3 je nan D <= 0.0")
    }

    (vec![x1, x2, x3], discriminant, q, r)
}
