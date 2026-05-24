//! Single-pole low-pass filter.
//!
//! Used to denoise per-subcarrier amplitude streams before higher-level
//! processing. The constant time is derived from the cutoff frequency via
//! `α = 1 - exp(-2π fc / fs)`.

/// Streaming single-pole IIR low-pass filter.
#[derive(Debug, Clone)]
pub struct LowPass {
    alpha: f32,
    state: f32,
    initialised: bool,
}

impl LowPass {
    /// Construct a filter with the given cutoff and sample rate.
    #[must_use]
    pub fn new(cutoff_hz: f32, sample_rate_hz: f32) -> Self {
        let alpha = 1.0 - (-2.0 * std::f32::consts::PI * cutoff_hz / sample_rate_hz).exp();
        Self {
            alpha,
            state: 0.0,
            initialised: false,
        }
    }

    /// Reset the filter state.
    pub fn reset(&mut self) {
        self.state = 0.0;
        self.initialised = false;
    }

    /// Push a new sample and return the filtered output.
    pub fn step(&mut self, x: f32) -> f32 {
        if !self.initialised {
            self.state = x;
            self.initialised = true;
        } else {
            self.state += self.alpha * (x - self.state);
        }
        self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dc_passes_unchanged() {
        let mut lp = LowPass::new(10.0, 100.0);
        for _ in 0..100 {
            let y = lp.step(5.0);
            assert!((y - 5.0).abs() < 1e-3, "expected steady-state 5.0, got {y}");
        }
    }

    #[test]
    fn high_frequency_is_attenuated() {
        let fs = 100.0;
        let mut lp = LowPass::new(1.0, fs);
        let mut max_y: f32 = 0.0;
        for i in 0..1000 {
            // 25 Hz signal — well above 1 Hz cutoff.
            let x = (2.0 * std::f32::consts::PI * 25.0 * i as f32 / fs).sin();
            let y = lp.step(x);
            if i > 200 {
                max_y = max_y.max(y.abs());
            }
        }
        assert!(max_y < 0.2, "expected strong attenuation, got max {max_y}");
    }
}
