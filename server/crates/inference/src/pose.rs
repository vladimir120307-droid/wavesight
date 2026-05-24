//! Pose head — 17-keypoint body pose from CSI.
//!
//! Target accuracy: PCK@20 ≥ 25 % at M4 (see ROADMAP). Until then this head
//! emits a structurally valid response with very high uncertainty so the
//! dashboard can render the scaffolding without lying about content.

use serde::{Deserialize, Serialize};
use wavesight_core::{FusedScene, Result, Uncertainty};

use crate::Head;

/// One 2-D keypoint with its visibility.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Keypoint {
    /// Horizontal position in metres, room frame.
    pub x: f32,
    /// Vertical position in metres, room frame.
    pub y: f32,
    /// Visibility in `[0, 1]`. Below 0.3 the dashboard hides the point.
    pub visibility: f32,
}

/// Output of the pose head.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PosePrediction {
    /// 17 keypoints in COCO order.
    pub keypoints: [Keypoint; 17],
    /// Epistemic uncertainty of the whole pose.
    pub uncertainty: Uncertainty,
}

/// Stub pose head — placeholder until M4.
pub struct PoseHead;

impl Head for PoseHead {
    type Output = PosePrediction;

    fn predict(&mut self, _scene: &FusedScene) -> Result<Self::Output> {
        let invisible = Keypoint {
            x: 0.0,
            y: 0.0,
            visibility: 0.0,
        };
        Ok(PosePrediction {
            keypoints: [invisible; 17],
            uncertainty: Uncertainty(1.0),
        })
    }
}
