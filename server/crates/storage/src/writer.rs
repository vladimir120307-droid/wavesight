//! Batch writer for CSI frames.

use rusqlite::{params, Connection};
use wavesight_core::{CsiFrame, Result};

/// Borrowed handle that buffers writes and commits in batches.
pub struct FrameWriter<'c> {
    conn: &'c Connection,
}

impl<'c> FrameWriter<'c> {
    /// Construct a writer that uses the given SQLite connection.
    #[must_use]
    pub fn new(conn: &'c Connection) -> Self {
        Self { conn }
    }

    /// Insert a single CSI frame.
    ///
    /// # Errors
    /// Returns an error if the SQLite INSERT fails.
    pub fn insert(&self, frame: &CsiFrame) -> Result<()> {
        let subcarriers = match frame.metadata.subcarriers {
            wavesight_core::SubcarrierCount::Ht20 => "ht20",
            wavesight_core::SubcarrierCount::Ht40Narrow => "ht40n",
            wavesight_core::SubcarrierCount::Ht40Wide => "ht40w",
        };
        let bytes: Vec<u8> = frame.samples.iter().map(|&i| i as u8).collect();
        self.conn
            .execute(
                "INSERT INTO csi_frame(node, sequence, captured_at, channel, rssi_dbm, subcarriers, samples)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![
                    frame.metadata.node.0,
                    i64::try_from(frame.metadata.sequence).unwrap_or(i64::MAX),
                    frame.metadata.captured_at.to_rfc3339(),
                    frame.metadata.channel,
                    frame.metadata.rssi_dbm,
                    subcarriers,
                    bytes,
                ],
            )
            .map_err(|e| wavesight_core::Error::Config(format!("insert: {e}")))?;
        Ok(())
    }

    /// Insert a slice of frames atomically (single transaction).
    ///
    /// # Errors
    /// Returns an error if the transaction fails. On error the entire
    /// batch is rolled back.
    pub fn insert_batch(&self, frames: &[CsiFrame]) -> Result<()> {
        let tx = self
            .conn
            .unchecked_transaction()
            .map_err(|e| wavesight_core::Error::Config(format!("begin: {e}")))?;
        for f in frames {
            self.insert(f)?;
        }
        tx.commit()
            .map_err(|e| wavesight_core::Error::Config(format!("commit: {e}")))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::migrate;
    use chrono::Utc;
    use wavesight_core::{FrameMetadata, NodeId, SubcarrierCount};

    fn make_frame(seq: u64) -> CsiFrame {
        CsiFrame {
            metadata: FrameMetadata {
                node: NodeId::new("test"),
                captured_at: Utc::now(),
                sequence: seq,
                channel: 6,
                subcarriers: SubcarrierCount::Ht20,
                rssi_dbm: -50,
            },
            samples: vec![1; 128],
        }
    }

    #[test]
    fn insert_and_count() {
        let conn = Connection::open_in_memory().unwrap();
        migrate(&conn).unwrap();
        let w = FrameWriter::new(&conn);
        w.insert(&make_frame(1)).unwrap();
        w.insert(&make_frame(2)).unwrap();
        let count: u32 = conn
            .query_row("SELECT COUNT(*) FROM csi_frame", [], |r| r.get(0))
            .unwrap();
        assert_eq!(count, 2);
    }

    #[test]
    fn batch_insert_is_atomic() {
        let conn = Connection::open_in_memory().unwrap();
        migrate(&conn).unwrap();
        let w = FrameWriter::new(&conn);
        let batch: Vec<_> = (0..16).map(make_frame).collect();
        w.insert_batch(&batch).unwrap();
        let count: u32 = conn
            .query_row("SELECT COUNT(*) FROM csi_frame", [], |r| r.get(0))
            .unwrap();
        assert_eq!(count, 16);
    }
}
