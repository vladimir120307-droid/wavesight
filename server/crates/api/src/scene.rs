//! REST handlers exposing the latest scene snapshot.

use axum::{extract::State, Json};
use serde::Serialize;
use wavesight_core::Uncertainty;

use crate::AppState;

/// Single-entity payload as exposed over REST.
#[derive(Debug, Clone, Serialize)]
pub struct ScenePayload {
    /// ISO 8601 timestamp the snapshot was computed at.
    pub timestamp: String,
    /// Per-node presence indications.
    pub presence: Vec<NodePresence>,
}

/// Per-node presence summary.
#[derive(Debug, Clone, Serialize)]
pub struct NodePresence {
    /// Node identifier.
    pub node: String,
    /// `true` when the rule-based detector currently fires.
    pub present: bool,
    /// Energy proxy on the configured window.
    pub energy: f32,
    /// Epistemic uncertainty (0 = sure, 1 = unknown).
    pub uncertainty: Uncertainty,
}

/// GET /api/v1/scene
pub async fn current_scene(State(state): State<AppState>) -> Json<ScenePayload> {
    let guard = state.latest_scene.read().await;
    let payload = guard.clone().unwrap_or_else(|| ScenePayload {
        timestamp: chrono::Utc::now().to_rfc3339(),
        presence: vec![],
    });
    Json(payload)
}

/// GET /api/v1/nodes
pub async fn nodes(State(state): State<AppState>) -> Json<Vec<String>> {
    Json(state.hub.nodes().into_iter().map(|n| n.0).collect())
}
