//! Benchmarks for the CQL evaluation engine.
//!
//! Covers 10.1 (baseline benchmarks for evaluation) and 10.2 (regression
//! tracking).  Pre-compilation is performed in setup so only evaluation time
//! is measured.  Run with:
//!
//! ```text
//! cargo bench -p rh-cql --bench bench_eval
//! ```

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rh_cql::{
    compile_with_model, evaluate_elm, CqlDateTime, EvalContextBuilder, FixedClock, Value,
};

// ---------------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------------

fn bench_clock() -> FixedClock {
    FixedClock::new(CqlDateTime {
        year: 2024,
        month: Some(6),
        day: Some(15),
        hour: Some(10),
        minute: Some(30),
        second: Some(0),
        millisecond: None,
        offset_seconds: None,
    })
}

fn default_ctx() -> rh_cql::EvalContext {
    EvalContextBuilder::new(bench_clock()).build()
}

/// Compile a standalone (no FHIR model) CQL snippet; panic on any error.
fn compile_no_model(cql: &str) -> rh_cql::elm::Library {
    let result = compile_with_model(cql, None, None).expect("compile failed");
    assert!(
        result.errors.is_empty(),
        "compilation errors: {:?}",
        result.errors
    );
    result.library
}

// ---------------------------------------------------------------------------
// Benchmarks
// ---------------------------------------------------------------------------

fn bench_eval_arithmetic(c: &mut Criterion) {
    let lib = compile_no_model("library T define X: (10 + 20) * 3 - 5");
    let ctx = default_ctx();
    c.bench_function("eval/arithmetic", |b| {
        b.iter(|| evaluate_elm(black_box(&lib), black_box("X"), black_box(&ctx)).unwrap())
    });
}

fn bench_eval_boolean_logic(c: &mut Criterion) {
    let lib = compile_no_model("library T define X: true and not false or (1 < 2 and 3 >= 3)");
    let ctx = default_ctx();
    c.bench_function("eval/boolean_logic", |b| {
        b.iter(|| evaluate_elm(black_box(&lib), black_box("X"), black_box(&ctx)).unwrap())
    });
}

fn bench_eval_conditional(c: &mut Criterion) {
    let lib = compile_no_model("library T define X: if (1 + 1 = 2) then 'yes' else 'no'");
    let ctx = default_ctx();
    c.bench_function("eval/conditional", |b| {
        b.iter(|| evaluate_elm(black_box(&lib), black_box("X"), black_box(&ctx)).unwrap())
    });
}

fn bench_eval_string_concat(c: &mut Criterion) {
    let lib = compile_no_model("library T define X: 'foo' + 'bar' + 'baz' + 'qux'");
    let ctx = default_ctx();
    c.bench_function("eval/string_concat", |b| {
        b.iter(|| evaluate_elm(black_box(&lib), black_box("X"), black_box(&ctx)).unwrap())
    });
}

fn bench_eval_list_count(c: &mut Criterion) {
    let lib = compile_no_model("library T define X: Count({1,2,3,4,5,6,7,8,9,10})");
    let ctx = default_ctx();
    c.bench_function("eval/list_count", |b| {
        b.iter(|| evaluate_elm(black_box(&lib), black_box("X"), black_box(&ctx)).unwrap())
    });
}

fn bench_eval_repeated_lookup(c: &mut Criterion) {
    // Measures the overhead of calling evaluate_elm many times on the same
    // library — exercises the expression-lookup path in the eval engine.
    let cql = {
        let mut s = String::from("library T\n");
        for i in 0..20 {
            s.push_str(&format!("define E{}: {} + {}\n", i, i, i + 1));
        }
        s
    };
    let lib = compile_no_model(&cql);
    let ctx = default_ctx();

    let sizes = [1usize, 5, 20];
    let mut group = c.benchmark_group("eval/repeated_lookup");
    for &n in &sizes {
        group.bench_with_input(BenchmarkId::new("exprs", n), &n, |b, &count| {
            b.iter(|| {
                for i in 0..count {
                    let name = format!("E{}", i);
                    evaluate_elm(black_box(&lib), black_box(name.as_str()), black_box(&ctx))
                        .unwrap();
                }
            })
        });
    }
    group.finish();
}

fn bench_eval_null_handling(c: &mut Criterion) {
    let lib = compile_no_model("library T define X: null is null");
    let ctx = default_ctx();
    c.bench_function("eval/null_is_null", |b| {
        b.iter(|| {
            let v = evaluate_elm(black_box(&lib), black_box("X"), black_box(&ctx)).unwrap();
            assert_eq!(v, Value::Boolean(true));
        })
    });
}

criterion_group!(
    benches,
    bench_eval_arithmetic,
    bench_eval_boolean_logic,
    bench_eval_conditional,
    bench_eval_string_concat,
    bench_eval_list_count,
    bench_eval_repeated_lookup,
    bench_eval_null_handling,
);
criterion_main!(benches);
