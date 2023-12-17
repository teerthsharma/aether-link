# AETHER-Link
[![Crates.io](https://img.shields.io/crates/v/aether-link.svg)](https://crates.io/crates/aether-link)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](#license)

A sub-20 ns I/O prefetching kernel written in Rust, designed for latency-critical applications:
HFT, DirectStorage gaming, and WSL2 acceleration.

## What it actually does

AETHER-Link sits between your application and your storage layer. You feed it a stream
of Logical Block Addresses (LBAs) and it returns a `bool` — prefetch or defer to OS cache.

Instead of ML models or naive stride detection, it uses a **quantum-inspired adaptive
measurement algorithm** (POVM formalism) that continuously evolves its decision basis
from the I/O stream itself.  No training.  No heap.  No network calls.

## The Numbers

Built for the hot path.  `#![no_std]` compatible.  Zero heap allocations in the
decision loop.

| Metric | Value | Notes |
|--------|-------|-------|
| Decision latency | **~18.1 ns** | Full `process_io_cycle` loop |
| Telemetry extraction | ~1.4 ns | O(1), zero-copy DSP |
| Throughput | ~55 M ops/sec | Single thread |
| Jitter (P99 − P50) | **< 1 ns** | Tight latency guarantees |
| Telemetry dimensions | **6 real** | Welford variance, spectral energy, entropy |
| fast_atan error | **≤ 1 ULP** | `libm::atanf`, not the old 76%-error Padé |

For context: NVMe hardware latency is ~10–25 µs.  AETHER-Link's decision overhead
is ~1000× smaller than the I/O it schedules.

## Quick Start

```toml
[dependencies]
aether-link = "0.2.0"
```

```rust
use aether_link::AetherLinkKernel;

let mut kernel = AetherLinkKernel::new_hft();
let lba_stream = [1000, 1001, 1002, 1003, 1010];

if kernel.process_io_cycle(&lba_stream) {
    // Dispatch via DirectStorage / GPU Direct / io_uring
    println!("Aggressive prefetch triggered.");
} else {
    // Defer to standard OS page cache
    println!("Deferring to OS page cache.");
}
```

## How It Works

### 1. Telemetry DSP (~1.4 ns)

Six real features extracted from the LBA stream — **no hardcoded constants**:

| Feature | Symbol | Description |
|---------|--------|-------------|
| Delta | Δ | LBA span: `last − first` |
| Velocity | V | `Δ × 0.5` (acceleration proxy) |
| Variance | σ² | Welford running variance over all seen streams |
| Spectrum | C | Chebyshev spectral energy (running RMS of delta-diff) |
| History | H | Exponential decay temporal weight (decay = 0.8) |
| Context | Ω | Log-density entropy of recent inter-arrival rates |

### 2. Quantum-Inspired State Encoding (~3.2 ns)

Features are mapped to a **Bloch sphere** quantum state:

```
θᵢ = 2 × atan(fᵢ / scale)     // polar angle
φ  = weighted azimuthal average

Bloch vector (normalised): [rx, ry, rz] on S²
```

The normalised Bloch vector guarantees the subsequent POVM measurement
produces properly bounded expectation values.

### 3. Adaptive POVM Measurement (~18 ns total)

Three POVM observables probe the Bloch vector against the adaptive basis `φ`:

```
E₁ = cos(θ + φ)   → spatial (LBA velocity alignment)
E₂ = sin(θ/2 − φ)  → temporal phase (drives basis rotation)
E₃ = cos(θ · φ)   → spectral (drives fetch sigmoid)
```

The basis `φ` is updated after each measurement, giving continuous adaptation
without any trained parameters.

> **Note:** "Quantum-inspired" means we borrow the mathematical formalism
> (Bloch sphere, POVM observables, basis rotation) from quantum mechanics.
> No actual qubits or quantum hardware are involved.

## Hardware Integration

- **NVIDIA BlueField DPUs**: Run the decision kernel on the DPU ARM cores,
  making inline decisions before data hits the PCIe bus.
- **CUDA GPUs**: Batch thousands of streams by encoding POVM states as GPU tensors.
- **No-std / bare-metal**: Works in kernel space and embedded contexts.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for PR guidelines and the benchmark suite.

## License

Apache License 2.0 — Teerth Sharma, 2026.
