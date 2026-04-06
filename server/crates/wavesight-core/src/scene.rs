//! Fused, multi-node scene representation.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::entity::Entity;

/// One instantaneous snapshot of the room as the fusion layer sees it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FusedScene {
    /// PTP-synchronised timestamp for this scene snapshot.
    pub timestamp: DateTime<Utc>,
    /// Tracked entities in the scene.
    pub entities: Vec<Entity>,
    /// Number of nodes that contributed to this snapshot.
    pub contributing_nodes: u8,
    /// Observed PTP RMS jitter in microseconds for this window.
    pub ptp_rms_us: f32,
}
