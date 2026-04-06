//! Tracked entities in the fused scene.

use serde::{Deserialize, Serialize};

use crate::confidence::Uncertainty;

/// Opaque, stable identifier for a tracked entity within a single session.
///
/// IDs are not stable across server restarts. A re-identification layer is
/// out of scope for the alpha; see roadmap M5+.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EntityId(pub u32);

/// Coarse class label assigned by the fusion / inference layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntityClass {
    /// Human, ambulatory or stationary.
    Human,
    /// Pet (cat / dog / similar).
    Pet,
    /// Unclassified motion source.
    Unknown,
}

/// One tracked entity in a [`crate::FusedScene`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Entity {
    /// Session-stable identifier.
    pub id: EntityId,
    /// Coarse class.
    pub class: EntityClass,
    /// Position in metres in the room frame `(x, y, z)`.
    pub position: [f32; 3],
    /// Velocity in metres / second.
    pub velocity: [f32; 3],
    /// Epistemic uncertainty for this entity's existence + state.
    pub uncertainty: Uncertainty,
}
