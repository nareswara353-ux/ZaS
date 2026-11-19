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
fn test_strict_zero_allocation() {
    let before = ALLOC_COUNT.load(Ordering::SeqCst);
    
    let mut win = ZeroAllocWindow::<1024>::new();
    for i in 0..100_000 {
        win.push_and_slide(i as f64);
    }
    
    let after = ALLOC_COUNT.load(Ordering::SeqCst);
    assert_eq!(after - before, 0, "Ditemukan alokasi heap!");
    println!("\n[PASSED] Heap allocations count during 100k ops: {}", after - before);
}
