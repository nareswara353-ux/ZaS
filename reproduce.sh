#!/usr/bin/env bash
set -e

echo "======================================================================"
echo "          ARTIFACT REPRODUCTION & AUDIT BENCHMARK SUITE               "
echo "======================================================================"

# 1. Verification: Compile and Run 10M Zero-Allocation Heap Audit
echo -e "\n[1/3] Executing 10,000,000 Operations Strict Zero-Alloc Test..."
cargo test -p zero-alloc-core --test perf_multi_stream_audit test_zero_alloc_10m --release -- --nocapture

# 2. Benchmark Executions: Criterion Multi-Stream Latency Tests
echo -e "\n[2/3] Executing Criterion Multi-Stream Isolated Latency Benchmarks..."
cargo bench -p zero-alloc-benches

# 3. PMU Hardware Counters Audit: Linux perf stat across 3 streams
echo -e "\n[3/3] Running Linux perf stat PMU Audit (10M Iterations per Stream)..."
cargo test -p zero-alloc-core --test perf_multi_stream_audit --release --no-run > /dev/null 2>&1

BIN=$(ls -t target/release/deps/perf_multi_stream_audit-* | grep -v '\.d$' | head -n 1)

echo -e "\n---> A. Uniform Stream PMU Counter:"
perf stat -e branches,branch-misses $BIN perf_uniform_stream --nocapture

echo -e "\n---> B. Blockwise Stream PMU Counter:"
perf stat -e branches,branch-misses $BIN perf_blockwise_stream --nocapture

echo -e "\n---> C. Adversarial Stream PMU Counter:"
perf stat -e branches,branch-misses $BIN perf_adversarial_stream --nocapture

echo -e "\n======================================================================"
echo "               ALL REPRODUCTION AUDITS COMPLETED                      "
echo "======================================================================"
