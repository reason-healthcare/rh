//! Benchmarks for the CQL compilation pipeline.
//!
//! Covers 10.1 (baseline benchmarks for compilation) and 10.2 (regression
//! tracking).  Run with:
//!
//! ```text
//! cargo bench -p rh-cql --bench bench_compile
//! ```

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rh_cql::{compile, compile_to_json, validate, CompilerOptions};

// ---------------------------------------------------------------------------
// Fixture CQL strings
// ---------------------------------------------------------------------------

/// Minimal library — 1 define.
const SMALL_CQL: &str = r#"
library Small version '1.0.0'
define Answer: 42
"#;

/// Medium library — 10 defines covering common CQL constructs.
const MEDIUM_CQL: &str = r#"
library Medium version '1.0.0'
define IntValue: 1 + 2
define BoolTrue: true
define BoolFalse: not true
define StringConcat: 'hello' + ' ' + 'world'
define NullCheck: null is null
define ListSize: Count({1, 2, 3, 4, 5})
define IfExpr: if true then 1 else 2
define Arithmetic: (10 - 3) * (2 + 1)
define Comparison: 7 > 3 and 5 < 9
define Chained: (1 + 2) * (3 + 4) - 1
"#;

/// Generates a large library CQL string with `n` integer-define statements.
fn large_cql(n: usize) -> String {
    let mut s = String::from("library Large version '1.0.0'\n");
    for i in 0..n {
        s.push_str(&format!("define Expr{}: {} + {}\n", i, i, i + 1));
    }
    s
}

// ---------------------------------------------------------------------------
// Benchmarks
// ---------------------------------------------------------------------------

fn bench_compile_small(c: &mut Criterion) {
    c.bench_function("compile/small", |b| {
        b.iter(|| compile(black_box(SMALL_CQL), None).unwrap())
    });
}

fn bench_compile_medium(c: &mut Criterion) {
    c.bench_function("compile/medium", |b| {
        b.iter(|| compile(black_box(MEDIUM_CQL), None).unwrap())
    });
}

fn bench_compile_by_size(c: &mut Criterion) {
    let sizes = [10usize, 50, 100];
    let mut group = c.benchmark_group("compile/large");
    for &n in &sizes {
        let cql = large_cql(n);
        group.bench_with_input(BenchmarkId::new("defines", n), &cql, |b, src| {
            b.iter(|| compile(black_box(src.as_str()), None).unwrap())
        });
    }
    group.finish();
}

fn bench_compile_to_json(c: &mut Criterion) {
    c.bench_function("compile/to_json_pretty", |b| {
        b.iter(|| compile_to_json(black_box(MEDIUM_CQL), None, true).unwrap())
    });

    c.bench_function("compile/to_json_compact", |b| {
        b.iter(|| compile_to_json(black_box(MEDIUM_CQL), None, false).unwrap())
    });
}

fn bench_validate(c: &mut Criterion) {
    c.bench_function("compile/validate_only", |b| {
        b.iter(|| validate(black_box(MEDIUM_CQL), None).unwrap())
    });
}

fn bench_compile_options(c: &mut Criterion) {
    let debug_opts = CompilerOptions::debug();
    let cql = MEDIUM_CQL;
    let mut group = c.benchmark_group("compile/options");
    group.bench_function("default", |b| {
        b.iter(|| compile(black_box(cql), None).unwrap())
    });
    group.bench_function("debug", |b| {
        b.iter(|| compile(black_box(cql), Some(debug_opts.clone())).unwrap())
    });
    group.finish();
}

criterion_group!(
    benches,
    bench_compile_small,
    bench_compile_medium,
    bench_compile_by_size,
    bench_compile_to_json,
    bench_validate,
    bench_compile_options,
);
criterion_main!(benches);
