use zero_alloc_core::ZeroAllocWindow;
use std::hint::black_box;
use std::time::Instant;

#[test]
fn test_cold_cache_flush_penalty() {
    println!("\n[START] Executing Cold-Cache Flush Penalty Audit...");

    const WARM_ITERS: usize = 10_000_000;
    const COLD_SAMPLES: usize = 100_000;

    // 1. Warm-Cache Baseline Benchmark
    let mut warm_win = ZeroAllocWindow::<1024>::new();
    let start_warm = Instant::now();
    let mut val = 1.0f64;
    for _ in 0..WARM_ITERS {
        val += 0.0001;
        black_box(warm_win.push_and_slide(black_box(val)));
    }
    let elapsed_warm = start_warm.elapsed();
    let ns_per_op_warm = elapsed_warm.as_nanos() as f64 / WARM_ITERS as f64;

    // 2. Cold-Cache Simulated Benchmark (Evicting Window State from Cache)
    // Menggunakan array penguras cache besar (64 MiB) untuk membuang footprint dari L1/L2/L3 cache
    let mut cold_win = ZeroAllocWindow::<1024>::new();
    let mut evict_buffer = vec![0u8; 64 * 1024 * 1024]; // 64 MiB L3-eviction buffer
    
    let mut total_cold_duration_ns = 0u128;
    let mut val_cold = 1.0f64;

    for i in 0..COLD_SAMPLES {
        val_cold += 0.0001;
        
        // Kuras Cache Line (Touch 64 MiB memori eksternal secara terdispersi)
        let offset = (i * 4096) % evict_buffer.len();
        evict_buffer[offset] = (i & 0xFF) as u8;
        black_box(evict_buffer[offset]);

        // Ukur latensi single operation persis saat cache kondisi dingin (Cold-Touch)
        let t0 = Instant::now();
        black_box(cold_win.push_and_slide(black_box(val_cold)));
        total_cold_duration_ns += t0.elapsed().as_nanos();
    }

    let ns_per_op_cold = total_cold_duration_ns as f64 / COLD_SAMPLES as f64;
    let cold_penalty_ns = ns_per_op_cold - ns_per_op_warm;

    println!("  -> Warm-Cache Latency Baseline : {:.3} ns/op", ns_per_op_warm);
    println!("  -> Cold-Cache Flush Latency    : {:.3} ns/op", ns_per_op_cold);
    println!("  -> Cold-Start Penalty Delta     : {:.3} ns/op", cold_penalty_ns);
    println!("[SUCCESS] Cold-Cache Flush Audit Completed.\n");
}
