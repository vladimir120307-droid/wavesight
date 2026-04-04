//! Shared types for the WaveSight server.
//!
//! Calibrated uncertainty types implement the "Honest Mode" policy: every
//! shipped prediction must carry an explicit confidence interval and an
//! epistemic uncertainty scalar.

#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::pedantic)]

mod confidence;
mod error;
mod frame;
mod node;

pub use confidence::{ConfidenceInterval, Uncertainty};
pub use error::{Error, Result};
pub use frame::{CsiFrame, DspFrame, FrameMetadata, SubcarrierCount};
pub use node::{NodeId, NodeKind};