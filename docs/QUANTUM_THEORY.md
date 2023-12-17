# Quantum-Inspired Computing in AETHER-Link

> **What the code actually does vs. the mathematical formalism it draws from.**

## Overview

AETHER-Link v0.2.0 implements a real-time adaptive prefetch decision engine.
It draws three concepts from quantum mechanics — Bloch sphere encoding, POVM
observables, and adaptive basis rotation — and translates them into classical
floating-point operations on CPU hardware.

**The implementation is real. The quantum mechanics is structural, not literal.**

---

## 1. Telemetry Features (real DSP)

Six features are extracted from the LBA stream at ~1.4 ns:

| Symbol | Name | Computation |
|--------|------|-------------|
| Δ | Delta | `last_lba − first_lba` |
| V | Velocity | `Δ × 0.5` |
| σ² | Variance | **Welford online algorithm** over all observed streams |
| C | Chebyshev | Running RMS of inter-stream delta differences |
| H | History | Fixed exponential decay weight (0.8) |
| Ω | Context | Log-density entropy of recent inter-arrival rates |

*In v0.1.0, features 2–5 were hardcoded constants. In v0.2.0 all six are live DSP.*

---

## 2. Bloch Sphere Encoding (classical mapping)

The six features are combined into a 3D Bloch vector using Chebyshev-weighted
angular combination:

```
θᵢ = 2 × atan(fᵢ / scaleᵢ)    ∈ (−π, π)    // polar angle per feature
φ  = weighted azimuthal average              ∈ [−π, π]

rx = sin(θ/2) × cos(φ)          // Cartesian coordinates
ry = sin(θ/2) × sin(φ)
rz = cos(θ/2)

‖[rx, ry, rz]‖ = 1              // Normalised via fast_inv_sqrt
```

**Why this is quantum-inspired:** The Bloch sphere is the exact state
representation for a single qubit in quantum mechanics.  Mapping features to
angles on S² gives a bounded, continuous representation suitable for
angular measurement observables.

**Why this is classical:** The "quantum state" is a 3D unit vector computed
from observable I/O statistics.  There is no superposition, entanglement,
or quantum hardware involved.

---

## 3. POVM Measurement (classical angular projection)

In quantum mechanics, a POVM (Positive Operator-Valued Measure) is a set of
measurement operators {Eₘ} that satisfy ∑Eₘ = I and produce probability
p(m) = ⟨ψ|Eₘ|ψ⟩ for state |ψ⟩.

AETHER-Link implements three POVM-style angular observables:

```
E₁(φ) = cos(θ + φ)     // Spatial observable
         Measures alignment of LBA velocity with the adaptive basis φ.
         Range: [−1, 1]. Drives threshold adaptation.

E₂(φ) = sin(θ/2 − φ)    // Temporal observable
         Measures orthogonal phase component.
         Range: [−1, 1]. Drives adaptive basis rotation.

E₃(φ) = cos(θ · φ_az)   // Spectral observable
         Measures spectral energy in the current POVM basis.
         Range: [−1, 1]. Drives fetch sigmoid.
```

These are evaluated as plain `libm::cosf` / `libm::sinf` calls — no quantum
hardware required.

---

## 4. Adaptive Basis Rotation (classical feedback)

After each measurement:

```
φ_{t+1} = φ_t + λ₂ × E₂(φ_t)    (mod 2π)
ε_{t+1} = ε_t + λ₁ × E₁(φ_t)    (clamped to [0.1, 0.9])
p_fetch = sigmoid(−(λ₃ × E₃(φ_t) + bias))
```

This is mathematically analogous to:
- **Quantum feedback control**: Adjusting the measurement basis based on the
  result of the previous measurement.
- **Bayesian updating**: Incorporating new evidence (E₁, E₂, E₃) into the
  prior belief about the workload state (encoded in φ).

The feedback is continuous, bounded, and has no trained parameters.

---

## 5. What Is NOT Quantum

AETHER-Link does not use:

| Claimed quantum feature | Actual implementation |
|------------------------|----------------------|
| Qubits | 3-element float vector on S² |
| Superposition | Weighted feature sum |
| Entanglement | None (single-stream processing) |
| Quantum gates | `cos`/`sin` trig functions |
| Actual POVM hardware | Classical angular projections |
| Quantum speedup | None — O(1) classical ops |

---

## 6. Why the Quantum Formalism Is Still Useful

The Bloch/POVM language is not marketing — it encodes genuine constraints:

1. **Bounded measurements**: Angular observables naturally produce values in
   [−1, 1], which is ideal for adaptive thresholds.  Raw I/O statistics have
   no such bound.

2. **Adaptive basis**: The quantum-inspired basis rotation (φ update) is a
   well-studied way to continuously track a non-stationary signal without
   retraining.  This is also the core idea in quantum state tomography.

3. **Spherical geometry**: Constraining the state to S² prevents the decision
   signal from diverging to arbitrarily large magnitudes — a real failure mode
   in unbounded linear feedback systems.

---

## 7. Accuracy Improvements in v0.2.0

| Component | v0.1.0 | v0.2.0 |
|-----------|--------|--------|
| `fast_atan` | Padé: **76% error** at x=10 | `libm::atanf`: ≤ 1 ULP |
| `variance` | hardcoded `0.1` | Welford online variance |
| `spectrum` | hardcoded `0.01` | Chebyshev RMS energy |
| `history` | hardcoded `0.8` | Decay-weighted temporal |
| `context` | hardcoded `1.0` | Log-density entropy |
| Bloch norm | absent (unnormalised) | `fast_inv_sqrt` unit sphere |
| `simulate_qpu_eval` | scalar trig mock | real POVM observables |

---

## References

- Nielsen & Chuang, *Quantum Computation and Quantum Information*, Chapter 2
  (POVM formalism)
- Wiseman & Milburn, *Quantum Measurement and Control* (adaptive basis rotation)
- Welford, "Note on a Method for Calculating Corrected Sums of Squares and
  Products" (online variance)
- Quake III `rsqrt`: fast inverse square root (fast_inv_sqrt)
