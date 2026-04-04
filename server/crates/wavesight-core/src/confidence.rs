//! Calibrated uncertainty types (see ADR-004 — Honest Mode).

use serde::{Deserialize, Serialize};

/// Symmetric or asymmetric confidence interval on a scalar prediction.
///
/// `low` and `high` bound the credible region at the configured level
/// (default 90 %). Both endpoints are inclusive.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ConfidenceInterval {
    /// Lower bound of the credible region.
    pub low: f32,
    /// Upper bound of the credible region.
    pub high: f32,
    /// Credible-region level in `(0, 1)`; typically `0.9`.
    pub level: f32,
}

impl ConfidenceInterval {
    /// Width of the interval. Larger means more uncertain.
    #[must_use]
    pub fn width(&self) -> f32 {
        (self.high - self.low).abs()
    }

    /// `true` if the interval is "tight" relative to the given scale.
    #[must_use]
    pub fn is_tight(&self, scale: f32) -> bool {
        self.width() < scale
    }
}

/// Epistemic uncertainty on a [0, 1] scale. 0 = fully confident, 1 = no signal.
///
/// Above [`Uncertainty::REFUSE_THRESHOLD`] the inference layer is expected to
/// emit `None` for the point estimate; the dashboard renders "uncertain".
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Uncertainty(pub f32);

impl Uncertainty {
    /// Above this value the model refuses to predict.
    pub const REFUSE_THRESHOLD: Self = Self(0.7);

    /// `true` if the model should refuse to commit to a point estimate.
    #[must_use]
    pub fn should_refuse(self) -> bool {
        self.0 >= Self::REFUSE_THRESHOLD.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interval_width() {
        let ci = ConfidenceInterval {
            low: 1.0,
            high: 3.0,
            level: 0.9,
        };
        assert!((ci.width() - 2.0).abs() < f32::EPSILON);
    }

    #[test]
    fn refuse_threshold() {
        assert!(Uncertainty(0.71).should_refuse());
        assert!(!Uncertainty(0.69).should_refuse());
    }
}
