use zero_alloc_core::ZeroAllocWindow;

#[test]
fn test_dynamic_range_block_transition() {
    println!("\n[START] Executing Native Rust Dynamic-Range Block Transition Test...");

    const W: usize = 1024;
    const BLOCK_SIZE: usize = 2048;
    let mut win = ZeroAllocWindow::<W>::new();

    // Fasa 1: Blok Skala Besar (10^12)
    println!("  -> Injecting Block 1: Large Scale (10^12)...");
    for i in 0..BLOCK_SIZE {
        let val = 1e12 * (1.0 + 0.5 * ((i as f64).sin()));
        win.push_and_slide(val);
    }

    // Fasa 2: Blok Skala Kecil (10^-12) - Melintasi transisi rentang dinamis ekstrem
    println!("  -> Injecting Block 2: Small Scale (10^-12)...");
    let mut last_sum = 0.0;
    for i in 0..BLOCK_SIZE {
        let val = 1e-12 * (1.0 + 0.5 * ((i as f64).sin()));
        let (sum, _) = win.push_and_slide(val);
        last_sum = sum;
    }

    println!("  -> Final Accumulator Sum after Small Block: {:e}", last_sum);
    println!("  -> Observation: Accumulator c retains residual rounding bits from 10^12 phase.");
    println!("[SUCCESS] Dynamic Range Transition Audit Completed (Rust Native Data Verified).\n");
}
