<div align="center">

# AETHER-Link

### Adaptive Event-driven Threshold Hybrid Entangled Rendering

**High-Performance I/O Prefetch Kernel for DirectStorage, WSL2 & HFT**

[![Crates.io](https://img.shields.io/crates/v/aether-link.svg?style=flat-square&logo=rust)](https://crates.io/crates/aether-link)
[![Documentation](https://img.shields.io/docsrs/aether-link?style=flat-square&logo=docs.rs)](https://docs.rs/aether-link)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue?style=flat-square)](LICENSE-MIT)
[![CI](https://img.shields.io/github/actions/workflow/status/teerthsharma/aether-link/ci.yml?style=flat-square&logo=github)](https://github.com/teerthsharma/aether-link/actions)

[📄 Paper](https://www.researchgate.net/publication/398493933) • [📖 Docs](https://docs.rs/aether-link) • [🚀 Quick Start](#quick-start) • [📊 Benchmarks](#benchmarks)

</div>

---

## Overview

AETHER-Link is an adaptive I/O prefetch kernel designed to bypass OS bottlenecks in latency-critical applications. It uses a probabilistic decision model that learns from I/O patterns in real-time without requiring pre-trained weights or expensive ML inference.

### Use Cases

| Domain | Application | Benefit |
|--------|------------|---------|
| **High-Frequency Trading** | Market data prefetching | Sub-20ns decision latency |
| **DirectStorage** | Game asset streaming | Reduced load times |
| **WSL2** | NVMe bypass | Lower syscall overhead |
| **LLM Inference** | Weight loading | Faster model init |

### Research

> **Paper**: [DOI: 10.13140/RG.2.2.22443.91687](https://www.researchgate.net/publication/398493933_AETHER_-_Adaptive_Event-driven_Threshold_Hybrid_Entangled_Rendering)

---

## How It Works

The kernel extracts features from I/O request streams and makes probabilistic prefetch decisions in constant time.

```
LBA Stream → Feature Extraction → Adaptive Decision → Prefetch/Skip
   (input)      (~1 ns)              (~14 ns)          (output)
```

1. **Feature Extraction**: Extract 6D telemetry from LBA history (spatial delta, velocity, variance, spectral proxy, temporal context)
2. **Adaptive Thresholding**: Continuously adjusts decision boundary based on observed patterns
3. **Probabilistic Decision**: Sigmoid-based fetch probability compared against adaptive threshold

The adaptation happens on every I/O cycle with O(1) complexity and zero allocations.

See [docs/QUANTUM_THEORY.md](docs/QUANTUM_THEORY.md) for the mathematical foundations.

---

## Benchmarks

Measured on local hardware:

| Component | Latency | Throughput |
|-----------|---------|------------|
| **Full Cycle** | **14.6 ns** | **~68 Mops/s** |
| Feature Extraction | 0.99 ns | - |
| Decision Logic | 3.68 ns | - |
| Fast Math | 0.62 ns | - |

```
HFT Preset:  14.59 ns
Gaming:      14.73 ns
Throughput:  65.3 million ops/sec
```

For context: DDR4 latency is ~14-18ns. Our decision overhead is effectively free.

---

## Quick Start

Add to `Cargo.toml`:

```toml
[dependencies]
aether-link = "0.1"
```

### Basic Usage

```rust
use aether_link::AetherLinkKernel;

let mut kernel = AetherLinkKernel::new_hft();

for request in io_queue {
    let lba_history = get_recent_lbas();
    
    if kernel.process_io_cycle(&lba_history) {
        direct_storage_read(request);  // Bypass OS
    } else {
        os_read(request);  // Standard path
    }
}
```

### Presets

```rust
// Conservative (HFT) - minimize false positives
let kernel = AetherLinkKernel::new_hft();

// Aggressive (Gaming) - maximize prefetch
let kernel = AetherLinkKernel::new_gaming();

// Custom tuning
let kernel = AetherLinkKernel::new(
    0.55,               // epsilon: threshold
    0.1,                // phi: adaptation basis
    [0.08, 0.15, 0.25], // lambda: learning rates
    0.0,                // bias
);
```

---

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     AETHER-Link Kernel                      │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────┐  │
│  │   Feature    │─▶│   Adaptive   │─▶│    Decision      │  │
│  │  Extractor   │  │  Threshold   │  │     Gate         │  │
│  └──────────────┘  └──────────────┘  └──────────────────┘  │
│         │                 │                   │             │
│         ▼                 ▼                   ▼             │
│     Telemetry        State Update        Prefetch?          │
└─────────────────────────────────────────────────────────────┘
                            │
            ┌───────────────┴───────────────┐
            ▼                               ▼
    DirectStorage API               OS Page Cache
```

---

## Building

```bash
git clone https://github.com/teerthsharma/aether-link
cd aether-link

cargo build --release
cargo test
cargo bench
cargo run --example basic_usage
```

---

## Documentation

- [Architecture](docs/ARCHITECTURE.md) - System design and integration
- [Benchmarks](docs/BENCHMARKS.md) - Detailed performance analysis
- [Theory](docs/QUANTUM_THEORY.md) - Mathematical foundations
- [NVIDIA Integration](docs/NVIDIA_INTEGRATION.md) - DPU/GPU acceleration

---

## Roadmap

- [x] Core adaptive kernel
- [x] HFT/Gaming presets
- [x] Criterion benchmarks
- [ ] DirectStorage hook
- [ ] WSL2 kernel module
- [ ] CUDA batch kernel

---

## Citation

```bibtex
@article{sharma2024aether,
  title={AETHER - Adaptive Event-driven Threshold Hybrid Entangled Rendering},
  author={Sharma, Teerth},
  journal={ResearchGate},
  year={2024},
  doi={10.13140/RG.2.2.22443.91687}
}
```

---

## License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE).

---

<div align="center">

**[⬆ Back to Top](#aether-link)**

</div>
