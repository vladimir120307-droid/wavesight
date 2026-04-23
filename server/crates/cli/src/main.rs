//! `wavesight` command-line entry point.

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "wavesight", version, about = "WaveSight edge server CLI")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Run the edge server.
    Serve {
        /// HTTP / WebSocket bind address.
        #[arg(long, default_value = "0.0.0.0:8080")]
        listen: String,
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
        Command::Serve { listen } => {
            tracing::info!(%listen, "wavesight edge server starting");
            let cfg = api::ApiConfig {
                http_listen: listen.parse()?,
                ..Default::default()
            };
            api::serve(cfg).await?;
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
