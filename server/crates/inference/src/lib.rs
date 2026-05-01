//! On-edge model inference.
//!
//! All heads expose calibrated uncertainty (see ADR-004 — Honest Mode).

#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

mod fall;
mod pose;
mod presence;
mod registry;
mod sleep;
mod vitals;

pub use fall::{FallEvent, FallHead, FallSeverity};
pub use pose::{Keypoint, PoseHead, PosePrediction};
pub use presence::{PresenceHead, PresencePrediction};
pub use registry::{ModelRef, ModelRegistry};
pub use sleep::{SleepHead, SleepPrediction, SleepStage};
pub use vitals::{VitalsHead, VitalsPrediction};

use wavesight_core::{FusedScene, Result};

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
