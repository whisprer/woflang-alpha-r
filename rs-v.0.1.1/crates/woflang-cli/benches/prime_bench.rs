//! Criterion benchmarks for Woflang prime checking.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use woflang_core::InterpreterContext;
use woflang_runtime::Interpreter;

fn create_interp() -> Interpreter {
    let mut interp = Interpreter::new();
    woflang_ops::register_all(&mut interp);
    interp
}

fn bench_prime_check(c: &mut Criterion) {
    let mut group = c.benchmark_group("prime_check");

    let test_numbers: &[(u64, &str)] = &[
        (97, "small_prime"),
        (9973, "medium_prime"),
        (2_147_483_647, "mersenne_31"),
        (1_000_000_007, "billion_prime"),
        (561, "carmichael"),
        (1_000_000_000, "large_composite"),
    ];

    for (num, name) in test_numbers {
        group.bench_with_input(BenchmarkId::new("interpreter", name), num, |b, &n| {
            let mut interp = create_interp();
            let cmd = format!("{n} prime_check");
            b.iter(|| {
                interp.clear();
                interp.exec_line(black_box(&cmd)).unwrap();
                interp.stack().peek().unwrap().as_bool()
            });
        });
    }

    group.finish();
}

fn bench_arithmetic(c: &mut Criterion) {
    let mut group = c.benchmark_group("arithmetic");

    group.bench_function("add_1000", |b| {
        let mut interp = create_interp();
        b.iter(|| {
            interp.clear();
            for i in 0..1000 {
                interp.exec_line(&format!("{i}")).unwrap();
            }
            for _ in 0..999 {
                interp.exec_line("+").unwrap();
            }
            black_box(interp.stack().peek().unwrap().as_integer().unwrap())
        });
    });

    group.bench_function("mul_chain", |b| {
        let mut interp = create_interp();
        b.iter(|| {
            interp.clear();
            interp.exec_line("2 2 * 2 * 2 * 2 * 2 * 2 * 2 * 2 * 2 *").unwrap();
            black_box(interp.stack().peek().unwrap().as_integer().unwrap())
        });
    });

    group.finish();
}

fn bench_stack_ops(c: &mut Criterion) {
    let mut group = c.benchmark_group("stack_ops");

    group.bench_function("dup_drop_1000", |b| {
        let mut interp = create_interp();
        interp.exec_line("42").unwrap();
        b.iter(|| {
            for _ in 0..1000 {
                interp.exec_line("dup").unwrap();
            }
            for _ in 0..1000 {
                interp.exec_line("drop").unwrap();
            }
            black_box(interp.stack().len())
        });
    });

    group.finish();
}

criterion_group!(benches, bench_prime_check, bench_arithmetic, bench_stack_ops);
criterion_main!(benches);
