use std::f32::consts::PI;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn sdf_benchmark(crit: &mut Criterion) {
    // Cubic constants
    let a = black_box(1.0);
    let b = black_box(100.4);
    let c = black_box(-100.4);
    let d = black_box(-0.29);
    crit.bench_function("default overlap correction", |bencher| {
        bencher.iter(|| todo!())
    });
}

criterion_group!(benches, sdf_benchmark);
criterion_main!(benches);
