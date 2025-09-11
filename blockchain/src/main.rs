//! Optimic Core Library Test
//! 
//! This is a simple test binary to verify the blockchain core functionality

use optimic_core::{init_node, VERSION, NAME};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    println!("🚀 {} v{}", NAME, VERSION);
    println!("Initializing Optimic blockchain core...");

    // Test node initialization
    init_node().await?;

    println!("✅ Optimic core initialized successfully!");
    
    Ok(())
}
