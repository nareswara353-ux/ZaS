use zero_alloc_core::window::ZeroAllocWindow;

#[test]
fn test_ulp_real_window() {
    let mut win = ZeroAllocWindow::<1024>::new();
    let mut buffer = [0.0f64; 1024];
    let mut head = 0;
    let mut last_engine_sum = 0.0f64;

    for i in 1..=10_000_000 {
        let val = if i % 2 == 0 { 1e8 + (i as f64 * 1e-5) } else { -1e8 + (i as f64 * 1e-5) };
        buffer[head] = val;
        head = (head + 1) & (1024 - 1);
        let (sum, _) = win.push_and_slide(val);
        last_engine_sum = sum;
    }

    let true_sum: f64 = buffer.iter().sum();
    let diff_bits = (last_engine_sum.to_bits() as i64 - true_sum.to_bits() as i64).abs();

    println!("==================================================");
    println!("Ground Truth (Exact Sum) : {:.15}", true_sum);
    println!("Engine Real Window Sum   : {:.15}", last_engine_sum);
    println!("Measured ULP Distance    : {} ULP", diff_bits);
    println!("==================================================");
}
