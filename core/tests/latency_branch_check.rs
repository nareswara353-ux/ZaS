use std::time::Instant;
use zero_alloc_core::window::ZeroAllocWindow;

#[test]
fn test_latency_distribution_comparison() {
    let mut win_uniform = ZeroAllocWindow::<1024>::new();
    let mut win_test2 = ZeroAllocWindow::<1024>::new();

    const ITERS: usize = 10_000_000;

    // 1. Benchmark Uniform / Monotonic Stream
    let start_uniform = Instant::now();
    for i in 1..=ITERS {
        let val = 1e8 + (i as f64 * 1e-5);
        win_uniform.push_and_slide(val);
    }
    let elapsed_uniform = start_uniform.elapsed();
    let ns_per_op_uniform = elapsed_uniform.as_nanos() as f64 / ITERS as f64;

    // 2. Benchmark Test 2 Alternating Magnitude Stream (10^8 vs 10^-3 per 2048 samples)
    let start_test2 = Instant::now();
    for i in 1..=ITERS {
        let block = ((i - 1) / 2048) % 2;
        let scale = if block == 0 { 1e8 } else { 1e-3 };
        let val = scale * (1.0 + 0.5 * (i as f64).sin());
        win_test2.push_and_slide(val);
    }
    let elapsed_test2 = start_test2.elapsed();
    let ns_per_op_test2 = elapsed_test2.as_nanos() as f64 / ITERS as f64;

    println!("\n==================================================");
    println!("Uniform Stream Latency   : {:.3} ns/op", ns_per_op_uniform);
    println!("Test 2 Stream Latency    : {:.3} ns/op", ns_per_op_test2);
    println!("Delta Latency            : {:.3} ns/op", ns_per_op_test2 - ns_per_op_uniform);
    println!("==================================================");
}
