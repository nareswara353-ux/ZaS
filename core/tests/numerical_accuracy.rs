use zero_alloc_core::window::ZeroAllocWindow;

/// Menghitung jarak ULP (Unit in the Last Place) eksak berbasis bit-level IEEE-754 f64
fn ulp_distance(a: f64, b: f64) -> u64 {
    let a_bits = a.to_bits();
    let b_bits = b.to_bits();
    
    // Jika tanda sama, jarak ULP adalah selisih nilai integer bit-nya
    if (a_bits >> 63) == (b_bits >> 63) {
        if a_bits > b_bits { a_bits - b_bits } else { b_bits - a_bits }
    } else {
        // Jika beda tanda, hitung terhadap zero
        a_bits + b_bits
    }
}

#[test]
fn test_error_non_divergence_checkpoints() {
    const WIN_SIZE: usize = 1024;
    let checkpoints = [10_000, 100_000, 1_000_000, 10_000_000];

    let mut win_neumaier = ZeroAllocWindow::<WIN_SIZE>::new();
    let mut naive_buf = [0.0f64; WIN_SIZE];
    let mut naive_running_sum = 0.0f64;
    let mut head = 0;

    println!("\n=========================================================================================================");
    println!(" N_OPS        | GROUND TRUTH SUM       | NAIVE RUNNING ERROR          | NEUMAIER ERROR");
    println!("              |                        | Abs Error      | Exact ULP   | Abs Error      | Exact ULP");
    println!("=========================================================================================================");

    let mut current_op = 0;
    for &target in &checkpoints {
        while current_op < target {
            // Memicu pencampuran magnitudo untuk menguji kestabilan numerik
            let scale = if current_op % 7 == 0 { 1_000_000.0 } else { 0.0000001 };
            let val = ((current_op % 100) as f64 + 1.0) * scale;

            win_neumaier.push_and_slide(val);

            let x_out = naive_buf[head];
            naive_buf[head] = val;
            naive_running_sum = naive_running_sum + val - x_out;
            head = (head + 1) & (WIN_SIZE - 1);

            current_op += 1;
        }

        let (neumaier_sum, _) = win_neumaier.push_and_slide(0.0);
        let ground_truth: f64 = naive_buf.iter().sum();

        let naive_abs_err = (naive_running_sum - ground_truth).abs();
        let neumaier_abs_err = (neumaier_sum - ground_truth).abs();

        let naive_ulp = ulp_distance(naive_running_sum, ground_truth);
        let neumaier_ulp = ulp_distance(neumaier_sum, ground_truth);

        println!(
            "{:<12} | {:<22.6} | {:<14.4e} | {:<10} | {:<14.4e} | {:<10}",
            current_op, ground_truth, naive_abs_err, naive_ulp, neumaier_abs_err, neumaier_ulp
        );
    }
    println!("=========================================================================================================\n");
}
