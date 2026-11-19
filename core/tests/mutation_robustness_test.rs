use zero_alloc_core::ZeroAllocWindow;

#[test]
#[should_panic(expected = "Window size W must be a power of two")]
fn test_mutation_invalid_window_non_power_of_two() {
    // Memverifikasi assertion power-of-two (W=1000) menggagalkan instansiasi secara eksplisit
    let _ = ZeroAllocWindow::<1000>::new();
}

#[test]
fn test_mutation_zero_input_stability() {
    let mut win = ZeroAllocWindow::<64>::new();
    for _ in 0..100 {
        let (sum, _) = win.push_and_slide(0.0);
        assert_eq!(sum, 0.0, "Zero input mutation test gagal!");
    }
}

#[test]
fn test_mutation_subnormal_float_stability() {
    let mut win = ZeroAllocWindow::<64>::new();
    let subnormal = f64::MIN_POSITIVE / 2.0; // Denormalized float
    for _ in 0..64 {
        let (sum, _) = win.push_and_slide(subnormal);
        assert!(sum.is_finite(), "Subnormal float memicu instabilitas!");
    }
}
