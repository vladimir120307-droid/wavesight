//! axum router and WebSocket handler for the `/ingest` endpoint.

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use chrono::Utc;
use futures_util::stream::StreamExt;
use tracing::{debug, info, warn};
use wavesight_core::NodeId;

use crate::hub::IngestHub;
use crate::wire::{anchor_boot, item_into_frame, parse_batch};

/// Ingest-layer runtime configuration.
#[derive(Debug, Clone)]
pub struct IngestConfig {
    /// Maximum buffered frames per node.
    pub per_node_buffer: usize,
}

impl Default for IngestConfig {
    fn default() -> Self {
        Self {
            per_node_buffer: 256,
        }
    }
}

/// Build the ingest router. Mount under `/` on a dedicated port (the
/// firmware connects to `ws://server:PORT/ingest`).
#[must_use]
pub fn router(hub: IngestHub) -> Router {
    Router::new()
        .route("/ingest", get(ws_handler))
        .route("/healthz", get(healthz))
        .with_state(hub)
}

async fn healthz() -> &'static str {
    "ok"
}

async fn ws_handler(ws: WebSocketUpgrade, State(hub): State<IngestHub>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| serve_node(socket, hub))
}

async fn serve_node(mut socket: WebSocket, hub: IngestHub) {
    let mut boot_offset = None;
    let mut current_node: Option<NodeId> = None;
    let mut frames_seen: u64 = 0;

    while let Some(msg) = socket.next().await {
        let Ok(msg) = msg else {
            warn!("websocket read error, closing connection");
            break;
        };
        let text = match msg {
            Message::Text(t) => t,
            Message::Binary(_) => {
                warn!("binary frame on ingest endpoint, ignoring");
                continue;
            }
            Message::Close(_) => break,
            Message::Ping(_) | Message::Pong(_) => continue,
        };

        let batch = match parse_batch(&text) {
            Ok(b) => b,
            Err(err) => {
                warn!(?err, "malformed batch dropped");
                continue;
            }
        };

        let node = NodeId::new(batch.node.clone());
        if current_node.as_ref() != Some(&node) {
            hub.register(node.clone());
            info!(node = %batch.node, "node connected");
            current_node = Some(node.clone());
        }

        for item in &batch.batch {
            if boot_offset.is_none() {
                boot_offset = Some(anchor_boot(Utc::now(), item.ts_us));
            }
            let offset = boot_offset.expect("anchored above");
            match item_into_frame(item, &node, offset) {
                Ok(frame) => {
                    frames_seen += 1;
                    hub.push(frame);
                }
                Err(err) => {
                    debug!(?err, seq = item.seq, "frame decode failed, skipping");
                }
            }
        }
    }

    if let Some(n) = current_node {
        info!(node = %n.0, frames = frames_seen, "node disconnected");
    }
}
