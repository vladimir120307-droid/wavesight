//! On-edge model inference.
//!
//! All heads expose calibrated uncertainty (see ADR-004 — Honest Mode).

#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

use serde::{Deserialize, Serialize};
use wavesight_core::{ConfidenceInterval, FusedScene, Result, Uncertainty};

/// Output of the presence head.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresencePrediction {
    /// Posterior probability that a human is present.
    pub probability: f32,
    /// Credible interval on the probability.
    pub interval: ConfidenceInterval,
    /// Epistemic uncertainty.
    pub uncertainty: Uncertainty,
}

/// Output of the vitals head.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VitalsPrediction {
    /// Estimated heart-rate in beats per minute, or `None` when uncertain.
    pub hr_bpm: Option<f32>,
    /// Credible interval on `hr_bpm` (always populated even when `hr_bpm`
    /// is `None`, so the UI can show the band).
    pub hr_interval: ConfidenceInterval,
    /// Estimated breathing rate in breaths per minute, or `None` when uncertain.
    pub br_bpm: Option<f32>,
    /// Credible interval on `br_bpm`.
    pub br_interval: ConfidenceInterval,
    /// Epistemic uncertainty.
    pub uncertainty: Uncertainty,
}

/// Trait every inference head implements.
pub trait Head {
    /// Prediction emitted by this head.
    type Output;

    /// Run inference on a single fused scene.
    ///
    /// # Errors
    /// Implementation-specific.
    fn predict(&mut self, scene: &FusedScene) -> Result<Self::Output>;
}

/// Stub presence head — replaced with a Candle-loaded model in M2.
pub struct PresenceHead;

impl Head for PresenceHead {
    type Output = PresencePrediction;

    fn predict(&mut self, scene: &FusedScene) -> Result<Self::Output> {
        let prob = if scene.entities.is_empty() { 0.0 } else { 1.0 };
        Ok(PresencePrediction {
            probability: prob,
            interval: ConfidenceInterval {
                low: prob,
                high: prob,
                level: 0.9,
            },
            uncertainty: Uncertainty(0.5),
        })
    }
}
