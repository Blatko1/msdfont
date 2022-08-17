use std::f32::consts::PI;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(crit: &mut Criterion) {
    // Cubic constants
    let a = black_box(1.0);
    let b = black_box(100.4);
    let c = black_box(-100.4);
    let d = black_box(-0.29);
    crit.bench_function("old version cubic root finder", |bencher| {
        bencher.iter(|| old_version_find_cubic(a, b, c, d))
    });
    crit.bench_function("sdf-test cubic root finder", |bencher| {
        bencher.iter(|| sdf_test_cubic(a, b, c, d))
    });
    crit.bench_function("msdfgen solve_cubic_norm", |bencher| {
        bencher.iter(|| msdfgen_solve_cubic_norm(b, c, d, a))
    });
    crit.bench_function("fast cubic root finder", |bencher| {
        bencher.iter(|| fast_find_cubic(a, b, c, d))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

fn fast_find_cubic(a: f32, b: f32, c: f32, d: f32) -> [Option<f32>; 3] {
    let mut b = b / a;
    let c = c / a;
    let d = d / a;

    let q = (b * b - 3.0 * c) / 9.0;
    let r = (b * 2.0 * b * b - 9.0 * c * b + 27.0 * d) / 54.0;

    let qqq = q * q * q;
    let r2 = r * r;
    let third = 1.0 / 3.0;
    b *= third;

    if r2 > qqq {
        let s = -r.signum() * (r.abs() + (r2 - qqq).sqrt()).cbrt();
        let x1 = (s + q / s) - b;
        return [Some(x1), None, None];
    }
    let q_sqrt = q.sqrt();
    let two_pi = 2.0 * PI;
    let theta = (r / q_sqrt.powi(3)).acos();
    let m = -2.0 * q_sqrt;
    let x1 = m * (theta * third).cos() - b;
    let x2 = m * ((theta + two_pi) * third).cos() - b;
    let x3 = m * ((theta - two_pi) * third).cos() - b;
    return [Some(x1), Some(x2), Some(x3)];
}

pub fn msdfgen_solve_cubic_norm(a: f32, b: f32, c: f32, div: f32) -> [f32; 3] {
    let mut a = a / div;
    let b = b / div;
    let c = c / div;
    let mut result = [0.0; 3];
    let a2 = a * a;
    let mut q = (a2 - 3.0 * b) / 9.0;
    let r = (a * (2.0 * a2 - 9.0 * b) + 27.0 * c) / 54.0;
    let r2 = r * r;
    let q3 = q * q * q;
    let mut result_a;
    let result_b;
    if r2 < q3 {
        let mut t = r / q3.sqrt();
        if t < -1.0 {
            t = -1.0;
        }
        if t > 1.0 {
            t = 1.0;
        }
        t = t.acos();
        a /= 3.0;
        q = -2.0 * q.sqrt();
        result[0] = q * (t / 3.0).cos() - a;
        result[1] = q * ((t + 2.0 * std::f32::consts::PI) / 3.0).cos() - a;
        result[2] = q * ((t - 2.0 * std::f32::consts::PI) / 3.0).cos() - a;
        return result;
    } else {
        result_a = -(r.abs() + (r2 - q3).sqrt()).powf(1.0 / 3.0);
        if r < 0.0 {
            result_a = -result_a
        };
        result_b = if result_a == 0.0 { 0.0 } else { q / result_a };
        a /= 3.0;
        result[0] = (result_a + result_b) - a;
        result[1] = -0.5 * (result_a + result_b) - a;
        result[2] = 0.5 * 3.0f32.sqrt() * (result_a - result_b);
        if result[2].abs() < f32::EPSILON {
            return result;
        }
        return result;
    }
}

fn old_version_find_cubic(
    _a: f32,
    _b: f32,
    _c: f32,
    _d: f32,
) -> (Vec<f32>, f32, f32, f32) {
    let b = _b / _a;
    let c = _c / _a;
    let d = _d / _a;

    let q = (3.0 * c - b * b) / 9.0;
    let r = (9.0 * b * c - 27.0 * d - 2.0 * b * b * b) / 54.0;
    let qqq = q * q * q;
    let discriminant = qqq + r * r;
    let third = 1.0 / 3.0;

    if discriminant > 0.0 {
        let s = (r + discriminant.sqrt()).cbrt();
        let t = (r - discriminant.sqrt()).cbrt();
        if s.is_nan() {
            println!("s: {}", s);
        }
        if t.is_nan() {
            println!("t: {}", t);
        }
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

    return (vec![x1, x2, x3], discriminant, q, r);
}

pub fn sdf_test_cubic(
    a: f32,
    b: f32,
    c: f32,
    d: f32,
) -> (Option<f32>, Option<f32>, Option<f32>) {
    let b = b / a;
    let c = c / a;
    let d = d / a;

    let q = (b * b - 3.0 * c) / 9.0;
    let r = (2.0 * b * b * b - 9.0 * b * c + 27.0 * d) / 54.0;
    let qqq = q * q * q;
    let discriminant = qqq - r * r;
    let third = 1.0 / 3.0;

    if discriminant >= 0.0 {
        let twopi = 2.0 * PI;
        let theta = (r / qqq.sqrt()).acos();
        let mult = -2.0 * q.sqrt();
        let add = -b * third;
        let r0 = mult * (third * theta).cos() + add;
        let r1 = mult * (third * (theta + twopi)).cos() + add;
        let r2 = mult * (third * (theta + twopi + twopi)).cos() + add;
        return (Some(r0), Some(r1), Some(r2));
    }

    let temp = ((-discriminant).sqrt() + r.abs()).powf(third);
    let sign = r.signum();
    let r = -sign * (temp + q / temp) - third * b;
    return (Some(r), None, None);
}
