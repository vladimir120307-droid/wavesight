//! Frame types — raw CSI and post-DSP representations.

use serde::{Deserialize, Serialize};

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