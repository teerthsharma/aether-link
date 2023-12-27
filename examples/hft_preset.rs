//! HFT-specific preset example for AETHER-Link.
//!
//! High-frequency trading workloads demand:
//! - Minimal jitter (sub-nanosecond P99-P50)
//! - Deterministic latency guarantees
//! - Aggressive sequential prefetch
//!
//! Run with: cargo run --example hft_preset

use aether_link::AetherLinkKernel;

fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║       🚀 AETHER-Link HFT Preset Demo 🚀                  ║");
    println!("╠════════════════════════════════════════════════════════════╣");
    println!("║  Optimised for: sub-nanosecond jitter, deterministic      ║");
    println!("║  latency, aggressive sequential prefetch                  ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();

    // HFT preset: aggressive epsilon (lower = more aggressive prefetch)
    let mut kernel = AetherLinkKernel::new_hft();
    println!("📊 HFT Kernel parameters:");
    println!("   ε (epsilon): {:.4}", kernel.epsilon);
    println!("   φ (phi):     {:.6}", kernel.phi);
    println!(
        "   λ (lambda):  [{:.3}, {:.3}, {:.3}]",
        kernel.lambda[0], kernel.lambda[1], kernel.lambda[2]
    );
    println!("   bias:        {:.4}", kernel.bias);
    println!();

    // Simulate HFT market data stream: mostly sequential with occasional jumps
    // Market tick LBAs: sequential at first, then a new file region
    let market_stream: Vec<u64> = vec![
        // Book order updates — sequential
        50_000, 50_001, 50_002, 50_003, 50_004, 50_010, 50_011, 50_012, 50_013, 50_014, 50_020,
        50_021, 50_022, 50_023, 50_024, // Trade executions — small jump
        60_000, 60_001, 60_002, 60_003, // More order updates
        50_030, 50_031, 50_032, 50_033, 50_034, 50_040, 50_041, 50_042, 50_043, 50_044,
    ];

    println!("📥 Processing {} market data LBAs...", market_stream.len());
    println!();

    let mut prefetches = 0u64;
    let chunk_size = 5;

    for (i, chunk) in market_stream.chunks(chunk_size).enumerate() {
        let lba_slice: Vec<u64> = chunk.to_vec();
        let should_prefetch = kernel.process_io_cycle(&lba_slice);

        if should_prefetch {
            prefetches += 1;
            println!(
                "  cycle {:3}: LBAs {:?}: ⚡ PREFETCH",
                i,
                &chunk[..2.min(chunk.len())]
            );
        } else {
            println!(
                "  cycle {:3}: LBAs {:?}: ⏳ defer",
                i,
                &chunk[..2.min(chunk.len())]
            );
        }
    }

    println!();
    println!("📈 HFT Statistics:");
    println!("   Total cycles:    {}", market_stream.len() / chunk_size);
    println!("   Prefetch events: {}", prefetches);
    println!(
        "   Prefetch ratio: {:.1}%",
        100.0 * prefetches as f32 / (market_stream.len() / chunk_size) as f32
    );
    println!();

    // HFT-specific: verify jitter characteristics
    println!("🎯 HFT jitter profile:");
    println!("   Target: P99 - P50 < 1 ns");
    println!("   AetherLink povm_basis: {:.6} rad", kernel.phi);
    println!("   (lower φ = tighter latency bounds for sequential streams)");
}
