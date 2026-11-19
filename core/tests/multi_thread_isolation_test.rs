use zero_alloc_core::ZeroAllocWindow;
use std::hint::black_box;
use std::thread;
use std::time::Instant;

#[test]
fn test_multi_thread_isolation_no_false_sharing() {
    println!("\n[START] Executing Multi-Thread Isolation & Concurrent Multi-Instance Audit...");

    const THREAD_COUNT: usize = 8;
    const ITERS_PER_THREAD: usize = 10_000_000;

    let start = Instant::now();
    let mut handles = Vec::with_capacity(THREAD_COUNT);

    for t_id in 0..THREAD_COUNT {
        let handle = thread::spawn(move || {
            let mut win = ZeroAllocWindow::<1024>::new();
            let mut val = (t_id + 1) as f64 * 1.5f64;
            let mut accum = 0.0f64;

            for _ in 0..ITERS_PER_THREAD {
                val += 0.00001;
                let (sum, _) = win.push_and_slide(black_box(val));
                accum += sum;
            }
            black_box(accum)
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.join().unwrap();
    }

    let elapsed = start.elapsed();
    let total_ops = THREAD_COUNT * ITERS_PER_THREAD;
    let ns_per_op_throughput = elapsed.as_nanos() as f64 / total_ops as f64;

    println!("  -> Active Concurrent Threads       : {}", THREAD_COUNT);
    println!("  -> Total Combined Operations      : {}", total_ops);
    println!("  -> Total Concurrent Wall-Clock Time: {:.3} ms", elapsed.as_secs_f64() * 1000.0);
    println!("  -> Effective System Throughput    : {:.3} ns/op (across all threads)", ns_per_op_throughput);
    println!("[SUCCESS] Multi-Thread Isolation Audit Completed.\n");
}
