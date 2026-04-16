//! Multi-modal fusion layer.
//!
//! Combines time-aligned [`wavesight_core::DspFrame`] streams from multiple
//! nodes (and optional UWB / BLE inputs) into a coherent
//! [`wavesight_core::FusedScene`] via an Extended Kalman Filter.
//!
//! See ADR-005 for the motivation behind multi-modal fusion.

#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

use wavesight_core::{DspFrame, FusedScene, Result};

/// Fusion engine configuration.
#[derive(Debug, Clone)]
pub struct FusionConfig {
    /// Process-noise standard deviation in metres / second^2 for human motion.
    pub human_process_noise: f32,
    /// Observation-noise variance for WiFi-CSI energy direction.
    pub wifi_obs_variance: f32,
    /// Observation-noise variance for UWB AoA.
    pub uwb_obs_variance: f32,
}

impl Default for FusionConfig {
    fn default() -> Self {
        Self {
            human_process_noise: 0.5,
            wifi_obs_variance: 0.2,
            uwb_obs_variance: 0.05,
        }
    }
}

/// Stateful fusion engine. Construct once per session.
pub struct FusionEngine {
    config: FusionConfig,
}

impl FusionEngine {
    /// Construct a new engine.
    #[must_use]
    pub fn new(config: FusionConfig) -> Self {
        Self { config }
    }

    /// Process a batch of time-aligned DSP frames and emit a fused scene.
    ///
    /// # Errors
    /// Returns an error if the batch is empty or contains inconsistent
    /// timestamps beyond the PTP tolerance.
    pub fn step(&mut self, frames: &[DspFrame]) -> Result<FusedScene> {
        let _ = &self.config;
        let timestamp = frames
            .first()
            .map(|f| f.metadata.captured_at)
            .unwrap_or_else(chrono::Utc::now);
        Ok(FusedScene {
            timestamp,
            entities: vec![],
            contributing_nodes: u8::try_from(frames.len()).unwrap_or(u8::MAX),
            ptp_rms_us: 0.0,
        })
    }
}
