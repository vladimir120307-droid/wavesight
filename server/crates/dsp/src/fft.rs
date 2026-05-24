//! FFT primitives for CSI amplitude streams.

use num_complex::Complex32;
use rustfft::FftPlanner;

/// Result of an amplitude STFT analysis.
#[derive(Debug, Clone)]
pub struct AmplitudeSpectrum {
    /// Magnitude per bin (length = `signal.len() / 2 + 1`, real input).
    pub bins: Vec<f32>,
    /// Sample rate the FFT was computed at.
    pub sample_rate_hz: f32,
}

impl AmplitudeSpectrum {
    /// Return the dominant bin and its frequency in Hz.
    #[must_use]
    pub fn peak(&self) -> (usize, f32) {
        let (idx, _mag) = self
            .bins
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or((0, &0.0));
        let bin_hz = self.sample_rate_hz / (self.bins.len() as f32 * 2.0 - 2.0);
        (idx, idx as f32 * bin_hz)
    }
}

/// Compute the amplitude spectrum of a real signal.
///
/// `signal` is zero-padded or truncated to the nearest power of two; this
/// is appropriate for the short windows (typically 32–256 samples) we use.
#[must_use]
pub fn ampl_spectrum(signal: &[f32], sample_rate_hz: f32) -> AmplitudeSpectrum {
    let n = signal.len().next_power_of_two().max(2);
    let mut buf: Vec<Complex32> = signal
        .iter()
        .map(|x| Complex32::new(*x, 0.0))
        .chain(std::iter::repeat(Complex32::new(0.0, 0.0)))
        .take(n)
        .collect();

    let mut planner = FftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(n);
    fft.process(&mut buf);

    let bins = buf[..n / 2 + 1].iter().map(|c| c.norm()).collect();
    AmplitudeSpectrum {
        bins,
        sample_rate_hz,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn detects_pure_tone() {
        let fs = 100.0_f32;
        let tone_hz = 12.0_f32;
        let samples: Vec<f32> = (0..256)
            .map(|i| (2.0 * PI * tone_hz * i as f32 / fs).sin())
            .collect();
        let spec = ampl_spectrum(&samples, fs);
        let (_idx, hz) = spec.peak();
        assert!((hz - tone_hz).abs() < 1.0, "peak {hz} far from {tone_hz}");
    }
}
