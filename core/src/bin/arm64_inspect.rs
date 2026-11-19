use zero_alloc_core::ZeroAllocWindow;

fn main() {
    let mut win = ZeroAllocWindow::<1024>::new();
    let res = win.push_and_slide(123.456);
    // Menggunakan print sederhana untuk mencegah compiler menghapus (DCE) eksekusi
    if res.0 == 0.0 {
        println!("zero");
    }
}
