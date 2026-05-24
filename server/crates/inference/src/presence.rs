//! Presence head — answers "is there a human in the scene?".

use serde::{Deserialize, Serialize};
use wavesight_core::{ConfidenceInterval, FusedScene, Result, Uncertainty};

use crate::Head;

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
