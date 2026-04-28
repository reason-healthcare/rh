use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rh_fsh::compile_fsh;

fn make_fsh_input(n: usize) -> String {
    let mut buf = String::new();
    for i in 0..n {
        buf.push_str(&format!(
            "Profile: MyProfile{i}\nParent: Observation\nTitle: \"Profile {i}\"\n\n"
        ));
    }
    buf
}

fn bench_small(c: &mut Criterion) {
    let input = make_fsh_input(10);
    c.bench_function("compile_small", |b| {
        b.iter(|| compile_fsh(black_box(&input), "bench").ok())
    });
}

fn bench_medium(c: &mut Criterion) {
    let input = make_fsh_input(100);
    c.bench_function("compile_medium", |b| {
        b.iter(|| compile_fsh(black_box(&input), "bench").ok())
    });
}

fn bench_large(c: &mut Criterion) {
    let input = make_fsh_input(1000);
    c.bench_function("compile_large", |b| {
        b.iter(|| compile_fsh(black_box(&input), "bench").ok())
    });
}

criterion_group!(benches, bench_small, bench_medium, bench_large);
criterion_main!(benches);
