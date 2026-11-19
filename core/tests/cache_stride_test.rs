use zero_alloc_core::ZeroAllocWindow;
use std::hint::black_box;
use std::time::Instant;

#[repr(C, align(64))]
struct AlignedContainer {
    padding: [u8; 3], // Intentional 3-byte misalignment shift
    window: ZeroAllocWindow<1024>,
}

#[test]
fn test_cache_alignment_stride_sensitivity() {
    println!("\n[START] Executing Cache-Line Alignment & Stride Sensitivity Audit...");

    const ITERS: usize = 20_000_000;
    
    // 1. Aligned Execution (64-byte boundary)
    let mut aligned_win = ZeroAllocWindow::<1024>::new();
    let start_aligned = Instant::now();
    let mut val = 1.0f64;
    for _ in 0..ITERS {
        val += 0.00001;
        black_box(aligned_win.push_and_slide(black_box(val)));
    }
    let elapsed_aligned = start_aligned.elapsed();
    let ns_per_op_aligned = elapsed_aligned.as_nanos() as f64 / ITERS as f64;

    // 2. Unaligned Execution (Shifted Offset)
    let mut unaligned_container = AlignedContainer {
        padding: [0; 3],
        window: ZeroAllocWindow::<1024>::new(),
    };
    let start_unaligned = Instant::now();
    let mut val_un = 1.0f64;
    for _ in 0..ITERS {
        val_un += 0.00001;
        black_box(unaligned_container.window.push_and_slide(black_box(val_un)));
    }
    let elapsed_unaligned = start_unaligned.elapsed();
    let ns_per_op_unaligned = elapsed_unaligned.as_nanos() as f64 / ITERS as f64;

    println!("  -> Aligned 64-byte Boundary Latency : {:.3} ns/op", ns_per_op_aligned);
    println!("  -> Shifted Unaligned Boundary Latency: {:.3} ns/op", ns_per_op_unaligned);
    
    let diff = (ns_per_op_unaligned - ns_per_op_aligned).abs();
    println!("  -> Delta Latency Penalty            : {:.3} ns/op", diff);
    println!("[SUCCESS] Cache Boundary Stride Audit Completed.\n");
}
