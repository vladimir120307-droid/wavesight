//! On-edge model inference.

#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

use serde::{Deserialize, Serialize};
use wavesight_core::{ConfidenceInterval, Uncertainty};

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
    /// Credible interval on `hr_bpm`.
    pub hr_interval: ConfidenceInterval,
    /// Estimated breathing rate in breaths per minute, or `None` when uncertain.
    pub br_bpm: Option<f32>,
    /// Credible interval on `br_bpm`.
    pub br_interval: ConfidenceInterval,
    /// Epistemic uncertainty.
    pub uncertainty: Uncertainty,
}