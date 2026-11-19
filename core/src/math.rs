#![forbid(unsafe_code)]

/// Menghitung akumulasi Neumaier summation untuk menjaga keakuratan galat hingga 0 ULP.
/// Mengembalikan tuple `(sum_baru, compensation_baru)`.
#[inline(always)]
pub fn neumaier_add(sum: f64, c: f64, val: f64) -> (f64, f64) {
    let t = sum + val;
    let new_c = if sum.abs() >= val.abs() {
        c + ((sum - t) + val)
    } else {
        c + ((val - t) + sum)
    };
    (t, new_c)
}
