use zero_alloc_ffi::{zero_alloc_create, zero_alloc_push};
use zero_alloc_core::window::ZeroAllocWindow;
use std::time::Instant;

#[test]
fn test_ffi_boundary_overhead() {
    const ITERATIONS: usize = 1_000_000;

    let mut native_win = ZeroAllocWindow::<1024>::new();        
    let start_native = Instant::now();
    let mut val1 = 1.05f64;
    for _ in 0..ITERATIONS {
        val1 += 0.001;
        std::hint::black_box(native_win.push_and_slide(std::hint::black_box(val1)));
    }
    let elapsed_native = start_native.elapsed().as_nanos() as f64 / ITERATIONS as f64;

    let mut ffi_handle = zero_alloc_create();
    let mut sum_out = 0.0f64;
    let mut mean_out = 0.0f64;
    let mut val2 = 1.05f64;
    let start_ffi = Instant::now();
    for _ in 0..ITERATIONS {
        val2 += 0.001;
        zero_alloc_push(
            &mut ffi_handle,
            std::hint::black_box(val2),
            &mut sum_out,
            &mut mean_out,
        );
    }
    let elapsed_ffi = start_ffi.elapsed().as_nanos() as f64 / ITERATIONS as f64;

    let overhead = elapsed_ffi - elapsed_native;

    println!("\n=======================================================================");
    println!("=== FFI BOUNDARY OVERHEAD EVALUATION (C-ABI VS RUST NATIVE) ===");
    println!("=======================================================================");
    println!("Native Rust Call Latency : {:.2} ns/op", elapsed_native);
    println!("FFI C-ABI Call Latency   : {:.2} ns/op", elapsed_ffi);
    println!("FFI Boundary Overhead    : {:.2} ns/op", overhead);
    println!("=======================================================================\n");
}
