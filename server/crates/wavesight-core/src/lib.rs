//! Shared types for the WaveSight server.

#![forbid(unsafe_code)]

mod frame;
mod node;

pub use frame::SubcarrierCount;
pub use node::{NodeId, NodeKind};