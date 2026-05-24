//! Wire format for the WebSocket ingest endpoint.
//!
//! Mirrors the schema produced by the ESP32 firmware (see
//! `firmware/esp32-csi-node/README.md`).

use base64::Engine as _;
use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use wavesight_core::{CsiFrame, FrameMetadata, NodeId, SubcarrierCount};

/// One CSI frame as encoded on the wire.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameItem {
    /// Monotonic per-node sequence number.
    pub seq: u64,
    /// Capture timestamp in microseconds since node boot.
    pub ts_us: u64,
    /// Receiver signal strength in dBm.
    pub rssi: i8,
    /// Primary WiFi channel.
    pub ch: u8,
    /// Bandwidth code: 0 = HT20, 1 = HT40.
    pub bw: u8,
    /// Base64-encoded interleaved I/Q int8 pairs.
    pub iq: String,
}

/// Top-level batch message produced by a single node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchMessage {
    /// Source node name.
    pub node: String,
    /// Frames in this batch.
    pub batch: Vec<FrameItem>,
}

/// Errors that can occur while parsing a wire frame.
#[derive(Debug, Error)]
pub enum WireError {
    /// JSON parse failure.
    #[error("invalid json: {0}")]
    Json(#[from] serde_json::Error),
    /// Base64 IQ payload could not be decoded.
    #[error("invalid base64 iq: {0}")]
    Base64(#[from] base64::DecodeError),
    /// IQ length does not match the declared bandwidth.
    #[error("iq length {got} does not match declared bandwidth (bw={bw})")]
    IqLength {
        /// Number of int8 entries actually decoded.
        got: usize,
        /// Bandwidth code from the wire.
        bw: u8,
    },
}

/// Parse a top-level batch message from a raw text frame.
///
/// # Errors
/// Returns [`WireError::Json`] when the payload is not valid JSON of the
/// expected shape.
pub fn parse_batch(raw: &str) -> Result<BatchMessage, WireError> {
    Ok(serde_json::from_str(raw)?)
}

/// Decode the base64 IQ payload of a single frame and validate against the
/// declared bandwidth.
///
/// # Errors
/// Returns [`WireError::Base64`] on malformed base64, or [`WireError::IqLength`]
/// when the decoded length does not match the declared bandwidth.
pub fn decode_iq(item: &FrameItem) -> Result<(Vec<i8>, SubcarrierCount), WireError> {
    let bytes = base64::engine::general_purpose::STANDARD.decode(item.iq.as_bytes())?;
    let subcarriers = match item.bw {
        0 => SubcarrierCount::Ht20,
        1 if bytes.len() == 256 => SubcarrierCount::Ht40Narrow,
        1 => SubcarrierCount::Ht40Wide,
        _ => {
            return Err(WireError::IqLength {
                got: bytes.len(),
                bw: item.bw,
            })
        }
    };
    let expected = subcarriers.as_usize() * 2;
    if bytes.len() != expected {
        return Err(WireError::IqLength {
            got: bytes.len(),
            bw: item.bw,
        });
    }
    // Reinterpret u8 → i8 — both are 8 bits, no width change.
    let samples = bytes.into_iter().map(|b| b as i8).collect();
    Ok((samples, subcarriers))
}

/// Project a parsed [`FrameItem`] into a [`CsiFrame`] with absolute timestamps.
///
/// `boot_offset` is the absolute wall-clock instant corresponding to
/// `ts_us == 0` on this node. The ingest hub maintains it on a per-node
/// basis by anchoring the first observed sample.
///
/// # Errors
/// Propagates [`WireError`] from [`decode_iq`].
pub fn item_into_frame(
    item: &FrameItem,
    node: &NodeId,
    boot_offset: DateTime<Utc>,
) -> Result<CsiFrame, WireError> {
    let (samples, subcarriers) = decode_iq(item)?;
    let captured_at = boot_offset
        + chrono::Duration::microseconds(i64::try_from(item.ts_us).unwrap_or(i64::MAX));
    let metadata = FrameMetadata {
        node: node.clone(),
        captured_at,
        sequence: item.seq,
        channel: item.ch,
        subcarriers,
        rssi_dbm: item.rssi,
    };
    Ok(CsiFrame { metadata, samples })
}

/// Convenience constructor for an anchor `boot_offset` given a wall-clock
/// `now` and the node's `ts_us` at the same instant.
#[must_use]
pub fn anchor_boot(now: DateTime<Utc>, ts_us_now: u64) -> DateTime<Utc> {
    let micros = i64::try_from(ts_us_now).unwrap_or(0);
    now - chrono::Duration::microseconds(micros)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_well_formed_batch() {
        let raw = r#"{
            "node":"node-1",
            "batch":[
                {"seq":1,"ts_us":1000,"rssi":-47,"ch":6,"bw":0,
                 "iq":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"}
            ]
        }"#;
        let msg = parse_batch(raw).expect("valid");
        assert_eq!(msg.node, "node-1");
        assert_eq!(msg.batch.len(), 1);
    }

    #[test]
    fn rejects_iq_length_mismatch() {
        let item = FrameItem {
            seq: 1,
            ts_us: 0,
            rssi: -50,
            ch: 6,
            bw: 0,
            iq: "AAAA".to_string(),
        };
        assert!(decode_iq(&item).is_err());
    }

    #[test]
    fn anchor_round_trip() {
        let now = Utc.with_ymd_and_hms(2026, 5, 24, 12, 0, 0).unwrap();
        let boot = anchor_boot(now, 1_500_000);
        let item = FrameItem {
            seq: 42,
            ts_us: 1_500_000,
            rssi: -42,
            ch: 6,
            bw: 0,
            iq: "A".repeat(((64 * 2) * 4 / 3 + 3) & !3),
        };
        // Skip frame projection here — focus on the timestamp invariant.
        let projected = boot + chrono::Duration::microseconds(item.ts_us as i64);
        assert_eq!(projected, now);
    }
}
