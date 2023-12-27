//! Gaming (DirectStorage) preset example for AETHER-Link.
//!
//! Gaming workloads with DirectStorage feature:
//! - Large, sparse texture/mesh loads
//! - Burst I/O patterns (level transitions)
//! - Streamed assets with variable stride
//!
//! Run with: cargo run --example gaming --features tokio-runtime

use aether_link::AetherLinkKernel;

fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║      🎮 AETHER-Link Gaming / DirectStorage Demo 🚀        ║");
    println!("╠════════════════════════════════════════════════════════════╣");
    println!("║  Optimised for: large asset loads, burst patterns,         ║");
    println!("║  minimal CPU overhead during gameplay                      ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();

    // Gaming preset: more conservative than HFT
    let mut kernel = AetherLinkKernel::new_gaming();
    println!("📊 Gaming Kernel parameters:");
    println!("   ε (epsilon): {:.4}", kernel.epsilon);
    println!("   φ (phi):     {:.6}", kernel.phi);
    println!(
        "   λ (lambda):  [{:.3}, {:.3}, {:.3}]",
        kernel.lambda[0], kernel.lambda[1], kernel.lambda[2]
    );
    println!("   bias:        {:.4}", kernel.bias);
    println!();

    // Simulate gaming I/O: texture loads, mesh loads, streaming
    println!("📥 Simulating game load sequence:");
    println!();

    let game_scenarios = vec![
        (
            "Level start (large mesh)",
            vec![
                100_000, 100_001, 100_002, 100_003, 100_004, 100_005, 100_006, 100_007,
            ],
        ),
        (
            "Texture streaming",
            vec![200_000, 200_010, 200_020, 200_030, 200_040, 200_050],
        ),
        (
            "Shader pre-fetch",
            vec![300_000, 300_001, 300_002, 300_003, 300_100, 300_101],
        ),
        (
            "Background streaming",
            vec![400_000, 400_001, 400_002, 400_500, 400_501, 400_502],
        ),
        (
            "Texture MIP chain",
            vec![500_000, 500_001, 500_128, 500_256, 500_384, 500_512],
        ),
    ];

    let mut total_prefetches = 0u64;

    for (name, lbas) in &game_scenarios {
        let should_prefetch = kernel.process_io_cycle(lbas);
        let icon = if should_prefetch {
            "⚡ PREFETCH"
        } else {
            "⏳ defer"
        };
        println!(
            "  {}: {} — {}",
            name,
            icon,
            if should_prefetch {
                "GPU direct!"
            } else {
                "OS cache"
            }
        );
        if should_prefetch {
            total_prefetches += 1;
        }
    }

    println!();
    println!("📈 Gaming I/O Statistics:");
    println!("   Total asset groups: {}", game_scenarios.len());
    println!(
        "   DirectStorage triggers: {} / {} ({:.0}%)",
        total_prefetches,
        game_scenarios.len(),
        100.0 * total_prefetches as f32 / game_scenarios.len() as f32
    );
    println!();
    println!("💡 With DirectStorage + AETHER-Link:");
    println!("   GPU reads directly from NVMe, bypassing CPU/OS page cache.");
    println!("   AETHER-Link predicts which assets to stage before GPU needs them.");
}
