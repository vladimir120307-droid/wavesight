//! Digital signal processing for CSI streams.
//!
//! Consumes [`wavesight_core::CsiFrame`] and produces
//! [`wavesight_core::DspFrame`] via calibration, filtering, FFT and Doppler
//! estimation.

#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

mod fft;
mod filter;
mod presence;

pub use fft::{ampl_spectrum, AmplitudeSpectrum};
pub use filter::LowPass;
pub use presence::{PresenceDetector, PresenceState};

use wavesight_core::{CsiFrame, DspFrame, Result, SubcarrierCount};

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

/// Convert a raw CSI frame into a DSP frame.
///
/// # Errors
/// Returns an error if the frame is malformed (wrong sample count for the
/// declared subcarrier layout).
pub fn process(frame: &CsiFrame, _config: &DspConfig) -> Result<DspFrame> {
    let expected = match frame.metadata.subcarriers {
        SubcarrierCount::Ht20 => 64,
        SubcarrierCount::Ht40Narrow => 128,
        SubcarrierCount::Ht40Wide => 256,
    } * 2;
    if frame.samples.len() != expected {
        return Err(wavesight_core::Error::MalformedFrame(format!(
            "expected {expected} samples, got {}",
            frame.samples.len()
        )));
    }

    let n_sub = expected / 2;
    let amplitude = (0..n_sub)
        .map(|i| {
            let re = f32::from(frame.samples[i * 2]);
            let im = f32::from(frame.samples[i * 2 + 1]);
            (re * re + im * im).sqrt() / 127.0
        })
        .collect();

    let phase = (0..n_sub)
        .map(|i| {
            let re = f32::from(frame.samples[i * 2]);
            let im = f32::from(frame.samples[i * 2 + 1]);
            im.atan2(re)
        })
        .collect();

    let doppler = vec![0.0; n_sub];

    Ok(DspFrame {
        metadata: frame.metadata.clone(),
        amplitude,
        phase,
        doppler,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use wavesight_core::{FrameMetadata, NodeId};

    fn make_frame(n: usize) -> CsiFrame {
        CsiFrame {
            metadata: FrameMetadata {
                node: NodeId::new("test"),
                captured_at: Utc::now(),
                sequence: 0,
                channel: 6,
                subcarriers: SubcarrierCount::Ht20,
                rssi_dbm: -50,
            },
            samples: vec![1i8; n],
        }
    }

    #[test]
    fn rejects_wrong_size() {
        let bad = make_frame(10);
        assert!(process(&bad, &DspConfig::default()).is_err());
    }

    #[test]
    fn processes_valid_ht20() {
        let good = make_frame(128);
        let out = process(&good, &DspConfig::default()).expect("should succeed");
        assert_eq!(out.amplitude.len(), 64);
        assert_eq!(out.phase.len(), 64);
    }
}
