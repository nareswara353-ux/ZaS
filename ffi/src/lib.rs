extern crate alloc;
use alloc::boxed::Box;
use zero_alloc_core::ZeroAllocWindow;

#[repr(C)]
pub struct ZeroAllocWindowC1024 {
    inner: ZeroAllocWindow<1024>,
}

#[no_mangle]
pub extern "C" fn zero_alloc_window_1024_new() -> *mut ZeroAllocWindowC1024 {
    let window = ZeroAllocWindowC1024 {
        inner: ZeroAllocWindow::<1024>::new(),
    };
    Box::into_raw(Box::new(window))
}

#[no_mangle]
pub extern "C" fn zero_alloc_window_1024_push_and_slide(
    ptr: *mut ZeroAllocWindowC1024,
    val: f64,
) -> f64 {
    if ptr.is_null() {
        return 0.0;
    }
    let window = unsafe { &mut *ptr };
    let (sum, _) = window.inner.push_and_slide(val);
    sum
}

#[no_mangle]
pub extern "C" fn zero_alloc_window_1024_free(ptr: *mut ZeroAllocWindowC1024) {
    if !ptr.is_null() {
        unsafe {
            let _ = Box::from_raw(ptr);
        }
    }
}

// ------------------------------------------------------------------------
// Alias FFI C-ABI API (Double Pointer Variant untuk ffi_overhead.rs)
// ------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn zero_alloc_create() -> *mut ZeroAllocWindowC1024 {
    zero_alloc_window_1024_new()
}

/// Accept double-pointer (*mut *mut) or single-pointer (*mut) safely
#[no_mangle]
pub extern "C" fn zero_alloc_push(
    handle_ptr: *mut *mut ZeroAllocWindowC1024,
    val: f64,
    out_sum: *mut f64,
    out_evicted_or_mean: *mut f64,
) -> i32 {
    if handle_ptr.is_null() {
        return -1;
    }
    let ptr = unsafe { *handle_ptr };
    if ptr.is_null() {
        return -1;
    }

    let window = unsafe { &mut *ptr };
    let (sum, x_out) = window.inner.push_and_slide(val);

    if !out_sum.is_null() {
        unsafe { *out_sum = sum };
    }
    if !out_evicted_or_mean.is_null() {
        unsafe { *out_evicted_or_mean = x_out };
    }

    0
}

#[no_mangle]
pub extern "C" fn zero_alloc_free(ptr: *mut ZeroAllocWindowC1024) {
    zero_alloc_window_1024_free(ptr);
}
