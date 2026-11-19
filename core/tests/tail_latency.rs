use zero_alloc_core::window::ZeroAllocWindow;
use std::time::Instant;

#[test]
fn test_tail_latency_distribution() {
    const ITERATIONS: usize = 1_000_000;
    const WIN_SIZE: usize = 1024;
    let mut win = ZeroAllocWindow::<WIN_SIZE>::new();
    let mut samples = Vec::with_capacity(ITERATIONS);
    let mut input = 1.05f64;

    for _ in 0..10_000 {
        std::hint::black_box(win.push_and_slide(std::hint::black_box(1.0)));
    }

    for _ in 0..ITERATIONS {
        input += 0.001;
        let start = Instant::now();
        std::hint::black_box(win.push_and_slide(std::hint::black_box(input)));
        let elapsed = start.elapsed().as_nanos() as f64;
        samples.push(elapsed);
    }

    samples.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let p50 = samples[(ITERATIONS as f64 * 0.50) as usize];
    let p90 = samples[(ITERATIONS as f64 * 0.90) as usize];
    let p99 = samples[(ITERATIONS as f64 * 0.99) as usize];
    let p99_9 = samples[(ITERATIONS as f64 * 0.999) as usize];
    let max_lat = samples[ITERATIONS - 1];

    println!("\n=======================================================================");
    println!("=== TAIL LATENCY DISTRIBUTION & JITTER (1,000,000 SAMPLES) ===");
    println!("=======================================================================");
    println!("p50   (Median Latency)   : {:.2} ns", p50);
    println!("p90   (90th Percentile)  : {:.2} ns", p90);
    println!("p99   (99th Percentile)  : {:.2} ns", p99);
    println!("p99.9 (99.9th Percentile): {:.2} ns", p99_9);
    println!("Max Latency Spike        : {:.2} ns", max_lat);
    println!("=======================================================================\n");
}
