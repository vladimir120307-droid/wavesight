//! Multi-modal fusion layer.

#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

/// Fusion engine configuration.
#[derive(Debug, Clone)]
pub struct FusionConfig {
    /// Process-noise standard deviation in metres / second^2 for human motion.
    pub human_process_noise: f32,
    /// Observation-noise variance for WiFi-CSI energy direction.
    pub wifi_obs_variance: f32,
    /// Observation-noise variance for UWB AoA.
    pub uwb_obs_variance: f32,
}

impl Default for FusionConfig {
    fn default() -> Self {
        Self {
            human_process_noise: 0.5,
            wifi_obs_variance: 0.2,
            uwb_obs_variance: 0.05,
        }
    }
}