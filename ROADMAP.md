# WaveSight Roadmap

> _Last updated: 2026-05-24 В· See also [README.md](README.md) and [docs/adr/](docs/adr/)._

The roadmap is intentionally long. We are building a serious open-source platform, not a tech demo. Each milestone (Mx) ends with a tagged release, a video demonstration and a reproducible benchmark run.

## Status legend

- вђ planned
- в—ђ in progress
- в‘ shipped (tagged release)

---

## M0 вЂ” Foundation в—ђ

_Goal: anyone can read the repo, understand what we are building, and run `cargo check` clean._

- в‘ Monorepo layout (firmware / server / dashboard / mobile / training / eval / integrations / examples / docs)
- в‘ Bilingual README (EN + RU) with honest comparison vs RuView
- в‘ MIT license, CONTRIBUTING, CODE_OF_CONDUCT, SECURITY
- в‘ ADR-001..010 вЂ” foundational decisions
- в‘ Cargo workspace skeleton
- в‘ GitHub Actions CI (build, lint, test, security audit)
- в‘ Issue / PR templates, editorconfig, pre-commit
- вђ Discord server
- вђ Logo + favicon
- вђ docs.wavesight.dev landing page

## M1 вЂ” First photon вђ

_Goal: one ESP32-S3 streams real CSI to the Rust server and the dashboard shows a moving line when you walk past._

- вђ ESP32-S3 firmware (`firmware/esp32-csi-node`): WiFi join, CSI capture, NTP time-sync, WebSocket upload
- вђ `server/crates/csi-ingest`: WebSocket + gRPC ingestion endpoints
- вђ `server/crates/dsp`: FFT, CFR filtering, per-subcarrier amplitude/phase extraction
- вђ `server/crates/storage`: SQLite + Parquet timeseries
- вђ Minimal React dashboard: live spectrogram + binary presence indicator
- вђ End-to-end test on real hardware
- вђ YouTube demo video: walk in front of a single node
- вђ Tagged release `v0.1.0`

## M2 вЂ” Mesh & fusion вђ

_Goal: 3+ nodes form a self-healing ESP-MESH, share PTP-aligned CSI, and the server fuses them into a single coherent stream._

- вђ ESP-MESH topology with automatic root election
- вђ IEEE 1588 PTP time-sync, target в‰¤ 1 Ојs error across the mesh
- вђ `server/crates/fusion`: multi-node Kalman filter
- вђ Time-aligned ring buffer per node
- вђ Background subtraction / clutter map
- вђ 2D presence heatmap in dashboard
- вђ `eval/`: first reproducible benchmark вЂ” presence precision/recall in two rooms
- вђ Release `v0.2.0`

## M3 вЂ” Vitals вђ

_Goal: detect heart rate and breathing within В±2 BPM of a Polar H10 reference._

- вђ Doppler estimation in `dsp`
- вђ Vitals model in `inference` (1D CNN, Candle)
- вђ Reference dataset capture (10+ subjects, Polar H10 ground truth)
- вђ Dashboard vitals chart with confidence bands
- вђ Honest report: per-distance MAE, per-subject scatter
- вђ Release `v0.3.0`

## M4 вЂ” Pose & fall вђ

_Goal: 17-keypoint pose at в‰Ґ 25 % PCK@20 with camera ground-truth. EldGuard MVP catches 90 % of staged falls with в‰¤ 1 false positive/day._

- вђ Pose model (transformer head over fused CSI embedding)
- вђ Fall detection model (specialized, latency-optimised)
- вђ `examples/eldguard`: ready-to-deploy vertical app
- вђ Mobile push notifications via Flutter app
- вђ Family-sharing access model
- вђ Release `v0.4.0`

## M5 вЂ” Sleep & smart-home вђ

_Goal: SleepWave produces a 4-stage sleep hypnogram comparable to Withings Sleep. PresenceOS drives Home Assistant heating/lighting based on per-room occupancy._

- вђ Sleep staging model (Wake / Light / Deep / REM)
- вђ `examples/sleepwave`: nightly report PDF + dashboard panel
- вђ Per-room occupancy with sub-second latency
- вђ Home Assistant integration via HACS (`integrations/home-assistant/wavesight`)
- вђ Matter bridge prototype
- вђ Release `v0.5.0`

## M6 вЂ” UWB & 3D вђ

_Goal: DWM3000 anchors provide true angle-of-arrival; the dashboard renders a Three.js room with live human figures._

- вђ DWM3000 firmware (TWR + AoA)
- вђ UWB fusion into `fusion` crate
- вђ Three.js dashboard with floor-plan upload + live figures
- вђ VR preview (WebXR)
- вђ Release `v0.6.0`

## M7 вЂ” Honest 1.0 вђ

_Goal: tagged 1.0 with a published benchmark paper-style report and a public dataset on HuggingFace._

- вђ 50+ hour public dataset (consent-cleared)
- вђ Benchmark CI gating every release
- вђ `wavesight bench` CLI subcommand
- вђ Reference report PDF in `eval/REPORT_v1.0.pdf`
- вђ HackerNews + Habr launch posts
- вђ Release `v1.0.0` рџЋ‰

---

## Beyond 1.0

- mmWave radar fusion (XeThru / Texas Instruments IWR6843)
- Bluetooth LE RSSI / channel sounding fusion
- Local LLM voice assistant overlay (Whisper + Llama 3.1)
- Differential-privacy federated learning across consenting deployments
- Plugin system (Wasm) for community-built models
- Certification path: CE marking + FDA Class II (EldGuard medical claim)
