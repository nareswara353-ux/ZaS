use zero_alloc_core::window::ZeroAllocWindow;

/// Hitung penjumlahan eksak dari buffer 1024 elemen dengan pengurutan magnitudo 
/// dan kompensasi Neumaier (ekuivalen presisi eksak math.fsum).
fn exact_buffer_sum(buffer: &[f64; 1024]) -> f64 {
    let mut sorted = *buffer;
    sorted.sort_by(|a, b| a.abs().partial_cmp(&b.abs()).unwrap());
    let mut sum = 0.0f64;
    let mut comp = 0.0f64;
    for &val in &sorted {
        let t = sum + val;
        if sum.abs() >= val.abs() {
            comp += (sum - t) + val;
        } else {
            comp += (val - t) + sum;
        }
        sum = t;
    }
    sum + comp
}

#[test]
fn print_exact_discriminative_accuracy() {
    let mut engine = ZeroAllocWindow::<1024>::new();
    let mut naive_sum = 0.0f64;
    let mut buffer = [0.0f64; 1024];
    let mut head = 0;
    let mut count = 0;

    let checkpoints = [10_000, 100_000, 1_000_000, 10_000_000];
    let mut cp_idx = 0;

    println!("\n{:<10} | {:<22} | {:<15} | {:<15}", "N", "Exact Ground Truth", "Neumaier ULP", "Naive ULP");
    println!("{}", "-".repeat(70));

    for i in 1..=10_000_000 {
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
            // Baseline Naive: Uncompensated subtract-on-evict (sum += val - old_val)
            naive_sum += val - old_val;
        }

        let (engine_sum, _) = engine.push_and_slide(val);

        if cp_idx < checkpoints.len() && i == checkpoints[cp_idx] {
            let true_exact_sum = exact_buffer_sum(&buffer);
            
            let neumaier_ulp = (engine_sum.to_bits() as i64 - true_exact_sum.to_bits() as i64).abs();
            let naive_ulp = (naive_sum.to_bits() as i64 - true_exact_sum.to_bits() as i64).abs();

            println!("{:<10} | {:<22.12} | {:<15} | {:<15}", i, true_exact_sum, neumaier_ulp, naive_ulp);
            cp_idx += 1;
        }
    }
}
