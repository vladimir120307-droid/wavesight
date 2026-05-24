//! Live WebSocket fan-out to dashboard clients.

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
};
use dsp::{process, DspConfig};
use futures_util::{SinkExt, StreamExt};
use serde::Serialize;
use tracing::{debug, warn};

use crate::AppState;

/// Single update pushed to a dashboard client.
#[derive(Debug, Clone, Serialize)]
pub struct StreamUpdate {
    /// ISO 8601 timestamp.
    pub timestamp: String,
    /// Originating node.
    pub node: String,
    /// Sequence number.
    pub sequence: u64,
    /// Receiver signal strength.
    pub rssi_dbm: i8,
    /// Per-subcarrier amplitude (downsampled to <=128 bins for dashboard).
    pub amplitude: Vec<f32>,
}

/// GET /api/v1/stream — WebSocket upgrade.
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| serve_client(socket, state))
}

async fn serve_client(socket: WebSocket, state: AppState) {
    let mut rx = state.hub.subscribe();
    let (mut sink, _stream) = socket.split();
    let cfg = DspConfig::default();

    loop {
        let frame = match rx.recv().await {
            Ok(f) => f,
            Err(tokio::sync::broadcast::error::RecvError::Lagged(n)) => {
                debug!(skipped = n, "dashboard client lagging, skipping frames");
                continue;
            }
            Err(tokio::sync::broadcast::error::RecvError::Closed) => break,
        };

        let dsp_frame = match process(&frame, &cfg) {
            Ok(f) => f,
            Err(err) => {
                debug!(?err, "dsp failed on live frame");
                continue;
            }
        };

        let downsampled = downsample(&dsp_frame.amplitude, 64);
        let update = StreamUpdate {
            timestamp: dsp_frame.metadata.captured_at.to_rfc3339(),
            node: dsp_frame.metadata.node.0,
            sequence: dsp_frame.metadata.sequence,
            rssi_dbm: dsp_frame.metadata.rssi_dbm,
            amplitude: downsampled,
        };

        let payload = match serde_json::to_string(&update) {
            Ok(s) => s,
            Err(err) => {
                warn!(?err, "serialize failed");
                continue;
            }
        };
        if sink.send(Message::Text(payload)).await.is_err() {
            break;
        }
    }
}

fn downsample(input: &[f32], target: usize) -> Vec<f32> {
    if input.len() <= target {
        return input.to_vec();
    }
    let bucket = (input.len() as f32 / target as f32).ceil() as usize;
    input
        .chunks(bucket)
        .map(|c| c.iter().sum::<f32>() / c.len() as f32)
        .collect()
}
