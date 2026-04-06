//! Shared types for the WaveSight server.
//!
//! This crate defines the core data structures used across all server crates:
//! CSI frames, DSP frames, fused scenes, predictions with calibrated
//! uncertainty (see ADR-004), and error variants.

#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::pedantic)]

mod confidence;
mod entity;
mod error;
mod frame;
mod node;
mod scene;

pub use confidence::{ConfidenceInterval, Uncertainty};
pub use entity::{Entity, EntityClass, EntityId};
pub use error::{Error, Result};
pub use frame::{CsiFrame, DspFrame, FrameMetadata, SubcarrierCount};
pub use node::{NodeId, NodeKind};
pub use scene::FusedScene;
