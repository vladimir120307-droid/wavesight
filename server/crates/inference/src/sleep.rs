//! Sleep-staging head — the SleepWave vertical.
//!
//! Emits a coarse 4-stage hypnogram aligned to AASM categories.

use serde::{Deserialize, Serialize};
use wavesight_core::{FusedScene, Result, Uncertainty};

use crate::Head;

/// AASM-aligned sleep stage at the current epoch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SleepStage {
    /// Awake.
    Wake,
    /// Light sleep (N1+N2).
    Light,
    /// Deep / slow-wave sleep (N3).
    Deep,
    /// REM.
    Rem,
}

/// Output of the sleep head.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SleepPrediction {
    /// Current stage.
    pub stage: SleepStage,
    /// Per-stage posterior probability.
    pub posterior: [f32; 4],
    /// Epistemic uncertainty.
    pub uncertainty: Uncertainty,
}

/// Stub sleep head — placeholder until M5.
pub struct SleepHead;

impl Head for SleepHead {
    type Output = SleepPrediction;

    fn predict(&mut self, _scene: &FusedScene) -> Result<Self::Output> {
        Ok(SleepPrediction {
            stage: SleepStage::Wake,
            posterior: [1.0, 0.0, 0.0, 0.0],
            uncertainty: Uncertainty(1.0),
        })
    }
}
