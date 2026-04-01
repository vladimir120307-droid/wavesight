//! Shared types for the WaveSight server.

#![forbid(unsafe_code)]

mod frame;
mod node;

pub use frame::{CsiFrame, FrameMetadata, SubcarrierCount};
pub use node::{NodeId, NodeKind};