//! Top-level error type for the server workspace.

use thiserror::Error;

/// Convenient alias for crate-local fallible operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Top-level error variants surfaced by WaveSight crates.
#[derive(Debug, Error)]
pub enum Error {
    /// A serialization or deserialization failed.
    #[error("serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    /// An I/O operation failed.
    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),

    /// A frame was malformed or violated the wire contract.
    #[error("malformed frame: {0}")]
    MalformedFrame(String),

    /// The mesh PSK could not decrypt an incoming frame.
    #[error("decryption failed")]
    DecryptionFailed,

    /// Time synchronization is too loose for the requested operation.
    #[error("time-sync error: rms {rms_us:.1} μs exceeds threshold")]
    TimeSync {
        /// Observed RMS jitter in microseconds.
        rms_us: f32,
    },

    /// A model inference call failed.
    #[error("inference error: {0}")]
    Inference(String),

    /// A configuration value was invalid.
    #[error("configuration error: {0}")]
    Config(String),
}
