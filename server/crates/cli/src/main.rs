//! `wavesight` command-line entry point.

use std::net::SocketAddr;

use api::{serve_api, serve_ingest, ApiConfig, AppState};
use clap::{Parser, Subcommand};
use csi_ingest::IngestHub;

#[derive(Debug, Parser)]
#[command(name = "wavesight", version, about = "WaveSight edge server CLI")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Run the edge server (ingest + API in one process).
    Serve {
        /// HTTP / WebSocket bind address for the dashboard-facing API.
        #[arg(long, default_value = "0.0.0.0:8081")]
        api: SocketAddr,
        /// WebSocket bind address for ESP32 nodes to push CSI.
        #[arg(long, default_value = "0.0.0.0:8080")]
        ingest: SocketAddr,
    },
    /// Run the honest benchmark suite against a recorded dataset.
    Bench {
        /// Path to the dataset directory or HuggingFace ref.
        dataset: String,
    },
    /// Print configuration and exit.
    Doctor,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,wavesight=debug".into()),
        )
        .init();

    let cli = Cli::parse();
    match cli.command {
        Command::Serve { api: api_addr, ingest } => {
            tracing::info!(%api_addr, %ingest, "wavesight edge server starting");
            let hub = IngestHub::default();
            let state = AppState::new(hub.clone());
            let cfg = ApiConfig {
                http_listen: api_addr,
                ingest_listen: ingest,
            };

            let api_task = tokio::spawn(serve_api(state, cfg.http_listen));
            let ingest_task = tokio::spawn(serve_ingest(hub, cfg.ingest_listen));

            tokio::select! {
                r = api_task => r??,
                r = ingest_task => r??,
            }
        }
        Command::Bench { dataset } => {
            tracing::info!(%dataset, "honest benchmark stub — not yet implemented");
        }
        Command::Doctor => {
            println!("WaveSight {} — configuration OK", env!("CARGO_PKG_VERSION"));
        }
    }
    Ok(())
}
