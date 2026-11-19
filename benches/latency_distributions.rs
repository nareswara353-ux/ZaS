use criterion::{black_box, criterion_group, criterion_main, Criterion};
use zero_alloc_core::window::ZeroAllocWindow;

fn bench_stream_distributions(c: &mut Criterion) {
    let mut group = c.benchmark_group("latency_distributions_w1024");

    // 1. Uniform Stream (Baseline)
    group.bench_function("1_uniform_stream", |b| {
        let mut win = ZeroAllocWindow::<1024>::new();
        let mut i = 1u64;
        b.iter(|| {
            let val = black_box(1e8 + (i as f64 * 1e-5));
            i += 1;
            black_box(win.push_and_slide(val))
        });
    });

    // 2. Block-wise Stream / Test 2 (Transisi magnitudo per 2048 sampel)
    group.bench_function("2_blockwise_stream", |b| {
        let mut win = ZeroAllocWindow::<1024>::new();
        let mut i = 1u64;
        b.iter(|| {
            let block = ((i - 1) / 2048) % 2;
            let scale = if block == 0 { 1e8 } else { 1e-3 };
            let val = black_box(scale * (1.0 + 0.5 * (i as f64).sin()));
            i += 1;
            black_box(win.push_and_slide(val))
        });
    });

    // 3. Worst-Case Random Branch-Flipping Stream (Berganti magnitudo 1e8 vs 1e-8 SETIAP elemen)
    group.bench_function("3_worst_case_branch_flipping", |b| {
        let mut win = ZeroAllocWindow::<1024>::new();
        let mut i = 1u64;
        b.iter(|| {
            let val = black_box(if i % 2 == 0 { 1e8 } else { 1e-8 });
            i += 1;
            black_box(win.push_and_slide(val))
        });
    });

    group.finish();
}

criterion_group!(benches, bench_stream_distributions);
criterion_main!(benches);
