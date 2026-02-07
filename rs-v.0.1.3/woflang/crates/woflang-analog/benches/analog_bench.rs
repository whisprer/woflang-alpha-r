//! Benchmarks for analog computing operations.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use woflang_analog::prelude::*;

fn bench_basic_math(c: &mut Criterion) {
    reset_analog_mode();
    set_analog_mode(AnalogMode::Int201);

    c.bench_function("analog_add", |b| {
        b.iter(|| analog_add(black_box(80.0), black_box(50.0)))
    });

    c.bench_function("analog_mul", |b| {
        b.iter(|| analog_mul(black_box(50.0), black_box(50.0)))
    });

    c.bench_function("clamp_analog", |b| {
        b.iter(|| clamp_analog(black_box(150.0)))
    });
}

fn bench_trig(c: &mut Criterion) {
    reset_analog_mode();
    set_analog_mode(AnalogMode::FloatUnit);

    c.bench_function("analog_sin", |b| {
        b.iter(|| analog_sin(black_box(1.0)))
    });

    c.bench_function("analog_tanh", |b| {
        b.iter(|| analog_tanh(black_box(0.5)))
    });
}

fn bench_linear(c: &mut Criterion) {
    reset_analog_mode();
    set_analog_mode(AnalogMode::Int201);

    c.bench_function("analog_dot_2d", |b| {
        b.iter(|| {
            analog_dot_2d(
                black_box(3.0),
                black_box(4.0),
                black_box(1.0),
                black_box(2.0),
            )
        })
    });

    c.bench_function("analog_magnitude_3d", |b| {
        b.iter(|| analog_magnitude_3d(black_box(3.0), black_box(4.0), black_box(5.0)))
    });

    c.bench_function("analog_normalize_2d", |b| {
        b.iter(|| analog_normalize_2d(black_box(3.0), black_box(4.0)))
    });
}

fn bench_batch(c: &mut Criterion) {
    reset_analog_mode();
    set_analog_mode(AnalogMode::Int201);

    let a: Vec<f64> = (0..1000).map(|i| i as f64 * 0.1).collect();
    let b: Vec<f64> = (0..1000).map(|i| (1000 - i) as f64 * 0.1).collect();

    c.bench_function("batch_add_1000", |b_| {
        b_.iter(|| {
            use woflang_analog::math::batch_add;
            batch_add(black_box(&a), black_box(&b))
        })
    });
}

criterion_group!(benches, bench_basic_math, bench_trig, bench_linear, bench_batch);
criterion_main!(benches);
