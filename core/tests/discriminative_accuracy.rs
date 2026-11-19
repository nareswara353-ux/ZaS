use zero_alloc_core::window::ZeroAllocWindow;

#[test]
fn print_discriminative_accuracy() {
    let mut engine = ZeroAllocWindow::<1024>::new();
    let mut naive_sum = 0.0f64;
    let mut buffer = [0.0f64; 1024];
    let mut head = 0;
    let mut count = 0;

    let checkpoints = [10_000, 100_000, 1_000_000, 10_000_000];
    let mut cp_idx = 0;

    println!("\n{:<10} | {:<22} | {:<15} | {:<15}", "N", "Ground Truth", "Neumaier ULP", "Naive ULP");
    println!("{}", "-".repeat(70));

    for i in 1..=10_000_000 {
        // Blok bergantian 2048 sampel: skala 1e8 lalu 1e-3
        let block = ((i - 1) / 2048) % 2;
        let scale = if block == 0 { 1e8 } else { 1e-3 };
        let val = scale * (1.0 + 0.5 * (i as f64).sin());

        let old_val = buffer[head];
        buffer[head] = val;
        head = (head + 1) & (1024 - 1);

        if count < 1024 {
            count += 1;
            naive_sum += val;
        } else {
            naive_sum += val - old_val;
        }

        let (engine_sum, _) = engine.push_and_slide(val);

        if cp_idx < checkpoints.len() && i == checkpoints[cp_idx] {
            let true_sum: f64 = buffer.iter().sum();
            
            let neumaier_ulp = (engine_sum.to_bits() as i64 - true_sum.to_bits() as i64).abs();
            let naive_ulp = (naive_sum.to_bits() as i64 - true_sum.to_bits() as i64).abs();

            println!("{:<10} | {:<22.12} | {:<15} | {:<15}", i, true_sum, neumaier_ulp, naive_ulp);
            cp_idx += 1;
        }
    }
}
