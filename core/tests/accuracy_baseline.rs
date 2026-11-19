use zero_alloc_core::ZeroAllocWindow;

#[test]
fn test_accuracy_comparison() {
    let mut zero_alloc = ZeroAllocWindow::<1024>::new();
    let mut naive_sliding_sum = 0.0f64;
    let mut buffer = [0.0f64; 1024];
    let mut head = 0;
    let mut neumaier_sum = 0.0f64;

    for i in 1..=10_000_000 {
        let val = if i % 2 == 0 { 1e8 + (i as f64 * 1e-5) } else { -1e8 + (i as f64 * 1e-5) };
        let old_val = buffer[head];
        buffer[head] = val;
        head = (head + 1) % 1024;

        if i <= 1024 {
            naive_sliding_sum += val;
        } else {
            naive_sliding_sum += val - old_val;
        }

        // Tangkap nilai Neumaier langsung dari iterasi stream aktif
        let (sum, _) = zero_alloc.push_and_slide(val);
        neumaier_sum = sum;
    }

    let ground_truth: f64 = buffer.iter().sum();

    println!("==================================================");
    println!("Ground Truth (Full Sum) : {:.6}", ground_truth);
    println!("Neumaier ZeroAlloc Sum  : {:.6}", neumaier_sum);
    println!("Naive Sliding Sum       : {:.6}", naive_sliding_sum);
    println!("==================================================");
}
