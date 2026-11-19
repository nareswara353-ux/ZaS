use zero_alloc_core::window::ZeroAllocWindow;

#[test]
fn run_perf_branch_check() {
    let mut win = ZeroAllocWindow::<1024>::new();
    let mut state: u64 = 0x123456789ABCDEF0;

    let mut sum_out = 0.0;
    for _i in 0..10_000_000 {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        let sign = if (state & 1) == 0 { 1.0 } else { -1.0 };
        let val = sign * 1e8 * (1.0 + ((state & 0xFFFF) as f64 / 65535.0) * 0.1);
        let (s, _) = win.push_and_slide(val);
        sum_out += s;
    }
    std::hint::black_box(sum_out);
}
