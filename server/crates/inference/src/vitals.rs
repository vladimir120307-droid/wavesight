//! Vital signs head — heart rate and breathing rate from CSI.
//!
//! Trained against a Polar H10 / Withings reference; on commodity hardware
//! the breathing-rate channel is more accurate than HR.

use serde::{Deserialize, Serialize};
use wavesight_core::{ConfidenceInterval, FusedScene, Result, Uncertainty};

use crate::Head;

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

/// Stub vitals head — to be replaced with a trained 1-D CNN in M3.
pub struct VitalsHead;

impl Head for VitalsHead {
    type Output = VitalsPrediction;

    fn predict(&mut self, scene: &FusedScene) -> Result<Self::Output> {
        // While the trained model is absent we explicitly refuse to predict.
        let unsure = Uncertainty(0.9);
        let placeholder_ci = ConfidenceInterval {
            low: 0.0,
            high: 0.0,
            level: 0.9,
        };
        let any_present = !scene.entities.is_empty();
        Ok(VitalsPrediction {
            hr_bpm: None,
            hr_interval: placeholder_ci,
            br_bpm: None,
            br_interval: placeholder_ci,
            uncertainty: if any_present { unsure } else { Uncertainty(1.0) },
        })
    }
}
