use zero_alloc_core::ZeroAllocWindow;

/// Algoritma Shewchuk / Neumaier reference pembantu untuk memvalidasi sum secara eksak
fn reference_neumaier_sum(slice: &[f64]) -> f64 {
    let mut sum = 0.0;
    let mut c = 0.0;
    for &val in slice {
        let t = sum + val;
        if sum.abs() >= val.abs() {
            c += (sum - t) + val;
        } else {
            c += (val - t) + sum;
        }
        sum = t;
    }
    sum + c
}

#[test]
fn test_ultra_long_precision_drift_100m() {
    println!("\n[START] Executing 100,000,000 Operations Ultra-Long Precision Drift Test...");

    const W: usize = 1024;
    const TOTAL_OPS: usize = 100_000_000;
    
    let mut win = ZeroAllocWindow::<W>::new();
    let mut buffer_history = vec![0.0f64; W];
    
    // Pola LCG Deterministik untuk menghasilkan stream acak magnitudo ekstrem
    let mut seed: u64 = 0xDEADBEEF;
    let lcg = |s: &mut u64| -> f64 {
        *s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let raw = (*s >> 11) as f64 / (1u64 << 53) as f64; // Normalisasi [0, 1)
        
        // Pilihan magnitudo ekstrem: scale dari 1e-12 hingga 1e12 dengan alih tanda
        let exponent = ((s.wrapping_shr(32) % 25) as i32) - 12; // [-12 .. 12]
        let sign = if (*s & 1) == 0 { 1.0 } else { -1.0 };
        sign * raw * 10.0f64.powi(exponent)
    };

    let mut max_ulp_err: u64 = 0;
    let mut checkpoint_count = 0;

    for i in 0..TOTAL_OPS {
        let val = lcg(&mut seed);
        let head = i % W;
        buffer_history[head] = val;

        let (engine_sum, _) = win.push_and_slide(val);

        // Verifikasi Checkpoint setiap 10.000.000 iterasi
        if (i + 1) % 10_000_000 == 0 {
            checkpoint_count += 1;
            let ref_sum = reference_neumaier_sum(&buffer_history);
            
            let engine_bits = engine_sum.to_bits();
            let ref_bits = ref_sum.to_bits();
            let ulp_diff = if engine_bits > ref_bits {
                engine_bits - ref_bits
            } else {
                ref_bits - engine_bits
            };

            if ulp_diff > max_ulp_err {
                max_ulp_err = ulp_diff;
            }

            println!(
                "  -> Checkpoint [{}/10] (N = {} ops): Engine = {:.15e}, Ref = {:.15e}, ULP Diff = {}",
                checkpoint_count,
                i + 1,
                engine_sum,
                ref_sum,
                ulp_diff
            );

            assert_eq!(
                ulp_diff, 0,
                "Precision drift terdeteksi pada iterasi ke-{}! ULP diff = {}",
                i + 1,
                ulp_diff
            );
        }
    }

    println!("[SUCCESS] 100,000,000 Ops Completed. Max ULP Drift = {} ULP.\n", max_ulp_err);
}
