//! Tokio async runtime integration example for AETHER-Link.
//!
//! Run with: cargo run --example tokio_integration --features tokio-runtime

use aether_link::AetherLinkKernel;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Async wrapper for AetherLinkKernel with tokio runtime support.
/// Allows non-blocking I/O decision processing in async contexts.
#[derive(Clone)]
pub struct AsyncAetherLink {
    kernel: Arc<RwLock<AetherLinkKernel>>,
}

impl AsyncAetherLink {
    /// Creates a new async-wrapped kernel with default settings.
    pub fn new() -> Self {
        Self {
            kernel: Arc::new(RwLock::new(AetherLinkKernel::default())),
        }
    }

    /// Creates a new async-wrapped kernel with HFT preset.
    pub fn new_hft() -> Self {
        Self {
            kernel: Arc::new(RwLock::new(AetherLinkKernel::new_hft())),
        }
    }

    /// Creates a new async-wrapped kernel with gaming preset.
    pub fn new_gaming() -> Self {
        Self {
            kernel: Arc::new(RwLock::new(AetherLinkKernel::new_gaming())),
        }
    }

    /// Process I/O cycle asynchronously.
    /// Returns true if prefetch is recommended.
    pub async fn process_io_cycle(&self, lba_stream: &[u64]) -> bool {
        let mut kernel = self.kernel.write().await;
        kernel.process_io_cycle(lba_stream)
    }

    /// Get current prefetch ratio.
    pub async fn prefetch_ratio(&self) -> f32 {
        let kernel = self.kernel.read().await;
        kernel.prefetch_ratio()
    }
}

impl Default for AsyncAetherLink {
    fn default() -> Self {
        Self::new()
    }
}

#[tokio::main]
async fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║  🚀 AETHER-Link Tokio Async Runtime Integration Demo 🚀     ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();

    let async_kernel = AsyncAetherLink::new();
    println!("📊 Async kernel initialized with default parameters");
    println!();

    // Simulate async I/O stream processing
    let lba_stream: Vec<u64> = vec![
        1000, 1001, 1002, 1003, 1004, 1010, 1011, 1012, 2000, 2001, 2002, 2003, 2004, 2005, 2006,
    ];

    println!(
        "📥 Processing async LBA stream ({} addresses):",
        lba_stream.len()
    );
    println!("   {:?}", lba_stream);
    println!();

    let should_prefetch = async_kernel.process_io_cycle(&lba_stream).await;

    println!(
        "⚡ Decision: {}",
        if should_prefetch {
            "PREFETCH → Bypass OS, use DirectStorage!"
        } else {
            "STANDARD → Use OS page cache"
        }
    );
    println!();

    // Demonstrate concurrent processing
    let stream1 = async_kernel.clone();
    let stream2 = async_kernel.clone();

    let (result1, result2) = tokio::join!(
        async move {
            let data: Vec<u64> = (0..10).map(|i| i * 100).collect();
            stream1.process_io_cycle(&data).await
        },
        async move {
            let data: Vec<u64> = (0..10).map(|i| i * 200).collect();
            stream2.process_io_cycle(&data).await
        }
    );

    println!("🔄 Concurrent I/O cycle results:");
    println!("   Stream 1 prefetch: {}", result1);
    println!("   Stream 2 prefetch: {}", result2);
    println!();

    let ratio = async_kernel.prefetch_ratio().await;
    println!("📈 Overall prefetch ratio: {:.1}%", ratio * 100.0);
    println!();

    println!("✅ Tokio integration demo complete!");
    println!("   Use #[tokio::test] for async unit tests.");
}
