//! Timeseries storage for WaveSight.
//!
//! Two backends:
//! - **SQLite** for the rolling 24-hour raw CSI buffer.
//! - **Parquet** files for indefinite-retention aggregated derivatives.

#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

mod schema;
mod writer;

pub use schema::{migrate, SCHEMA_VERSION};
pub use writer::FrameWriter;

use std::path::PathBuf;

use rusqlite::Connection;
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
    conn: Connection,
    config: StorageConfig,
}

impl Storage {
    /// Open or create the storage layer.
    ///
    /// # Errors
    /// Returns an error if directories or the SQLite file cannot be created
    /// or if schema migration fails.
    pub fn open(config: StorageConfig) -> Result<Self> {
        if let Some(parent) = config.sqlite_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::create_dir_all(&config.parquet_dir)?;

        let conn = Connection::open(&config.sqlite_path)
            .map_err(|e| wavesight_core::Error::Config(format!("sqlite: {e}")))?;
        migrate(&conn).map_err(|e| wavesight_core::Error::Config(format!("migrate: {e}")))?;

        Ok(Self { conn, config })
    }

    /// Construct a writer bound to this storage handle.
    #[must_use]
    pub fn writer(&self) -> FrameWriter<'_> {
        FrameWriter::new(&self.conn)
    }

    /// Storage config in effect.
    #[must_use]
    pub fn config(&self) -> &StorageConfig {
        &self.config
    }

    /// Prune raw CSI rows older than the configured retention window.
    ///
    /// # Errors
    /// Returns an error if the DELETE statement fails.
    pub fn prune_expired(&self) -> Result<usize> {
        let cutoff = chrono::Utc::now()
            - chrono::Duration::hours(i64::from(self.config.raw_retention_hours));
        let n = self
            .conn
            .execute(
                "DELETE FROM csi_frame WHERE captured_at < ?1",
                rusqlite::params![cutoff.to_rfc3339()],
            )
            .map_err(|e| wavesight_core::Error::Config(format!("prune: {e}")))?;
        Ok(n)
    }
}
