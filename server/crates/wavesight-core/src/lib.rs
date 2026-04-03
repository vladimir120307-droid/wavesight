//! Shared types for the WaveSight server.

#![forbid(unsafe_code)]

mod error;
mod frame;
mod node;

pub use error::{Error, Result};
pub use frame::{CsiFrame, DspFrame, FrameMetadata, SubcarrierCount};
pub use node::{NodeId, NodeKind};