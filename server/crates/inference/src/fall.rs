//! Fall-detection head — specialised for the EldGuard vertical.
//!
//! Latency-optimised: a fall must be flagged within 2 seconds of the
//! initial impact for the mobile-app push notification to feel useful.

use serde::{Deserialize, Serialize};
use wavesight_core::{FusedScene, Result, Uncertainty};

use crate::Head;

/// Coarse severity assigned to a detected fall event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FallSeverity {
    /// Slow descent, controlled landing.
    Soft,
    /// Hard impact, no follow-up motion observed.
    Hard,
    /// Hard impact followed by no motion for ≥ 30 seconds.
    Unresponsive,
}

/// One detected fall event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FallEvent {
    /// Severity bucket.
    pub severity: FallSeverity,
    /// Epistemic uncertainty of the detection.
    pub uncertainty: Uncertainty,
}

/// Stub fall head — placeholder until M4.
pub struct FallHead;

impl Head for FallHead {
    type Output = Option<FallEvent>;

    fn predict(&mut self, _scene: &FusedScene) -> Result<Self::Output> {
        Ok(None)
    }
}
