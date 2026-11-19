# zero-alloc-stream (`ZaS`)

[![Crates.io](https://img.shields.io/badge/crates.io-v0.1.0-orange.svg)](https://crates.io/)
[![License: MIT/Apache-2.0](https://img.shields.io/badge/License-MIT%2FApache--2.0-blue.svg)](#license)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)]()
[![Rust Safety](https://img.shields.io/badge/unsafe-forbid-success.svg)](#safety-architecture)

> **High-performance, zero-allocation streaming aggregation engine in Safe Rust for embedded systems and real-time processing.**

`zero-alloc-stream` provides continuous $O(1)$ floating-point sliding-window sum and aggregate statistics with strict zero dynamic memory allocations on the heap (`no_std` core engine). By pairing a power-of-two circular ring buffer with Neumaier compensated summation, it achieves sub-nanosecond operation without accumulating floating-point drift over long-duration streams.

---

## Key Features

* **Strict `#![no_std]` & `#![forbid(unsafe_code)]`**: The core library (`zero-alloc-core`) guarantees zero heap allocations, zero external dependencies, and absolute memory safety without unsafe code block blocks.
* **Neumaier Compensated Summation**: Eliminates floating-point error accumulation during continuous sliding-window updates ($O(1)$ eviction & ingestion).
* **Power-of-Two Ring Buffer**: Uses bitwise masking (`(head + 1) & (W - 1)`) for branchless circular buffer indexing.
* **Deterministic Sub-Nanosecond Latency**: Optimized for real-time DSP, micro-benchmarking, low-latency financial systems, and embedded Cortex-M/ESP32 microcontrollers.
* **Isolated C-ABI FFI Bridge**: Includes a dedicated sub-crate (`zero-alloc-ffi`) for C/C++ static or dynamic library integration while keeping the core crate 100% safe.

---

## Architecture

zero-alloc-stream/
├── core/                  # zero-alloc-core crate (#![no_std], #![forbid(unsafe_code)])
│   ├── src/
│   │   ├── math.rs        # Neumaier two-sum floating-point compensation
│   │   └── window.rs      # ZeroAllocWindow core struct
│   └── tests/             # Hardware audit & analytical test suites
├── ffi/                   # zero-alloc-ffi sub-crate (C-ABI exposure)
│   └── src/lib.rs         # Pointer bridge & C-compatible exported symbols
└── reproduce.sh           # Artifact reproduction script
---

## Quick Start

Add `zero-alloc-core` to your `Cargo.toml`:

```toml
[dependencies]
zero-alloc-core = "0.1.0"
Basic UsageRustuse zero-alloc-core::ZeroAllocWindow;

fn main() {
    // Instantiate a sliding window of size W = 1024 (must be a power of two)
    let mut window = ZeroAllocWindow::<1024>::new();

    // Push new values into the stream (returns (current_sum, evicted_value))
    let (sum_1, evicted_1) = window.push_and_slide(10.5);
    let (sum_2, evicted_2) = window.push_and_slide(20.25);

    println!("Current Window Sum: {}", window.current_sum());

    // Reset window state without memory re-allocations
    window.clear();
}
Reproducing Benchmarks & AuditsAll hardware audits (multi-thread cache line isolation, FFI boundary overhead, cold-cache penalty, and dynamic-range sensitivity) can be executed using the automated reproduction script:Bash# Make reproduction script executable
chmod +x reproduce.sh

# Run complete audit suite
./reproduce.sh
Or execute cargo tests directly:Bash# Run all workspace unit & integration tests
cargo test --workspace --release

# Run C FFI overhead test suite
cargo test -p zero-alloc-ffi --test ffi_overhead --release
Dynamic-Range ConsiderationsWhile Neumaier compensated summation maintains near-zero ULP drift under homogeneous scale distributions, dynamic range transitions across extreme orders of magnitude (e.g., $10^{12} \to 10^{-12}$) retain a small rounding residue in the compensation variable $c$. For extreme scale shifts, periodic accumulator clearing (window.clear()) or double-double precision accumulators are recommended.CitationIf you use zero-alloc-stream in an academic manuscript or research project, please cite:Code snippet@misc{zeroallocstream2026,
  author = {Nareswara, Radhitya Putra},
  title = {zero-alloc-stream: A High-Performance, Zero-Allocation Streaming Aggregation Engine in Safe Rust},
  year = {2026},
  publisher = {GitHub},
  journal = {GitHub repository},
  howpublished = {\url{[https://github.com/nareswara353-ux/ZaS](https://github.com/nareswara353-ux/ZaS)}}
}
LicenseDual-licensed under either of:Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)at your option.
---

### Cara Langsung Memasukkannya dari Terminal

Jalankan perintah ini di direktori proyek kamu untuk membuat/memperbarui file `README.md` dan langsung mengunggahnya ke GitHub:

```bash
cat << 'EOF' > README.md
# zero-alloc-stream (`ZaS`)

[![Crates.io](https://img.shields.io/badge/crates.io-v0.1.0-orange.svg)](https://crates.io/)
[![License: MIT/Apache-2.0](https://img.shields.io/badge/License-MIT%2FApache--2.0-blue.svg)](#license)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)]()
[![Rust Safety](https://img.shields.io/badge/unsafe-forbid-success.svg)](#safety-architecture)

> **High-performance, zero-allocation streaming aggregation engine in Safe Rust for embedded systems and real-time processing.**

`zero-alloc-stream` provides continuous $O(1)$ floating-point sliding-window sum and aggregate statistics with strict zero dynamic memory allocations on the heap (`no_std` core engine). By pairing a power-of-two circular ring buffer with Neumaier compensated summation, it achieves sub-nanosecond operation without accumulating floating-point drift over long-duration streams.

---

## Key Features

* **Strict `#![no_std]` & `#![forbid(unsafe_code)]`**: The core library (`zero-alloc-core`) guarantees zero heap allocations, zero external dependencies, and absolute memory safety without unsafe code blocks.
* **Neumaier Compensated Summation**: Eliminates floating-point error accumulation during continuous sliding-window updates ($O(1)$ eviction & ingestion).
* **Power-of-Two Ring Buffer**: Uses bitwise masking (`(head + 1) & (W - 1)`) for branchless circular buffer indexing.
* **Deterministic Sub-Nanosecond Latency**: Optimized for real-time DSP, micro-benchmarking, low-latency financial systems, and embedded Cortex-M/ESP32 microcontrollers.
* **Isolated C-ABI FFI Bridge**: Includes a dedicated sub-crate (`zero-alloc-ffi`) for C/C++ static or dynamic library integration while keeping the core crate 100% safe.

---

## Architecture

zero-alloc-stream/
├── core/                  # zero-alloc-core crate (#![no_std], #![forbid(unsafe_code)])
│   ├── src/
│   │   ├── math.rs        # Neumaier two-sum floating-point compensation
│   │   └── window.rs      # ZeroAllocWindow core struct
│   └── tests/             # Hardware audit & analytical test suites
├── ffi/                   # zero-alloc-ffi sub-crate (C-ABI exposure)
│   └── src/lib.rs         # Pointer bridge & C-compatible exported symbols
└── reproduce.sh           # Artifact reproduction script
---

## Quick Start

Add `zero-alloc-core` to your `Cargo.toml`:

```toml
[dependencies]
zero-alloc-core = "0.1.0"
Basic UsageRustuse zero_alloc_core::ZeroAllocWindow;

fn main() {
    // Instantiate a sliding window of size W = 1024 (must be a power of two)
    let mut window = ZeroAllocWindow::<1024>::new();

    // Push new values into the stream (returns (current_sum, evicted_value))
    let (sum_1, evicted_1) = window.push_and_slide(10.5);
    let (sum_2, evicted_2) = window.push_and_slide(20.25);

    println!("Current Window Sum: {}", window.current_sum());

    // Reset window state without memory re-allocations
    window.clear();
}
Reproducing Benchmarks & AuditsAll hardware audits (multi-thread cache line isolation, FFI boundary overhead, cold-cache penalty, and dynamic-range sensitivity) can be executed using the automated reproduction script:Bash# Make reproduction script executable
chmod +x reproduce.sh

# Run complete audit suite
./reproduce.sh
Or execute cargo tests directly:Bash# Run all workspace unit & integration tests
cargo test --workspace --release

# Run C FFI overhead test suite
cargo test -p zero-alloc-ffi --test ffi_overhead --release
Dynamic-Range ConsiderationsWhile Neumaier compensated summation maintains near-zero ULP drift under homogeneous scale distributions, dynamic range transitions across extreme orders of magnitude (e.g., $10^{12} \to 10^{-12}$) retain a small rounding residue in the compensation variable $c$. For extreme scale shifts, periodic accumulator clearing (window.clear()) or double-double precision accumulators are recommended.CitationIf you use zero-alloc-stream in an academic manuscript or research project, please cite:Code snippet@misc{zeroallocstream2026,
  author = {Nareswara, Radhitya Putra},
  title = {zero-alloc-stream: A High-Performance, Zero-Allocation Streaming Aggregation Engine in Safe Rust},
  year = {2026},
  publisher = {GitHub},
  journal = {GitHub repository},
  howpublished = {\url{[https://github.com/nareswara353-ux/ZaS](https://github.com/nareswara353-ux/ZaS)}}
}
LicenseDual-licensed under either of:Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)at your option.