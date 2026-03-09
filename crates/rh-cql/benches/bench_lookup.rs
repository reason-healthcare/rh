//! Benchmarks for `CompiledLibrary` definition lookup.
//!
//! Covers 10.3 (verify measurable improvements for large library lookup) and
//! 10.2 (regression tracking).  Compares indexed O(1) `CompiledLibrary` lookups
//! against a linear-scan baseline to confirm the refactor advantage.  Run with:
//!
//! ```text
//! cargo bench -p rh-cql --bench bench_lookup
//! ```

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rh_cql::{
    elm::{ExpressionDef, ExpressionDefs, Library, StatementDef, VersionedIdentifier},
    CompiledLibrary,
};

// ---------------------------------------------------------------------------
// Fixture helpers
// ---------------------------------------------------------------------------

/// Build an ELM `Library` with `n` expression definitions named `Expr0` …
/// `Expr{n-1}`.
fn build_elm_library(n: usize) -> Library {
    let defs: Vec<StatementDef> = (0..n)
        .map(|i| {
            StatementDef::Expression(ExpressionDef {
                name: Some(format!("Expr{}", i)),
                ..Default::default()
            })
        })
        .collect();

    Library {
        identifier: Some(VersionedIdentifier {
            id: Some("BenchLib".to_string()),
            version: Some("1.0.0".to_string()),
            ..Default::default()
        }),
        statements: Some(ExpressionDefs { defs }),
        ..Default::default()
    }
}

/// Linear scan helper — simulates pre-refactor O(n) lookup for comparison.
fn linear_get_expression<'a>(lib: &'a Library, name: &str) -> Option<&'a ExpressionDef> {
    lib.statements
        .as_ref()?
        .defs
        .iter()
        .find_map(|def| match def {
            StatementDef::Expression(e) if e.name.as_deref() == Some(name) => Some(e),
            _ => None,
        })
}

// ---------------------------------------------------------------------------
// Indexed lookup benchmarks (CompiledLibrary)
// ---------------------------------------------------------------------------

fn bench_indexed_lookup_by_size(c: &mut Criterion) {
    let sizes = [10usize, 100, 500];
    let mut group = c.benchmark_group("lookup/indexed");

    for &n in &sizes {
        let compiled = CompiledLibrary::new(build_elm_library(n));
        let last_name = format!("Expr{}", n - 1);

        // Lookup the last-inserted expression — worst case for a linear scan.
        group.bench_with_input(BenchmarkId::new("last_of", n), &compiled, |b, lib| {
            b.iter(|| lib.get_expression(black_box(last_name.as_str())))
        });
    }
    group.finish();
}

fn bench_indexed_lookup_positions(c: &mut Criterion) {
    const N: usize = 200;
    let compiled = CompiledLibrary::new(build_elm_library(N));
    let mut group = c.benchmark_group("lookup/indexed/position");

    let cases = [
        ("first", "Expr0"),
        ("middle", "Expr100"),
        ("last", "Expr199"),
        ("missing", "ExprNone"),
    ];

    for (label, name) in cases {
        group.bench_function(label, |b| {
            b.iter(|| compiled.get_expression(black_box(name)))
        });
    }
    group.finish();
}

// ---------------------------------------------------------------------------
// Linear-scan baseline benchmarks (pre-refactor simulation)
// ---------------------------------------------------------------------------

fn bench_linear_scan_by_size(c: &mut Criterion) {
    let sizes = [10usize, 100, 500];
    let mut group = c.benchmark_group("lookup/linear_scan");

    for &n in &sizes {
        let lib = build_elm_library(n);
        let last_name = format!("Expr{}", n - 1);

        // Lookup the last expression — O(n) scan every time.
        group.bench_with_input(BenchmarkId::new("last_of", n), &lib, |b, elm| {
            b.iter(|| linear_get_expression(black_box(elm), black_box(last_name.as_str())))
        });
    }
    group.finish();
}

// ---------------------------------------------------------------------------
// CompiledLibrary construction overhead
// ---------------------------------------------------------------------------

fn bench_compiled_library_construction(c: &mut Criterion) {
    let sizes = [10usize, 100, 500];
    let mut group = c.benchmark_group("lookup/construction");

    for &n in &sizes {
        let elm = build_elm_library(n);
        group.bench_with_input(BenchmarkId::new("defs", n), &elm, |b, lib| {
            b.iter(|| CompiledLibrary::new(black_box(lib.clone())))
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_indexed_lookup_by_size,
    bench_indexed_lookup_positions,
    bench_linear_scan_by_size,
    bench_compiled_library_construction,
);
criterion_main!(benches);
