//! Model registry — maps `(name, version)` to a `safetensors` file on disk.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// Reference to a versioned model on disk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelRef {
    /// Model name (e.g. `presence`, `vitals`).
    pub name: String,
    /// Semantic version of the published weights.
    pub version: String,
    /// Path to the `safetensors` file.
    pub path: PathBuf,
    /// SHA-256 of the file for integrity checks.
    pub sha256: String,
}

/// In-memory registry. Populated from a TOML manifest, kept simple on purpose.
#[derive(Debug, Default)]
pub struct ModelRegistry {
    entries: HashMap<String, ModelRef>,
}

impl ModelRegistry {
    /// Construct an empty registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a model under its `(name, version)` key.
    pub fn register(&mut self, model: ModelRef) {
        let key = format!("{}@{}", model.name, model.version);
        self.entries.insert(key, model);
    }

    /// Look up a model by name and version.
    #[must_use]
    pub fn lookup(&self, name: &str, version: &str) -> Option<&ModelRef> {
        let key = format!("{name}@{version}");
        self.entries.get(&key)
    }

    /// Iterate all registered models.
    pub fn iter(&self) -> impl Iterator<Item = &ModelRef> {
        self.entries.values()
    }

    /// Load a manifest file (TOML) from the given path.
    ///
    /// # Errors
    /// Propagates `std::io::Error` from the file read; returns a generic
    /// error wrapped in [`wavesight_core::Error::Config`] for parse failures.
    pub fn load_manifest(&mut self, _path: &Path) -> wavesight_core::Result<()> {
        // Stub — actual TOML parsing lands when we publish the first model.
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookup_returns_registered_model() {
        let mut reg = ModelRegistry::new();
        reg.register(ModelRef {
            name: "presence".to_string(),
            version: "1.0.0".to_string(),
            path: PathBuf::from("models/presence-v1.safetensors"),
            sha256: "deadbeef".to_string(),
        });
        assert!(reg.lookup("presence", "1.0.0").is_some());
        assert!(reg.lookup("presence", "1.0.1").is_none());
    }
}
