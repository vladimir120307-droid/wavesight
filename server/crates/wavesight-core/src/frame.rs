//! Frame types — raw CSI and post-DSP representations.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::node::NodeId;

/// Number of subcarriers in a CSI capture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubcarrierCount {
    /// 64 subcarriers (HT20).
    Ht20,
    /// 128 subcarriers (HT40 narrow).
    Ht40Narrow,
    /// 256 subcarriers (HT40 wide).
    Ht40Wide,
}

impl SubcarrierCount {
    /// Integer subcarrier count.
    #[must_use]
    pub fn as_usize(self) -> usize {
        match self {
            Self::Ht20 => 64,
            Self::Ht40Narrow => 128,
            Self::Ht40Wide => 256,
        }
    }
}

/// Per-frame metadata shared between raw and DSP frames.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrameMetadata {
    /// Originating node.
    pub node: NodeId,
    /// PTP-synchronised capture timestamp.
    pub captured_at: DateTime<Utc>,
    /// Sequence number monotonically increasing per node.
    pub sequence: u64,
    /// WiFi channel and bandwidth descriptor.
    pub channel: u8,
    /// Subcarrier layout for this frame.
    pub subcarriers: SubcarrierCount,
    /// Received signal strength in dBm.
    pub rssi_dbm: i8,
}

/// Raw CSI frame as received from a node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsiFrame {
    /// Metadata header.
    pub metadata: FrameMetadata,
    /// Complex amplitude per subcarrier (interleaved I,Q as i8 pairs).
    pub samples: Vec<i8>,
}