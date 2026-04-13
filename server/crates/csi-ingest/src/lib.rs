//! CSI ingestion layer.
//!
//! Accepts authenticated CSI frame streams from sensing nodes over WebSocket
//! and gRPC, validates and decrypts them, and forwards them to the DSP layer
//! via per-node Tokio channels.

#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

use tokio::sync::mpsc;
use wavesight_core::{CsiFrame, NodeId, Result};

/// Per-node CSI frame channel sender.
pub type CsiSender = mpsc::Sender<CsiFrame>;

/// Per-node CSI frame channel receiver.
pub type CsiReceiver = mpsc::Receiver<CsiFrame>;

/// Configuration for the ingest layer.
#[derive(Debug, Clone)]
pub struct IngestConfig {
    /// Bind address for the WebSocket listener.
    pub listen: String,
    /// 32-byte mesh PSK used to authenticate and decrypt incoming frames.
    pub mesh_psk: [u8; 32],
    /// Maximum buffered frames per node.
    pub per_node_buffer: usize,
}

impl Default for IngestConfig {
    fn default() -> Self {
        Self {
            listen: "0.0.0.0:8080".to_string(),
            mesh_psk: [0; 32],
            per_node_buffer: 256,
        }
    }
}

/// Spawn the ingest server. Returns immediately; actual work runs in tasks.
///
/// # Errors
/// Returns an error if the listener fails to bind.
pub async fn serve(_config: IngestConfig) -> Result<()> {
    // M1 implementation lands with the firmware. See ROADMAP.md.
    Ok(())
}

/// Internal API placeholder for downstream subscribers.
#[must_use]
pub fn channel_for_node(_node: &NodeId, buffer: usize) -> (CsiSender, CsiReceiver) {
    mpsc::channel(buffer)
}
