# Changelog

All notable changes to WaveSight are documented in this file. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html) once we
reach `v1.0.0`. Pre-1.0 releases may break compatibility between minor versions.

## [Unreleased]

### Added

- Initial monorepo layout (firmware / server / dashboard / mobile / training /
  eval / integrations / examples / docs).
- Bilingual EN + RU README, getting-started, and architecture documents.
- 10 foundational ADRs covering language choice, monorepo layout, honest mode,
  multi-modal fusion, privacy, mesh topology, PTP sync, ML stack and dashboard
  stack.
- Rust workspace skeleton with seven crates: `wavesight-core`, `csi-ingest`,
  `dsp`, `fusion`, `inference`, `storage`, `api`, `cli`.
- GitHub Actions: rust, web, firmware, python, security audit, benchmark.
- MIT licence, Contributor Covenant CoC, threat-modelled SECURITY.md,
  CONTRIBUTING (EN + RU).
- Issue & PR templates, .gitignore, .editorconfig, pre-commit hooks.
- ROADMAP with 7 milestones up to v1.0.
- COMPARISON.md detailing honest technical differentiation vs RuView.

### Targeted next (M1 — First photon)

- ESP32-S3 firmware that streams CSI frames over WebSocket.
- DSP pipeline turning raw CSI into amplitude / phase / Doppler frames.
- Minimal React dashboard with live presence indicator.
- First reproducible benchmark (single-node presence).
