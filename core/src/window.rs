#![forbid(unsafe_code)]

use crate::math::neumaier_add;

#[derive(Debug, Clone)]
pub struct ZeroAllocWindow<const W: usize> {
    buffer: [f64; W],
    head: usize,
    sum: f64,
    c: f64,
}

impl<const W: usize> ZeroAllocWindow<W> {
    pub fn new() -> Self {
        assert!(W > 0 && (W & (W - 1)) == 0, "Window size W must be a power of two");
        Self {
            buffer: [0.0; W],
            head: 0,
            sum: 0.0,
            c: 0.0,
        }
    }

    /// Reset state window kembali ke kondisi awal tanpa alokasi ulang memori
    #[inline(always)]
    pub fn clear(&mut self) {
        self.buffer = [0.0; W];
        self.head = 0;
        self.sum = 0.0;
        self.c = 0.0;
    }

    #[inline(always)]
    pub fn push_and_slide(&mut self, val: f64) -> (f64, f64) {
        let x_out = self.buffer[self.head];

        // 1. Eviction-prior-ingestion: Evakuasi x_out terlebih dahulu
        let (s1, c1) = neumaier_add(self.sum, self.c, -x_out);

        // 2. Ingestion: Masukkan elemen baru x_in
        let (s2, c2) = neumaier_add(s1, c1, val);

        self.sum = s2;
        self.c = c2;
        self.buffer[self.head] = val;

        // Bitwise masking untuk W power-of-two (branchless indexing)
        self.head = (self.head + 1) & (W - 1);

        (self.sum + self.c, x_out)
    }

    #[inline(always)]
    pub fn current_sum(&self) -> f64 {
        self.sum + self.c
    }
}
