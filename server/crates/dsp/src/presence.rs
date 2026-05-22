//! Rule-based presence detector.
//!
//! The M1 detector is intentionally simple: a moving variance of the mean
//! per-frame amplitude crosses a threshold when something disturbs the
//! channel. This is *not* the production presence model — that lives in
//! the `inference` crate and is trained on labelled data — but it gives
//! us a working live indicator from the very first ESP32 frame.

use std::collections::VecDeque;

use wavesight_core::{DspFrame, Uncertainty};

/// Binary presence state with explicit uncertainty (ADR-004).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PresenceState {
    /// `true` when the detector believes a human is in the field.
    pub present: bool,
    /// Energy proxy that drove the decision.
    pub energy: f32,
    /// Calibrated uncertainty.
    pub uncertainty: Uncertainty,
}

/// Streaming presence detector.
#[derive(Debug, Clone)]
pub struct PresenceDetector {
    window: VecDeque<f32>,
    capacity: usize,
    threshold: f32,
    last_present: bool,
}

impl PresenceDetector {
    /// Construct a detector with a sliding window of `window_frames` and a
    /// variance threshold for "occupied".
    #[must_use]
    pub fn new(window_frames: usize, threshold: f32) -> Self {
        Self {
            window: VecDeque::with_capacity(window_frames),
            capacity: window_frames,
            threshold,
            last_present: false,
        }
    }

    /// Push a DSP frame and return the updated presence state.
    pub fn step(&mut self, frame: &DspFrame) -> PresenceState {
        let mean = if frame.amplitude.is_empty() {
            0.0
        } else {
            frame.amplitude.iter().sum::<f32>() / frame.amplitude.len() as f32
        };
        if self.window.len() == self.capacity {
            self.window.pop_front();
        }
        self.window.push_back(mean);

        let n = self.window.len();
        let avg = self.window.iter().sum::<f32>() / n as f32;
        let var = self
            .window
            .iter()
            .map(|x| {
                let d = x - avg;
                d * d
            })
            .sum::<f32>()
            / n as f32;
        let energy = var.sqrt();

        // Hysteresis: arming threshold = self.threshold, disarming = ×0.6.
        let present = if self.last_present {
            energy > self.threshold * 0.6
        } else {
            energy > self.threshold
        };
        self.last_present = present;

        // Uncertainty inversely proportional to window fill and "headroom"
        // above the threshold. Capped to honest bounds.
        let fill = (n as f32 / self.capacity as f32).clamp(0.0, 1.0);
        let headroom = (energy / self.threshold).clamp(0.0, 4.0);
        let raw = (1.0 - fill).max(1.0 / (1.0 + headroom));
        let u = Uncertainty(raw.clamp(0.05, 1.0));

        PresenceState {
            present,
            energy,
            uncertainty: u,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use wavesight_core::{FrameMetadata, NodeId, SubcarrierCount};

    fn frame_with_amplitude(values: Vec<f32>) -> DspFrame {
        let n = values.len();
        DspFrame {
            metadata: FrameMetadata {
                node: NodeId::new("t"),
                captured_at: Utc::now(),
                sequence: 0,
                channel: 6,
                subcarriers: SubcarrierCount::Ht20,
                rssi_dbm: -50,
            },
            amplitude: values,
            phase: vec![0.0; n],
            doppler: vec![0.0; n],
        }
    }

    #[test]
    fn empty_room_stays_absent() {
        let mut d = PresenceDetector::new(32, 0.05);
        for _ in 0..40 {
            let s = d.step(&frame_with_amplitude(vec![0.5; 64]));
            assert!(!s.present);
        }
    }

    #[test]
    fn motion_triggers_presence() {
        let mut d = PresenceDetector::new(16, 0.05);
        for i in 0..32 {
            let level = if i < 8 {
                0.5
            } else {
                0.5 + 0.4 * (i as f32).sin()
            };
            d.step(&frame_with_amplitude(vec![level; 64]));
        }
        // After enough motion the detector should fire.
        let last = d.step(&frame_with_amplitude(vec![0.95; 64]));
        assert!(last.present, "expected presence after sinusoidal motion");
    }
}
