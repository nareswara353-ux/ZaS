use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};
use zero_alloc_core::window::ZeroAllocWindow;

struct AllocationCounter;
static ALLOC_COUNT: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for AllocationCounter {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOC_COUNT.fetch_add(1, Ordering::SeqCst);
        System.alloc(layout)
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
    }
}

#[global_allocator]
static A: AllocationCounter = AllocationCounter;

#[test]
fn test_zero_alloc_10m() {
    let before = ALLOC_COUNT.load(Ordering::SeqCst);
    let mut win = ZeroAllocWindow::<1024>::new();
    for i in 0..10_000_000 {
        win.push_and_slide(i as f64);
    }
    let after = ALLOC_COUNT.load(Ordering::SeqCst);
    assert_eq!(after - before, 0, "Ditemukan alokasi heap!");
    println!("\n[PASSED 10M] Heap allocations count: {}", after - before);
}

#[test]
fn perf_uniform_stream() {
    let mut win = ZeroAllocWindow::<1024>::new();
    let mut sum = 0.0;
    for i in 1..=10_000_000 {
        let val = 1e8 + (i as f64 * 1e-5);
        let (s, _) = win.push_and_slide(val);
        sum += s;
    }
    std::hint::black_box(sum);
}

#[test]
fn perf_blockwise_stream() {
    let mut win = ZeroAllocWindow::<1024>::new();
    let mut sum = 0.0;
    for i in 1..=10_000_000 {
        let block = ((i - 1) / 2048) % 2;
        let scale = if block == 0 { 1e8 } else { 1e-3 };
        let val = scale * (1.0 + 0.5 * (i as f64).sin());
        let (s, _) = win.push_and_slide(val);
        sum += s;
    }
    std::hint::black_box(sum);
}

#[test]
fn perf_adversarial_stream() {
    let mut win = ZeroAllocWindow::<1024>::new();
    let mut sum = 0.0;
    let mut state: u64 = 0x8888888888888888;
    
    let mut next_u64 = || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };

    let mut last_a = 0.0;
    for i in 0..10_000_000 {
        let val = if i % 2 == 0 {
            let sign = if (next_u64() & 1) == 0 { 1.0 } else { -1.0 };
            let u = ((next_u64() & 0xFFFFFFFF) as f64 / 4294967295.0) * 2.0 - 1.0;
            let mag = (10.0f64).powf(u);
            last_a = sign * mag;
            last_a
        } else {
            let u = ((next_u64() & 0xFFFFFFFF) as f64 / 4294967295.0) * 2.0 - 1.0;
            -last_a * (1.0 + 1e-6 * u)
        };
        let (s, _) = win.push_and_slide(val);
        sum += s;
    }
    std::hint::black_box(sum);
}
