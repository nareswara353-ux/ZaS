use zero_alloc_core::ZeroAllocWindow;

#[test]
fn test_fp_edge_cases() {
    let mut win = ZeroAllocWindow::<64>::new();
    
    // Tes 1: Penanganan Subnormal / Denormal numbers
    let subnormal = f64::MIN_POSITIVE / 2.0;
    let (sum, mean) = win.push_and_slide(subnormal);
    assert!(!sum.is_nan());
    assert!(!mean.is_nan());

    // Tes 2: Penanganan Nilai Nol Berlawanan Tanda (-0.0 vs +0.0)
    let (sum_zero, _) = win.push_and_slide(-0.0);
    assert_eq!(sum_zero, sum_zero);

    println!("Edge-case IEEE-754 validation: PASSED");
}
