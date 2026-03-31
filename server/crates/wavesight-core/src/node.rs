//! Sensing-node identity and hardware class.

use serde::{Deserialize, Serialize};

/// Opaque, persistent identifier for a sensing node.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub String);

impl NodeId {
    /// Construct from a string slice.
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
}

/// Hardware family of a sensing node.
///
/// Used by the fusion layer to set process-noise priors and by the dashboard
/// to render capability icons.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeKind {
    /// ESP32-S3, single-antenna WiFi 4/5.
    Esp32S3,
    /// ESP32-C5, WiFi 6 2x2 MIMO.
    Esp32C5,
    /// ESP32-C6, WiFi 6 2x2 MIMO.
    Esp32C6,
    /// DecaWave / Qorvo DWM3000 UWB anchor.
    Dwm3000Uwb,
    /// Raspberry Pi with nexmon_csi.
    NexmonPi,
    /// Intel AX210 reference NIC.
    IntelAx210,
    /// Nordic nRF7002 reference NIC.
    Nrf7002,
}
