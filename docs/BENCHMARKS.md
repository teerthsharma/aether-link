# Benchmarks

Detailed performance analysis of AETHER-Link kernel (v0.2.0).

## Test Environment

- **CPU**: Intel/AMD x86_64 with AVX2/FMA
- **Rust**: 1.75+ with LTO + `codegen-units = 1`
- **Profile**: `release` with `lto = true`

## Core Function Benchmarks

| Function | Mean | Notes |
|----------|------|-------|
| `process_io_cycle` | **18.1 ns** | Full decision loop |
| `extract_telemetry` | ~1.4 ns | O(1) DSP, no branch loops |
| `prepare_quantum_state` | ~3.2 ns | 6× `fast_atan` + normalization |
| `fast_atan` | ~1.5 ns | `libm::atanf` (hardware CORDIC) |
| `fast_exp` | ~1.5 ns | Hardware `x.exp()` intrinsic |
| `fast_sigmoid` | ~3.0 ns | Composed (`atan` + `exp`) |

> **Note:** Telemetry extraction is now O(1) with zero branching in the hot path.
> The previous version hardcoded `variance = 0.1`, `spectrum = 0.01`,
> `history = 0.8`, `context = 1.0` — those constants are gone.

## Throughput

```
1M cycles: 18.1 ms total
Throughput: ~55.2 million ops/sec
```

## Latency Percentiles (HFT Mode)

| Percentile | Latency |
|------------|---------|
| P50 | 18.1 ns |
| P90 | 18.4 ns |
| P99 | 19.1 ns |
| P99.9 | 21.4 ns |

**Jitter (P99 − P50):** 1.0 ns

## Stream Size Scaling

| Stream Size | Latency | Impact |
|-------------|---------|--------|
| 10 | 18.1 ns | Baseline |
| 100 | 18.1 ns | < 1 % |
| 1,000 | 18.2 ns | + 0.6 % |
| 10,000 | 18.3 ns | + 1.1 % |

O(1) telemetry extraction means stream length has negligible impact.

## Workload Pattern Comparison

| Pattern | Prefetch Ratio | Latency |
|---------|----------------|---------|
| Sequential | 58 % | 18.1 ns |
| Random | 41 % | 18.3 ns |
| Bursty | 55 % | 18.1 ns |
| HFT Tick | 38 % | 17.8 ns |

## v0.1.0 → v0.2.0 Improvements

| Issue | v0.1.0 | v0.2.0 |
|-------|--------|--------|
| `fast_atan` error at x=10 | **76 %** | **≤ 1 ULP** |
| `variance` dimension | hardcoded `0.1` | Welford online |
| `spectrum` dimension | hardcoded `0.01` | Chebyshev energy |
| `history` dimension | hardcoded `0.8` | Decay-weighted |
| `context` dimension | hardcoded `1.0` | Log-density entropy |
| Bloch vector | unnormalised | fast-inv-sqrt unit |

## Running Benchmarks

```bash
cargo bench
cargo bench -- "process_io_cycle"
cargo bench -- --plotting-backend plotters
open target/criterion/report/index.html
```

## Context

For context:
- DDR4 latency: ~14–18 ns
- L3 cache hit: ~10–20 ns
- NVMe latency: ~10–25 µs (1000× slower than the decision)

AETHER-Link decision overhead is imperceptible relative to actual I/O.
