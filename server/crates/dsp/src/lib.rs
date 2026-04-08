//! Digital signal processing for CSI streams.

#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

/// DSP pipeline configuration.
#[derive(Debug, Clone)]
pub struct DspConfig {
    /// Window size in frames for STFT analysis.
    pub window: usize,
    /// Low-pass cutoff in Hz applied to amplitude streams.
    pub lowpass_hz: f32,
    /// Whether to subtract the calibrated clutter map.
    pub subtract_clutter: bool,
}

impl Default for DspConfig {
    fn default() -> Self {
        Self {
            window: 64,
            lowpass_hz: 30.0,
            subtract_clutter: true,
        }
    }
}