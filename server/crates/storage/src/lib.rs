//! Timeseries storage for WaveSight.
//!
//! Two backends:
//! - **SQLite** for the rolling 24-hour raw CSI buffer.
//! - **Parquet** files for indefinite-retention aggregated derivatives.

#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

use std::path::PathBuf;

use wavesight_core::Result;

/// Storage layer configuration.
#[derive(Debug, Clone)]
pub struct StorageConfig {
    /// Path to the SQLite database file.
    pub sqlite_path: PathBuf,
    /// Directory for Parquet aggregates.
    pub parquet_dir: PathBuf,
    /// Raw CSI retention in hours.
    pub raw_retention_hours: u32,
}

impl Default for StorageConfig {
    fn default() -> Self {
        let base = std::env::var_os("HOME")
            .or_else(|| std::env::var_os("USERPROFILE"))
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".wavesight")
            .join("data");
        Self {
            sqlite_path: base.join("wavesight.db"),
            parquet_dir: base.join("parquet"),
            raw_retention_hours: 24,
        }
    }
}

/// Stateful storage handle.
pub struct Storage {
    _config: StorageConfig,
}

impl Storage {
    /// Open or create the storage layer.
    ///
    /// # Errors
    /// Returns an error if directories cannot be created.
    pub fn open(config: StorageConfig) -> Result<Self> {
        std::fs::create_dir_all(&config.parquet_dir)?;
        Ok(Self { _config: config })
    }
}
