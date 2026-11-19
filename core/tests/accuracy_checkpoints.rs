use zero_alloc_core::window::ZeroAllocWindow;

#[test]
fn print_accuracy_checkpoints() {
    let mut win = ZeroAllocWindow::<1024>::new();
    let mut buffer = [0.0f64; 1024];
    let mut head = 0;

    let checkpoints = [10_000, 100_000, 1_000_000, 10_000_000];
    let mut cp_idx = 0;

    println!("\n{:<10} | {:<25} | {:<25} | {:<15} | {:<10}", "N", "Ground Truth", "Engine Sum", "Abs Error", "ULP");
    println!("{}", "-".repeat(95));

    for i in 1..=10_000_000 {
        let val = if i % 2 == 0 { 1e8 + (i as f64 * 1e-5) } else { -1e8 + (i as f64 * 1e-5) };
        buffer[head] = val;
        head = (head + 1) & (1024 - 1);
        let (sum, _) = win.push_and_slide(val);

        if cp_idx < checkpoints.len() && i == checkpoints[cp_idx] {
            let true_sum: f64 = buffer.iter().sum();
            let abs_err = (sum - true_sum).abs();
            let ulp = (sum.to_bits() as i64 - true_sum.to_bits() as i64).abs();
            println!("{:<10} | {:<25.15} | {:<25.15} | {:<15.6e} | {:<10}", i, true_sum, sum, abs_err, ulp);
            cp_idx += 1;
        }
    }
}
