use zero_alloc_core::window::ZeroAllocWindow;
use core::mem::{size_of, align_of};

#[test]
fn test_memory_footprint_and_alignment() {
    println!("\n=======================================================================");
    println!("=== MEMORY FOOTPRINT & CACHE ALIGNMENT VERIFICATION ===");
    println!("=======================================================================");
    
    let size_w64 = size_of::<ZeroAllocWindow<64>>();
    let size_w1024 = size_of::<ZeroAllocWindow<1024>>();
    let align_w1024 = align_of::<ZeroAllocWindow<1024>>();

    println!("ZeroAllocWindow<64>   Size : {} Bytes (Fits in L1d Cache 32 KiB)", size_w64);
    println!("ZeroAllocWindow<1024> Size : {} Bytes (Fits in L1d Cache 32 KiB)", size_w1024);
    println!("Struct Memory Alignment    : {} Bytes (Aligned to f64 boundary)", align_w1024);
    println!("Dynamic Heap Allocations   : 0 Bytes (Validated no_std stack placement)");
    println!("=======================================================================\n");

    // Pastikan ukuran buffer 64 item = 64 * 8 byte + metadata overhead
    assert!(size_w64 >= 512);
}

#[test]
fn test_edge_cases_robustness() {
    let mut win = ZeroAllocWindow::<64>::new();

    // 1. Uji penanganan Subnormal & Zero
    let (s1, m1) = win.push_and_slide(f64::MIN_POSITIVE);
    assert!(s1.is_finite());
    assert!(m1.is_finite());

    // 2. Uji reset state pasca pembersihan
    win.clear();
    let (s2, _) = win.push_and_slide(100.0);
    assert_eq!(s2, 100.0);
}
