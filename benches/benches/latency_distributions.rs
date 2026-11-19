use criterion::{black_box, criterion_group, criterion_main, Criterion};
use zero_alloc_core::window::ZeroAllocWindow;

fn bench_stream_distributions(c: &mut Criterion) {
    let mut group = c.benchmark_group("latency_distributions_w1024");
    const MASK: usize = 65535;

    // 1. Pre-generated Uniform Stream
    let mut uniform_data = [0.0f64; 65536];
    for i in 0..65536 {
        uniform_data[i] = 1e8 + (i as f64 * 1e-5);
    }

    // 2. Pre-generated Block-wise Stream (Test 2 tanpa sin() di dalam loop)
    let mut blockwise_data = [0.0f64; 65536];
    for i in 0..65536 {
        let block = (i / 2048) % 2;
        let scale = if block == 0 { 1e8 } else { 1e-3 };
        blockwise_data[i] = scale * (1.0 + 0.5 * (i as f64).sin());
    }

    // 3. Pre-generated True Antithetic Branch-Flipping Stream
    // Elemen berganti tanda +/- sekitar 1e8 sehingga sum window bergerak di sekitar 0,
    // memaksa kondisi |sum| >= |val| membalik secara acak terus-menerus.
    let mut branch_flip_data = [0.0f64; 65536];
    let mut state: u64 = 0x123456789ABCDEF0;
    for i in 0..65536 {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        let rand_factor = ((state & 0xFFFF) as f64 / 65535.0) - 0.5; // [-0.5, 0.5]
        let sign = if i % 2 == 0 { 1.0 } else { -1.0 };
        branch_flip_data[i] = sign * 1e8 * (1.0 + rand_factor * 0.05);
    }

    // --- BENCHMARKS (Mengukur MURNI latency engine via array index lookup) ---

    group.bench_function("1_uniform_stream", |b| {
        let mut win = ZeroAllocWindow::<1024>::new();
        let mut idx = 0usize;
        b.iter(|| {
            let val = black_box(uniform_data[idx & MASK]);
            idx += 1;
            black_box(win.push_and_slide(val))
        });
    });

    group.bench_function("2_blockwise_stream", |b| {
        let mut win = ZeroAllocWindow::<1024>::new();
        let mut idx = 0usize;
        b.iter(|| {
            let val = black_box(blockwise_data[idx & MASK]);
            idx += 1;
            black_box(win.push_and_slide(val))
        });
    });

    group.bench_function("3_true_branch_flipping", |b| {
        let mut win = ZeroAllocWindow::<1024>::new();
        let mut idx = 0usize;
        b.iter(|| {
            let val = black_box(branch_flip_data[idx & MASK]);
            idx += 1;
            black_box(win.push_and_slide(val))
        });
    });

    group.finish();
}

criterion_group!(benches, bench_stream_distributions);
criterion_main!(benches);
