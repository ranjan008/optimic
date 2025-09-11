//! Optimic Node Binary
//! 
//! Main executable for running an Optimic blockchain node

use anyhow::Result;
use clap::{Parser, Subcommand};
use optimic_core::{init_node, VERSION, NAME};
use tracing::{info, Level};
use tracing_subscriber;

#[derive(Parser)]
#[command(name = NAME)]
#[command(version = VERSION)]
#[command(about = "Optimic Protocol - High-Performance Layer 1 Blockchain for Options Trading")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    /// Set the logging level
    #[arg(short, long, default_value = "info")]
    log_level: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the blockchain node
    Start {
        /// Configuration file path
        #[arg(short, long, default_value = "config.toml")]
        config: String,
        
        /// Genesis file path
        #[arg(short, long, default_value = "genesis.json")]
        genesis: String,
    },
    
    /// Initialize a new node
    Init {
        /// Chain ID
        #[arg(short, long, default_value = "optimic-1")]
        chain_id: String,
    },
    
    /// Show node version
    Version,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Initialize logging
    let log_level = match cli.log_level.as_str() {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::INFO,
    };
    
    tracing_subscriber::fmt()
        .with_max_level(log_level)
        .init();

    info!("Starting {} v{}", NAME, VERSION);

    match cli.command {
        Commands::Start { config, genesis } => {
            info!("Starting node with config: {} and genesis: {}", config, genesis);
            start_node(config, genesis).await?;
        }
        Commands::Init { chain_id } => {
            info!("Initializing new node for chain: {}", chain_id);
            init_node().await?;
        }
        Commands::Version => {
            println!("{} v{}", NAME, VERSION);
        }
    }

    Ok(())
}

async fn start_node(config_path: String, genesis_path: String) -> Result<()> {
    info!("Node configuration: {}", config_path);
    info!("Genesis file: {}", genesis_path);
    
    // TODO: Implement actual node startup logic
    info!("Node startup logic not yet implemented");
    
    Ok(())
}
