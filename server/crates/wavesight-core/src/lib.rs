//! Shared types for the WaveSight server.

#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::pedantic)]

mod confidence;
mod entity;
mod error;
mod frame;
mod node;

pub use confidence::{ConfidenceInterval, Uncertainty};
pub use entity::{Entity, EntityClass, EntityId};
pub use error::{Error, Result};
pub use frame::{CsiFrame, DspFrame, FrameMetadata, SubcarrierCount};
pub use node::{NodeId, NodeKind};