//! CSI ingestion layer.
//!
//! Accepts authenticated CSI frame streams from sensing nodes over WebSocket
//! and gRPC, validates and decrypts them, and forwards them to the DSP layer
//! via per-node Tokio channels.

#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

mod handler;
mod hub;
mod wire;

pub use handler::{router, IngestConfig};
pub use hub::{IngestHub, NodeHandle};
pub use wire::{decode_iq, parse_batch, BatchMessage, FrameItem};

use tokio::sync::mpsc;
use wavesight_core::CsiFrame;

/// Per-node CSI frame channel sender.
pub type CsiSender = mpsc::Sender<CsiFrame>;

/// Per-node CSI frame channel receiver.
pub type CsiReceiver = mpsc::Receiver<CsiFrame>;
