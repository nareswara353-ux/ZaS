use criterion::{black_box, criterion_group, criterion_main, Criterion};
use zero_alloc_core::ZeroAllocWindow;
use zero_alloc_ffi::{
    zero_alloc_window_1024_free, zero_alloc_window_1024_new, zero_alloc_window_1024_push_and_slide,
};

fn bench_ffi_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("FFI_Bridge_Latency");

    // Baseline Native Rust (W=1024)
    group.bench_function("Native_Rust_O1/1024", |b| {
        let mut win = ZeroAllocWindow::<1024>::new();
        let mut val = 1.0f64;
        b.iter(|| {
            val += 0.001;
            black_box(win.push_and_slide(black_box(val)))
        });
    });

    // FFI C-ABI Wrapper (W=1024)
    group.bench_function("FFI_C_ABI_O1/1024", |b| {
        let win_ptr = zero_alloc_window_1024_new();
        let mut val = 1.0f64;
        b.iter(|| {
            val += 0.001;
            black_box(zero_alloc_window_1024_push_and_slide(win_ptr, black_box(val)))
        });
        zero_alloc_window_1024_free(win_ptr);
    });

    group.finish();
}

criterion_group!(benches, bench_ffi_comparison);
criterion_main!(benches);
