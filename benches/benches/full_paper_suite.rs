use criterion::{black_box, criterion_group, criterion_main, Criterion};
use zero_alloc_core::window::ZeroAllocWindow;

// Struct Naive Sliding Window O(W) dengan jaminan eksekusi loop penambahan per sampel
pub struct NaiveWindow<const W: usize> {
    buffer: [f64; W],
    head: usize,
}

impl<const W: usize> NaiveWindow<W> {
    pub fn new() -> Self {
        Self {
            buffer: [0.0; W],
            head: 0,
        }
    }

    #[inline(never)]
    pub fn push_and_sum(&mut self, val: f64) -> f64 {
        self.buffer[self.head] = val;
        self.head = (self.head + 1) % W;
        
        // Memaksa loop akumulasi O(W) dieksekusi tanpa di-hoist oleh LLVM
        let mut sum = 0.0;
        for i in 0..W {
            sum += self.buffer[i];
        }
        sum
    }
}

fn bench_full_paper_suite(c: &mut Criterion) {
    const MASK: usize = 65535;

    // Data pre-allocation (65536 sampel)
    let mut data = [0.0f64; 65536];
    for i in 0..65536 {
        data[i] = 1e8 + (i as f64 * 1e-5);
    }

    // ------------------------------------------------------------------------
    // 1. COMPARISON MATRIX ACROSS WINDOW SIZES (Table 3)
    // ------------------------------------------------------------------------
    let mut group_comp = c.benchmark_group("SlidingWindow_Comparison");

    // W = 64
    group_comp.bench_function("ZeroAlloc_O1/64", |b| {
        let mut win = ZeroAllocWindow::<64>::new();
        let mut idx = 0usize;
        b.iter(|| {
            let val = black_box(data[idx & MASK]);
            idx += 1;
            black_box(win.push_and_slide(val))
        });
    });
    group_comp.bench_function("Naive_OW/64", |b| {
        let mut win = NaiveWindow::<64>::new();
        let mut idx = 0usize;
        b.iter(|| {
            let val = black_box(data[idx & MASK]);
            idx += 1;
            black_box(win.push_and_sum(val))
        });
    });

    // W = 1024
    group_comp.bench_function("ZeroAlloc_O1/1024", |b| {
        let mut win = ZeroAllocWindow::<1024>::new();
        let mut idx = 0usize;
        b.iter(|| {
            let val = black_box(data[idx & MASK]);
            idx += 1;
            black_box(win.push_and_slide(val))
        });
    });
    group_comp.bench_function("Naive_OW/1024", |b| {
        let mut win = NaiveWindow::<1024>::new();
        let mut idx = 0usize;
        b.iter(|| {
            let val = black_box(data[idx & MASK]);
            idx += 1;
            black_box(win.push_and_sum(val))
        });
    });

    // W = 65536
    group_comp.bench_function("ZeroAlloc_O1/65536", |b| {
        let mut win = ZeroAllocWindow::<65536>::new();
        let mut idx = 0usize;
        b.iter(|| {
            let val = black_box(data[idx & MASK]);
            idx += 1;
            black_box(win.push_and_slide(val))
        });
    });
    group_comp.bench_function("Naive_OW/65536", |b| {
        let mut win = NaiveWindow::<65536>::new();
        let mut idx = 0usize;
        b.iter(|| {
            let val = black_box(data[idx & MASK]);
            idx += 1;
            black_box(win.push_and_sum(val))
        });
    });

    group_comp.finish();
}

criterion_group!(benches, bench_full_paper_suite);
criterion_main!(benches);
