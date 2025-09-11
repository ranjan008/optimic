//! Optimic Core - High-Performance Layer 1 Blockchain for Options Trading
//! 
//! This crate implements the core blockchain functionality for the Optimic protocol,
//! including consensus, state management, trading engine, and options infrastructure.

pub mod abci;
pub mod app;
pub mod consensus;
pub mod state;
pub mod storage;
pub mod types;
pub mod trading;
pub mod options;
pub mod collateral;

// Re-export core types for external use
pub use app::OptimicApp;
pub use types::*;

use anyhow::Result;
use tracing::{info, warn};

/// Initialize the Optimic blockchain node
pub async fn init_node() -> Result<()> {
    info!("Initializing Optimic blockchain node...");
    
    // TODO: Implement node initialization logic
    warn!("Node initialization not yet implemented");
    
    Ok(())
}

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_node_init() {
        assert!(init_node().await.is_ok());
    }
}
